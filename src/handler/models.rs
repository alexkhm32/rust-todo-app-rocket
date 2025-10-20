use crate::domain::{
    AuthToken, CreateTodoItemRequest as DomainCreateTodoItemRequest, Error, Filters,
    LoginRequest as DomainLoginRequest, RegisterRequest as DomainRegisterRequest,
    Status as TodoStatus, TodoItem, UpdateTodoItemRequest as DomainUpdateTodoItemRequest,
};
use rocket::serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

pub enum StatusQuery {
    None,
    Status(TodoStatus),
}

#[rocket::async_trait]
impl rocket::form::FromFormField<'_> for StatusQuery {
    fn from_value(field: rocket::form::ValueField<'_>) -> rocket::form::Result<'_, Self> {
        if field.value.is_empty() {
            Ok(StatusQuery::None)
        } else {
            match TodoStatus::from_str(field.value) {
                Ok(status) => Ok(StatusQuery::Status(status)),
                Err(_) => Err(rocket::form::Error::validation("Invalid status value").into()),
            }
        }
    }
}

impl Into<Option<TodoStatus>> for StatusQuery {
    fn into(self) -> Option<TodoStatus> {
        match self {
            StatusQuery::None => None,
            StatusQuery::Status(status) => Some(status),
        }
    }
}

pub struct StatusField(TodoStatus);

impl<'de> Deserialize<'de> for StatusField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match TodoStatus::from_str(&String::deserialize(deserializer)?) {
            Ok(status) => Ok(StatusField(status)),
            Err(err) => Err(rocket::serde::de::Error::custom(err)),
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterRequest {
    pub login: String,
    pub password: String,
}

impl Into<DomainRegisterRequest> for RegisterRequest {
    fn into(self) -> DomainRegisterRequest {
        DomainRegisterRequest {
            login: self.login,
            password: self.password,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginRequest {
    pub login: String,
    pub password: String,
}

impl Into<DomainLoginRequest> for LoginRequest {
    fn into(self) -> DomainLoginRequest {
        DomainLoginRequest {
            login: self.login,
            password: self.password,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

impl From<AuthToken> for AuthTokenResponse {
    fn from(token: AuthToken) -> Self {
        Self {
            access_token: token.token,
            token_type: "bearer".to_string(),
            expires_in: (token.expires - time::OffsetDateTime::now_utc()).whole_seconds() as usize,
        }
    }
}

#[derive(FromForm)]
pub struct GetTodoFilters {
    pub status: StatusQuery,
    #[field(default = Some(10))]
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl Into<Filters> for GetTodoFilters {
    fn into(self) -> Filters {
        Filters {
            status: self.status.into(),
            limit: self.limit,
            offset: self.offset,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTodoItemRequest {
    pub title: String,
    pub description: String,
}

impl CreateTodoItemRequest {
    pub fn into_domain(self, owner_id: i32) -> DomainCreateTodoItemRequest {
        DomainCreateTodoItemRequest {
            owner_id: owner_id,
            title: self.title,
            description: self.description,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTodoItemRequest {
    pub status: StatusField,
}

impl UpdateTodoItemRequest {
    pub fn into_domain(self, item_id: i32, owner_id: i32) -> DomainUpdateTodoItemRequest {
        DomainUpdateTodoItemRequest {
            owner_id,
            item_id,
            status: self.status.0,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<Meta>,
}

impl<T> From<T> for Response<T> {
    fn from(item: T) -> Self {
        Self {
            data: item,
            meta: None,
        }
    }
}

impl<T> From<(T, i64)> for Response<T> {
    fn from((item, total): (T, i64)) -> Self {
        Self {
            data: item,
            meta: Some(Meta { total: total }),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Meta {
    total: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TodoItemData {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub description: String,

    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,

    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
}

impl From<&TodoItem> for TodoItemData {
    fn from(model: &TodoItem) -> Self {
        Self {
            id: model.id,
            title: model.title.clone(),
            status: model.status.to_string(),
            description: model.description.clone(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    error: ErrorData,
}

impl From<&Error> for ErrorResponse {
    fn from(error: &Error) -> Self {
        match error {
            Error::NotFound(_) => Self {
                error: ErrorData {
                    code: "not_found".to_string(),
                },
            },
            Error::OperationNotApplicable(_) => Self {
                error: ErrorData {
                    code: "operation_not_applicable".to_string(),
                },
            },
            _ => Self {
                error: ErrorData {
                    code: "internal_server_error".to_string(),
                },
            },
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorData {
    pub code: String,
}

impl From<&Error> for rocket::http::Status {
    fn from(error: &Error) -> Self {
        match error {
            Error::NotFound(_) => rocket::http::Status::NotFound,
            Error::OperationNotApplicable(_) => rocket::http::Status::BadRequest,
            _ => rocket::http::Status::InternalServerError,
        }
    }
}
