use sea_orm::DatabaseConnection;
use sea_orm::Database;
use sea_orm::DbErr;

async fn init_data() -> DatabaseConnection{
    let database_connection  = Database::connect("sqlite://db.sqlite?mode=rwc").await;
    match database_connection {
        Ok(db) => {
            return  db;
        },
        Err(_) => panic!("We are not able to get connection from the database")
    }
}