
use diesel::result::Error;

use super::Controller;


impl Controller {
    pub fn create_task(&mut self, name: String, description: String, due_date: String, precedence: i32, status_id: i32, if_exist: String){

        let task = self.service.create_task(name, description, due_date, precedence, status_id, if_exist);
        
        match task {
            Ok(value) => println!("Task created {:?}", value),
            Err(Error::NotFound) => println!("Nothing has been done."),
            Err(_) => println!("Status doesn't exist.")
        }
    }

    pub fn update_task(&mut self, task_id: i32,task_name: String, task_description: String, task_due_date: String, task_precedence: i32, archive: bool){
        let task = self.service.update_task(task_id, task_name, task_description, task_due_date, task_precedence, archive);
    
        match task {
            Ok(value) => println!("Task updated {:?}", value),
            Err(_) => println!("Error updating the task")
        }
    }

    pub fn get_task_by_name(&mut self, task_name: String){
        let task = self.service.get_task_by_name(task_name); 
    
        match task {
            Ok(value) => println!("Task {:?}", value),
            Err(_) => println!("Error getting the task")
        }
    
    }

    pub fn get_task_by_id(&mut self, task_id: i32){
        let task = self.service.get_task_by_id(task_id); 
    
        match task {
            Ok(value) => println!("Task {:?}", value),
            Err(_) => println!("Error getting the task")
        }
    
    }

    pub fn get_task(&mut self, search_value: String){

        let tasks = self.service.get_task(search_value);
    
        match tasks {
            Ok(values) => {
                for task in values {
                    println!("{:?}", task);
                }
            },
            Err(_) => println!("Error getting tasks")
        }
    }

    pub fn delete_task(&mut self, task_id: i32){
        let size = self.service.delete_task(task_id);
    
        match size {
            Ok(_) => println!("Task deleted successfully"),
            Err(_) => println!("Error deleting the task.")
        }
    }
}




