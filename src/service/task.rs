
use crate::data;
use crate::schema::Task;
use diesel::result::Error;

use super::Service;


impl Service {
    
    pub fn create_task(&mut self, name: String, description: String, due_date: String, precedence: i32, status_id: i32, if_exist: String) -> Result<Task, Error>{


        let task = self.database.get_task_by_name(name.clone());
    
        match task {
            Ok(value) => {
                if if_exist == "donothing" {
                    Err(Error::NotFound)
                }else{
                    self.database.update_task(value.id, value.name, value.description, value.due_date, value.precedence, value.archive)
                }
            },
            Err(Error::NotFound) => {
                self.database.create_task(name, description, due_date, precedence, status_id)
            },
            Err(error) => Err(error)
        }
    
    }
    
    pub fn update_task(&mut self, task_id: i32,task_name: String, task_description: String, task_due_date: String, task_precedence: i32, archive: bool) -> Result<Task, Error> {
        self.database.update_task(task_id,   task_name, task_description, task_due_date, task_precedence, archive)
    }

    pub fn get_task_by_name(&mut self, task_name: String) -> Result<Task, Error> {
        self.database.get_task_by_name(task_name.clone())
    }

    pub fn get_task_by_id(&mut self, task_id: i32) -> Result<Task, Error> {
        self.database.get_task_by_id(task_id)
    }

    pub fn get_task(&mut self, search_value: String) -> Result<Vec<Task>, Error> {
        self.database.get_task(search_value)
    }

    pub fn delete_task(&mut self, task_id: i32) -> Result<usize, Error> {
        self.database.delete_task(task_id)
    }
    
}





