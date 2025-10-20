use crate::{
    domain::{TodoCreator, TodoGetter, TodoListerAndCounter, TodoUpdater},
    handler::{
        guards::AuthGuard,
        models::{self, UpdateTodoItemRequest},
    },
};
use rocket::{State, http::Status, response::status::Custom, serde::json::Json};
use std::sync::Arc;

#[post("/todo", data = "<request>")]
pub async fn post_todo(
    auth_guard: AuthGuard,
    request: Json<models::CreateTodoItemRequest>,
    creator: &State<Arc<dyn TodoCreator>>,
) -> Custom<Result<Json<models::Response<models::TodoItemData>>, Json<models::ErrorResponse>>> {
    let result = creator
        .inner()
        .create(request.into_inner().into_domain(auth_guard.account_id))
        .await;
    match result {
        Ok(item) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from(models::TodoItemData::from(
                &item,
            )))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}

#[get("/todo?<filters..>")]
pub async fn get_todo(
    #[allow(dead_code, unused_variables)] auth_guard: AuthGuard,
    filters: models::GetTodoFilters,
    lister: &State<Arc<dyn TodoListerAndCounter>>,
) -> Custom<Result<Json<models::Response<Vec<models::TodoItemData>>>, Json<models::ErrorResponse>>>
{
    match lister.inner().list(&filters.into()).await {
        Ok((items, total)) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from((
                items
                    .into_iter()
                    .map(|item| models::TodoItemData::from(&item))
                    .collect(),
                total,
            )))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}

#[get("/todo/<id>")]
pub async fn get_todo_by_id(
    #[allow(dead_code, unused_variables)] auth_guard: AuthGuard,
    id: i32,
    getter: &State<Arc<dyn TodoGetter>>,
) -> Custom<Result<Json<models::Response<models::TodoItemData>>, Json<models::ErrorResponse>>> {
    match getter.inner().one(id).await {
        Ok(item) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from(models::TodoItemData::from(
                &item,
            )))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}

#[patch("/todo/<id>", data = "<request>")]
pub async fn patch_todo_by_id(
    auth_guard: AuthGuard,
    id: i32,
    request: Json<UpdateTodoItemRequest>,
    updater: &State<Arc<dyn TodoUpdater>>,
) -> Custom<Result<Json<models::Response<models::TodoItemData>>, Json<models::ErrorResponse>>> {
    let request = request.into_inner().into_domain(id, auth_guard.account_id);
    let result = updater.inner().update(request).await;
    match result {
        Ok(item) => Custom(
            Status::Ok,
            Ok(Json(models::Response::from(models::TodoItemData::from(
                &item,
            )))),
        ),
        Err(err) => Custom(
            Status::from(&err),
            Err(Json(models::ErrorResponse::from(&err))),
        ),
    }
}
