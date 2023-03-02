use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer,Any};
use axum::{Router, routing::{get, post, put}, http::Method, Extension};

mod create_task;
mod get_tasks;
mod update_task;


use create_task::create_task;
use get_tasks::{get_task_by_id,get_task};
use update_task::update_task_put;



pub fn create_routes(db_connection: DatabaseConnection) -> Router {

    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any);

    Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/tasks", post(create_task))
    .route("/tasks/:id", get(get_task_by_id))
    .route("/tasks", get(get_task))
    .route("/tasks/:id", put(update_task_put))
    .layer(Extension(db_connection))
    .layer(cors)

}