use axum::{Extension, extract::Path, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, ActiveValue};
use crate::entities::task;



pub async fn delete_task_by_id(Extension(db): Extension<DatabaseConnection>, Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)>{
     
    task::Entity::delete_by_id(id).exec(&db).await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(StatusCode::ACCEPTED)
}


pub async fn soft_delete_task(Extension(db): Extension<DatabaseConnection>, Path(id): Path<i32>) -> Result<StatusCode, (StatusCode, String)>{
    let mut active_task = if let Some(task) = task::Entity::find_by_id(id).one(&db).await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?{
        task.into_active_model()
    } else {
        return Err((StatusCode::NOT_FOUND, "Task not found".to_owned()));
    };
    let now = chrono::offset::Utc::now();

    active_task.deleted_at = ActiveValue::set(Some(now.into()));

    task::Entity::update(active_task).exec(&db)
                                                            .await
                                                            .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;


    Ok(StatusCode::OK)

}   