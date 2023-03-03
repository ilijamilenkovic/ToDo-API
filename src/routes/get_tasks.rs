
use axum::{extract::{Path, Query},Extension, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, Condition, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::entities::task::{Entity as Task, self};




#[derive(Serialize)]
pub struct ResponseTask{
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}


pub async fn get_task_by_id(
    Path(id): Path<i32>,
    Extension(db): Extension<DatabaseConnection>
) -> Result<(StatusCode, Json<ResponseTask>),(StatusCode, String)>{


    let res = Task::find_by_id(id)
    .filter(task::Column::DeletedAt.is_null())
    .one(&db)
    .await
    .map_err(|err|->(StatusCode, String){ 
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })?;

    match res {
        Some(data)=>{
            Ok((StatusCode::OK, Json(ResponseTask{
                id: data.id,
                priority: data.priority,
                title: data.title,
                description: data.description,
                

            })))
        },
        None=>{
            Err((StatusCode::NOT_FOUND, "Couldn't find the task.".to_owned()))
        }
    }
}

#[derive(Deserialize)]
pub struct TaskQueryParams{
    priority: Option<String>,

}

pub async fn get_task(Extension(db): Extension<DatabaseConnection>, Query(params): Query<TaskQueryParams>) -> Result<(StatusCode, Json<Vec<ResponseTask>>), (StatusCode, String)>{
    
    let mut filter = Condition::all().add(task::Column::DeletedAt.is_null());//.add(task::Column::Priority.eq(params.priority).into_condition());

    if let Some(priority) = params.priority{
        filter = if priority.is_empty(){
            filter.add(task::Column::Priority.is_null())
        }else{  
            filter.add(task::Column::Priority.eq(priority))
        };
    }

    let tasks: Vec<ResponseTask> = Task::find()
                                .filter(filter)
                                .all(&db)
                                .await
                                .map_err(|err|->(StatusCode, String){
                                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                                })?
                                .into_iter()
                                .map(|db_task|
                                    ResponseTask { 
                                        id: db_task.id, 
                                        title: db_task.title, 
                                        priority: db_task.priority, 
                                        description: db_task.description}).collect();

    
    if tasks.len() == 0 {
        return Err((StatusCode::NOT_FOUND, "Couldn't find any task".to_owned()));
    }


    Ok((StatusCode::OK, Json(tasks)))



    

}

