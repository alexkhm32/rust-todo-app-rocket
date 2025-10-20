use std::str::FromStr;

use crate::domain::{
    Account as DomainAccount, Status as TodoItemStatus, TodoItem as DomainTodoItem,
};
use sqlx::types::chrono::NaiveDateTime;

#[derive(sqlx::FromRow, sqlx::Decode)]
pub struct Account {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl Into<DomainAccount> for Account {
    fn into(self) -> DomainAccount {
        DomainAccount {
            id: self.id,
            login: self.login,
            password: self.password,
            created_at: time::OffsetDateTime::from_unix_timestamp(
                self.created_at.and_utc().timestamp(),
            )
            .unwrap(),
        }
    }
}

#[derive(sqlx::FromRow, sqlx::Decode)]
pub struct TodoItem {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub status: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryInto<DomainTodoItem> for TodoItem {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<DomainTodoItem, Self::Error> {
        let status = TodoItemStatus::from_str(&self.status)?;
        let created_at =
            time::OffsetDateTime::from_unix_timestamp(self.created_at.and_utc().timestamp())?;
        let updated_at =
            time::OffsetDateTime::from_unix_timestamp(self.updated_at.and_utc().timestamp())?;

        Ok(DomainTodoItem {
            id: self.id,
            owner_id: self.owner_id,
            title: self.title,
            status: status,
            description: self.description,
            created_at: created_at,
            updated_at: updated_at,
        })
    }
}
