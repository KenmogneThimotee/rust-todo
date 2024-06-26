use diesel::prelude::*;


use diesel::sqlite::SqliteConnection ;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;
use std::env;

pub(crate) mod status;
pub(crate) mod task;


pub struct DataBase {

    database_url: String
}


impl  DataBase {
    
    pub fn new(database_url: String) -> Self {

        DataBase{
            database_url: database_url
        }
    }

    fn establish_connection(&self) -> SqliteConnection {
    
        SqliteConnection::establish(&self.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.database_url))
    }

    pub fn run_migrations(&self) {
        use diesel_migrations::MigrationHarness;
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        let conn = &mut self.establish_connection();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

    }   

    pub fn teardown_test_db(&self) {
        use diesel::sql_query;
        use diesel::RunQueryDsl;
    
        sql_query("DROP TABLE IF EXISTS TASK").execute(&mut self.establish_connection()).unwrap();
        sql_query("DROP TABLE IF EXISTS STATUS").execute(&mut self.establish_connection()).unwrap();
        // Repeat for other tables if needed
    }
}
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


#[cfg(test)]
mod tests;