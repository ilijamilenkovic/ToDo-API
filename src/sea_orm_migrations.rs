use sea_orm::{DbErr, Database, Statement, ConnectionTrait};


#[allow(dead_code)]
pub async fn run(database_url: &str, database_name: &str) -> Result<(), DbErr>
{
    let db = Database::connect(database_url).await?;
    
    db.execute(Statement::from_string(
                       db.get_database_backend(),
                       format!("DROP DATABASE IF EXISTS \"{}\";", database_name),
                   ))
                   .await?;
    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE \"{}\";", database_name),
    ))
    .await?;
        
                   
    Ok(())

    
}