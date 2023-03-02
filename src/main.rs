mod sea_orm_migrations;
use tasks_axum::{run,connect_database};
use dotenv::dotenv;
use dotenv_codegen::dotenv;





#[tokio::main]
async fn main(){
    dotenv().ok();
    let database_uri: String = dotenv!("DATABASE_URI").to_owned();



    let db_connection = connect_database(database_uri).await;
    match db_connection {
        Ok(db_connection)=>{
            run(db_connection).await;  
        }
        Err(error) =>{
            panic!("{}",error);
        }
    }

    
}
