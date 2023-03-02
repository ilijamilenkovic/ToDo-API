

use crate::entities::task;
use axum::{extract::Path, Extension, Json, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, prelude::DateTimeWithTimeZone, ActiveValue};
use serde::{Deserialize, Serialize};


#[derive(Deserialize,Serialize)]
pub struct TaskBody {   
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub is_default: Option<bool>,
    pub user_id: Option<i32>,
}

pub async fn update_task_put(Path(id): Path<i32>, 
Extension(db): Extension<DatabaseConnection>, 
Json(mut task_body): Json<TaskBody>)-> Result<StatusCode, (StatusCode, String)>{
    //dbg!(&task_body);    

    let update_task = task::ActiveModel{
         id: ActiveValue::set(id),
         priority: ActiveValue::set(task_body.priority),
         completed_at: ActiveValue::set(task_body.completed_at),
         description: ActiveValue::set(task_body.description),
         deleted_at: ActiveValue::set(task_body.deleted_at),
         is_default: ActiveValue::set(task_body.is_default),
         user_id: ActiveValue::set(task_body.user_id),
         title: ActiveValue::set(task_body.title)
    };

    task::Entity::update(update_task)
    .exec(&db)
    .await
    .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    task_body.id = Some(id);

    Ok(StatusCode::OK)

    
    

}