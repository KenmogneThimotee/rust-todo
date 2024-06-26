use crate::data::{self, DataBase};



pub(crate) mod status;
pub(crate) mod task;

pub struct Service {
    database: DataBase
}


impl  Service {
    
    pub fn new(database_url: String) -> Self {

        let database = data::DataBase::new(database_url);
        Service {
            database: database
        }
    }
}