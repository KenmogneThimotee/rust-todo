
use crate::data;
use crate::schema::Status;
use diesel::result::Error;
use dotenvy::dotenv;
use std::env;

use super::Service;


impl  Service {
    pub fn create_status(&mut self, name: String, color: String, if_exist: String) -> Result<Status, Error>{


        let status = self.database.get_status_by_name(name.clone());
    
        match status {
            Ok(value) => {
                if if_exist == "donothing" {
                    Err(Error::NotFound)
                }else{
                    self.database.update_status(value.id, value.name, value.color)
                }
            },
            Err(Error::NotFound) => {
                self.database.create_status(name, color)
            },
            Err(error) => Err(error)
        }
    }


    pub fn update_status(&mut self, status_id: i32, status_name: String, status_color: String) -> Result<Status, Error> {
        self.database.update_status(status_id, status_name, status_color)
    }


    pub fn get_status_by_name(&mut self, status_name: String) -> Result<Status, Error> {
        self.database.get_status_by_name(status_name.clone())
    }

    pub fn get_status_by_id(&mut self, status_id: i32) -> Result<Status, Error> {
        self.database.get_status_by_id(status_id)
    }

    pub fn get_status(&mut self, order_by: String, status_name: String) -> Result<Vec<Status>, Error> {
        self.database.get_status(order_by, status_name)
    }

    pub fn delete_status(&mut self, status_id: i32, cascade: bool) -> Result<usize, Error> {
        self.database.delete_status(status_id, cascade)
    }
}
