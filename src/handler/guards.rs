use std::sync::Arc;

use rocket::{
    Request, State,
    http::Status,
    request::{FromRequest, Outcome},
};

use crate::domain::AccountService;

pub struct AuthGuard {
    pub account_id: i32,
}

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let account_service = match request.guard::<&State<Arc<dyn AccountService>>>().await {
            Outcome::Success(service) => service,
            _ => return Outcome::Error((Status::Unauthorized, AuthError::Invalid)),
        };

        let auth_header = request.headers().get_one("Authorization");
        if auth_header.is_none() {
            return Outcome::Error((Status::Unauthorized, AuthError::Missing));
        }

        let auth_header = auth_header.unwrap();
        if !auth_header.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, AuthError::Invalid));
        }

        let token = &auth_header[7..];

        match account_service.inner().authorize(token.to_string()).await {
            Ok(account_id) => Outcome::Success(AuthGuard { account_id }),
            _ => Outcome::Error((Status::Unauthorized, AuthError::Invalid)),
        }
    }
}
