use axum::{Extension, Json, http::StatusCode};
use sea_orm::{DatabaseConnection, ActiveValue, ActiveModelTrait};
use serde::{Deserialize, Serialize};

use crate::entities::task;

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskBody{
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,

}

pub async fn create_task(Extension(database_connection): Extension<DatabaseConnection>, Json(task): Json<TaskBody>) -> Result<(StatusCode, Json<TaskBody>), (StatusCode, String)>{
    
    let return_task = task.clone();

    let new_task = task::ActiveModel{
        priority: ActiveValue::set(task.priority),
        title: ActiveValue::set(task.title),
        description: ActiveValue::set(task.description),
        ..Default::default()
    };

    new_task.save(&database_connection)
    .await
    .map_err(|err|->(StatusCode, String){
        (StatusCode::BAD_REQUEST,err.to_string())})?;

    
    Ok((StatusCode::CREATED, Json(return_task)))

    

    
}