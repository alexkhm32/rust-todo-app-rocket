use std::sync::Arc;

use rocket::{State, http::Status, response::status::Custom, serde::json::Json};

use crate::{domain::AccountService, handler::models};

#[post("/register", data = "<request>")]
pub async fn register(
    request: Json<models::RegisterRequest>,
    service: &State<Arc<dyn AccountService>>,
) -> Custom<Result<Json<models::Response<models::AuthTokenResponse>>, Json<models::ErrorResponse>>>
{
    let result = service.inner().register(request.into_inner().into()).await;
    match result {
        Ok(token) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from(
                models::AuthTokenResponse::from(token),
            ))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}

#[post("/login", data = "<request>")]
pub async fn login(
    request: Json<models::LoginRequest>,
    service: &State<Arc<dyn AccountService>>,
) -> Custom<Result<Json<models::Response<models::AuthTokenResponse>>, Json<models::ErrorResponse>>>
{
    let result = service.inner().login(request.into_inner().into()).await;
    match result {
        Ok(token) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from(
                models::AuthTokenResponse::from(token),
            ))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}
