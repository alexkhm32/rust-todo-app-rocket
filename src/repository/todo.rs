use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    domain::{
        CreateTodoItemRequest, Error, Filters, Status, TodoCounter, TodoCreator, TodoGetter,
        TodoItem, TodoLister, TodoUpdater, UpdateTodoItemRequest,
    },
    repository::models,
};

#[derive(Clone)]
pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool }
    }
}

#[async_trait]
impl TodoCreator for TodoRepository {
    async fn create(&self, request: CreateTodoItemRequest) -> Result<TodoItem, Error> {
        let result = sqlx::query_as::<_, (i32,)>(
            "INSERT INTO todo_items (owner_id, title, status, description) VALUES ($1, $2, $3, $4) RETURNING id",
        )
        .bind(request.owner_id)
        .bind(request.title)
        .bind(Status::Draft.to_string())
        .bind(request.description)
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(row) => self.one(row.0).await,
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }
}

#[async_trait]
impl TodoCounter for TodoRepository {
    async fn count(&self, filters: &Filters) -> Result<i64, Error> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT COUNT(*) FROM todo_items");

        if let Some(status) = &filters.status {
            query.push(" WHERE status = ");
            query.push_bind(status.to_string());
        }

        let result = query.build_query_as::<(i64,)>().fetch_one(&self.pool).await;

        match result {
            Ok(count) => Ok(count.0),
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }
}

#[async_trait]
impl TodoLister for TodoRepository {
    async fn list(&self, filters: &Filters) -> Result<Vec<TodoItem>, Error> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM todo_items");

        if let Some(status) = &filters.status {
            query.push(" WHERE status = ");
            query.push_bind(status.to_string());
        }

        if let Some(limit) = filters.limit {
            query.push(" LIMIT ");
            query.push_bind(limit);
        }

        if let Some(offset) = filters.offset {
            query.push(" OFFSET ");
            query.push_bind(offset);
        }

        let result = query
            .build_query_as::<models::TodoItem>()
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(records) => Ok(records.into_iter().map(|x| x.try_into().unwrap()).collect()),
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }
}

#[async_trait]
impl TodoGetter for TodoRepository {
    async fn one(&self, id: i32) -> Result<TodoItem, Error> {
        let result =
            sqlx::query_as::<_, models::TodoItem>("SELECT * FROM todo_items WHERE id = $1")
                .bind(id)
                .fetch_one(&self.pool)
                .await;
        match result {
            Ok(record) => Ok(record.try_into().unwrap()),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(Error::NotFound(err.to_string())),
                _ => Err(Error::Unknown(err.to_string())),
            },
        }
    }
}

#[async_trait]
impl TodoUpdater for TodoRepository {
    async fn update(&self, request: UpdateTodoItemRequest) -> Result<TodoItem, Error> {
        let result = sqlx::query_as::<_, models::TodoItem>(
            "UPDATE todo_items SET status = $1 WHERE id = $2 RETURNING *",
        )
        .bind(request.status.to_string())
        .bind(request.item_id)
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(item) => Ok(item.try_into().unwrap()),
            Err(err) => Err(Error::Unknown(err.to_string())),
        }
    }
}
