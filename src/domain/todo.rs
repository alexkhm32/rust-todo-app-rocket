use crate::domain::errors::Error;
use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub enum Status {
    Draft,
    InProgress,
    Completed,
    Rejected,
}

impl Status {
    pub fn can_be_updated_to(&self, next: &Self) -> bool {
        match self {
            Status::Draft => [Status::InProgress, Status::Rejected].contains(next),
            Status::InProgress => [Status::Completed, Status::Rejected].contains(next),
            _ => false,
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Draft => "draft".to_string(),
            Status::InProgress => "in_progress".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Rejected => "rejected".to_string(),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(Status::Draft),
            "in_progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "rejected" => Ok(Status::Rejected),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

pub struct TodoItem {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub status: Status,
    pub description: String,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
}

pub struct Filters {
    pub status: Option<Status>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub struct CreateTodoItemRequest {
    pub owner_id: i32,
    pub title: String,
    pub description: String,
}

pub struct UpdateTodoItemRequest {
    pub owner_id: i32,
    pub item_id: i32,
    pub status: Status,
}

#[async_trait]
pub trait TodoCreator: Send + Sync {
    async fn create(&self, request: CreateTodoItemRequest) -> Result<TodoItem, Error>;
}

#[async_trait]
pub trait TodoCounter: Send + Sync {
    async fn count(&self, filters: &Filters) -> Result<i64, Error>;
}

#[async_trait]
pub trait TodoLister: Send + Sync {
    async fn list(&self, filters: &Filters) -> Result<Vec<TodoItem>, Error>;
}

#[async_trait]
pub trait TodoListerAndCounter: Send + Sync {
    async fn list(&self, filters: &Filters) -> Result<(Vec<TodoItem>, i64), Error>;
}

#[async_trait]
pub trait TodoGetter: Send + Sync {
    async fn one(&self, id: i32) -> Result<TodoItem, Error>;
}

#[async_trait]
pub trait TodoUpdater: Send + Sync {
    async fn update(&self, request: UpdateTodoItemRequest) -> Result<TodoItem, Error>;
}
