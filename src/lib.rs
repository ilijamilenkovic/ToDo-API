use routes::create_routes;
use sea_orm::{DatabaseConnection, DbErr, Database};

mod entities;
mod routes;


pub async fn run(db_connection: DatabaseConnection){
    let app = create_routes(db_connection);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub async fn connect_database(database_uri: String) -> Result<DatabaseConnection, DbErr>{
    let db_connection = Database::connect(&database_uri).await?;
    Ok(db_connection)
}