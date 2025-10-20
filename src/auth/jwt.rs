use crate::domain::{AuthToken, AuthTokenGenerator as AuthTokenGeneratorTrait, Error};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::Duration;

#[derive(Serialize, Deserialize, Clone)]
struct Claims {
    exp: usize,
    sub: String,
}

pub struct AuthTokenGenerator {
    duration: Duration,
    secret: String,
}

impl AuthTokenGenerator {
    pub fn new(duration: Duration, secret: String) -> Self {
        Self { duration, secret }
    }
}

impl AuthTokenGeneratorTrait for AuthTokenGenerator {
    fn generate(&self, account_id: i32) -> Result<AuthToken, Error> {
        let expires = time::OffsetDateTime::now_utc() + self.duration;
        let claims = Claims {
            exp: expires.unix_timestamp() as usize,
            sub: account_id.to_string(),
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .unwrap();
        Ok(AuthToken {
            token,
            expires: expires,
        })
    }

    fn parse(&self, token: String) -> Result<i32, Error> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .unwrap();
        Ok(data.claims.sub.parse().unwrap())
    }
}
