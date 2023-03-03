use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer,Any};
use axum::{Router, routing::{get, post, put, patch, delete}, http::Method, Extension};

mod create_task;
mod get_tasks;
mod update_task;
mod delete_tasks;


use create_task::create_task;
use get_tasks::{get_task_by_id,get_task};
use update_task::{update_task_put, update_task_patch};
use delete_tasks::{delete_task_by_id, soft_delete_task};



pub fn create_routes(db_connection: DatabaseConnection) -> Router {

    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE]).allow_origin(Any);

    Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/tasks", post(create_task))
    .route("/tasks/:id", get(get_task_by_id))
    .route("/tasks", get(get_task))
    .route("/tasks/:id", put(update_task_put))
    .route("/tasks/:id", patch(update_task_patch))
    .route("/tasks/:id", delete(delete_task_by_id))
    .route("/tasks/soft_delete/:id", delete(soft_delete_task))
    .layer(Extension(db_connection))
    .layer(cors)

}