use crate::entities::task;
use axum::{extract::Path, Extension, Json, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, prelude::DateTimeWithTimeZone, ActiveValue, IntoActiveModel};
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



//Double option in order to determine if it's sometgin NULL, or just not set.
//Some(Some(value)) - property set to value
//Some(None) - property set to NULL
//None - property not set
#[derive(Deserialize)]
pub struct TaskBodyDoubleOption {   
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
   
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,


}

pub async fn update_task_patch(Path(id): Path<i32>, Extension(db): Extension<DatabaseConnection>, Json(task_body): Json<TaskBodyDoubleOption>) -> Result<StatusCode, (StatusCode, String)>{

    //1. get task from the database and make it into active model
    let mut active_task = if let Some(task) = task::Entity::find_by_id(id)
                                                                                .one(&db)
                                                                                .await
                                                                                .map_err(|err| 
                                                                                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?{
        task.into_active_model()
    } else{
        return Err((StatusCode::NOT_FOUND, "Task not found".to_owned()));
    };


    //2. change the active model according to the fields which are set in the json body
    if let Some(priority) = task_body.priority{
        active_task.priority = ActiveValue::set(priority);
    }
    if let Some(title) = task_body.title{
        active_task.title = ActiveValue::set(title);
    }
    if let Some(completed_at) = task_body.completed_at{
        active_task.completed_at = ActiveValue::set(completed_at);
    }
    if let Some(description) = task_body.description{
        active_task.description = ActiveValue::set(description);
    }
    if let Some(deleted_at) = task_body.deleted_at{
        active_task.deleted_at = ActiveValue::set(deleted_at);
    }

    task::Entity::update(active_task).exec(&db).await.map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    
    Ok(StatusCode::OK)
    
}


