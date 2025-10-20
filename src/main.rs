mod auth;
mod domain;
mod handler;
mod repository;
mod service;

use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct Config {
    database_url: String,
    auth_token_duration_seconds: i64,
    auth_token_secret: String,
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let config: Config = envy::from_env().expect("Failed to parse envs.");

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to init DB connection.");

    sqlx::migrate!("./src/migrations")
        .run(&db_pool)
        .await
        .expect("Failed to run DB migrations.");

    let account_repository = repository::account::AccountRepository::new(db_pool.clone());
    let todo_repository = repository::todo::TodoRepository::new(db_pool);

    let password_hasher = auth::PasswordHasher::new();

    let auth_token_generator = auth::AuthTokenGenerator::new(
        time::Duration::seconds(config.auth_token_duration_seconds),
        config.auth_token_secret,
    );

    let account_service = Arc::new(service::account::AccountService::new(
        account_repository,
        password_hasher,
        auth_token_generator,
    ));

    let todo_service = Arc::new(service::todo::TodoService::new(
        todo_repository.clone(),
        todo_repository.clone(),
        todo_repository.clone(),
        todo_repository.clone(),
        todo_repository,
    ));

    rocket::build()
        .manage(account_service as Arc<dyn domain::AccountService>)
        .manage(todo_service.clone() as Arc<dyn domain::TodoCreator>)
        .manage(todo_service.clone() as Arc<dyn domain::TodoListerAndCounter>)
        .manage(todo_service.clone() as Arc<dyn domain::TodoGetter>)
        .manage(todo_service as Arc<dyn domain::TodoUpdater>)
        .mount(
            "/",
            routes![
                handler::register,
                handler::login,
                handler::todo::post_todo,
                handler::todo::get_todo,
                handler::todo::get_todo_by_id,
                handler::todo::patch_todo_by_id
            ],
        )
}
