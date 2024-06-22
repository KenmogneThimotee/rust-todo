// use sea_orm::DatabaseConnection;
// use sea_orm::Database;
// use sea_orm::DbErr;
// use migration::{Migrator, MigratorTrait};

// pub async fn init_data() -> Result<DatabaseConnection, DbErr>{
//     let database_connection  = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
//     Migrator::up(&database_connection, None).await?;
//     Ok(database_connection)
// }


use diesel::{result::Error, sqlite::SqliteConnection} ;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use super::schema::{Status, CreateStatus};


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_status(name: String, color: String) -> Result<Status, Error> {
    use crate::schema::status::status ; 
    let conn = & mut establish_connection();
    let new_status = CreateStatus{
        name: name,
        color: color
    };

    Ok(diesel::insert_into(status::table)
        .values(&new_status)
        .returning(Status::as_returning())
        .get_result(conn)?)
}

pub fn update_status(id: i32, status_name: String, status_color: String) -> Result<Status, Error> {
    use crate::schema::status::status::dsl::* ; 
    let connection = &mut establish_connection();

    let returned_status = diesel::update(status.find(id))
        .set((name.eq(status_name), color.eq(status_color)))
        .returning(Status::as_returning())
        .get_result(connection)?;

    Ok(returned_status)
}

pub fn get_status_by_name(status_name: String) -> Result<Option<Status>, Error> {
    use crate::schema::status::status::dsl::* ; 

    let connection = &mut establish_connection();
    let results = status
        .filter(name.eq(&status_name))
        .limit(1)
        .select(Status::as_select())
        .load(connection)?;

    
    if results.len() == 1 {
        let returned_status = Status{
            id: results[0].id,
            name: results[0].name.clone(),
            color: results[0].color.clone()
        };
        Ok(Some(returned_status))
    }else{
        Ok(None)
    }
}