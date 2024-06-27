use diesel::prelude::*;


use diesel::sqlite::SqliteConnection ;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use dotenvy::dotenv;
use std::env;

pub(crate) mod status;
pub(crate) mod task;


pub struct DataBase {

    database_url: String,
    conn: SqliteConnection
}


impl DataBase {
    
    pub fn new(database_url: String) -> Self {

        let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        DataBase{
            database_url: database_url,
            conn: conn
        }
    }

    fn establish_connection(&self) -> SqliteConnection {
    
        SqliteConnection::establish(&self.database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", &self.database_url))

    }

    pub fn run_migrations(&mut self) {
        use diesel_migrations::MigrationHarness;
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        let conn = &mut self.conn; // &mut self.establish_connection();
        conn.run_pending_migrations(MIGRATIONS).unwrap();
        
    }   

}



#[cfg(test)]
mod tests;