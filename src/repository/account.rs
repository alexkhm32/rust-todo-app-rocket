use sqlx::PgPool;

use crate::{
    domain::{Account, AccountRepository as AccountRepositoryTrait, CreateAccountRequest, Error},
    repository::models,
};

#[derive(Clone)]
pub struct AccountRepository {
    pool: PgPool,
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
impl AccountRepositoryTrait for AccountRepository {
    async fn create(&self, request: CreateAccountRequest) -> Result<Account, Error> {
        let result = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO accounts (login, password) VALUES ($1, $2) RETURNING id",
        )
        .bind(request.login)
        .bind(request.password)
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok((id,)) => self.get(id).await,
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }

    async fn get(&self, id: i32) -> Result<Account, Error> {
        let result = sqlx::query_as::<_, models::Account>("SELECT * FROM accounts WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await;
        match result {
            Ok(account) => Ok(account.into()),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(Error::NotFound(err.to_string())),
                _ => Err(Error::Unknown(err.to_string())),
            },
        }
    }

    async fn get_by_login(&self, login: String) -> Result<Account, Error> {
        let result =
            sqlx::query_as::<_, models::Account>("SELECT * FROM accounts WHERE login = $1")
                .bind(login)
                .fetch_one(&self.pool)
                .await;
        match result {
            Ok(account) => Ok(account.into()),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(Error::NotFound(err.to_string())),
                _ => Err(Error::Unknown(err.to_string())),
            },
        }
    }
}
