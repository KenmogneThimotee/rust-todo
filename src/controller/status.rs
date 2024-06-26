
use crate::service;
use diesel::result::Error;

use super::Controller;


impl Controller {
    
    pub fn create_status(&self, name: String, color: String, if_exist: String){

        let status = self.service.create_status(name, color, if_exist);
    
        match status {
            Ok(value) => println!("Status created {:?}", value),
            Err(Error::NotFound) => println!("Nothing has been done."),
            Err(_) => println!("Internal system error.")
        }
    }


    pub fn update_status(&self, status_id: i32, status_name: String, status_color: String){
        let status = self.service.update_status(status_id, status_name, status_color);

        match status {
            Ok(value) => println!("Status updated {:?}", value),
            Err(_) => println!("Error updating the status")
        }
    }

    pub fn get_status_by_name(&self, status_name: String){
        let status = self.service.get_status_by_name(status_name); 

        match status {
            Ok(value) => println!("Status {:?}", value),
            Err(_) => println!("Error getting the status")
        }

    }

    pub fn get_status_by_id(&self, status_id: i32){
        let status = self.service.get_status_by_id(status_id); 

        match status {
            Ok(value) => println!("Status {:?}", value),
            Err(_) => println!("Error getting the status")
        }

    }

    pub fn get_status(&self, order_by: String, status_name: String){

        let statuses = self.service.get_status(order_by, status_name);

        match statuses {
            Ok(values) => {
                for status in values {
                    println!("{:?}", status);
                }
            },
            Err(_) => println!("Error getting statuses")
        }
    }

    pub fn delete_status(&self, status_id: i32, cascade: bool){
        let size = self.service.delete_status(status_id, cascade);

        match size {
            Ok(_) => println!("Status deleted successfully"),
            Err(_) => println!("Error deleting the task.")
        }
    }
}
// pub fn create_status(name: String, color: String, if_exist: String){

//     let status = service::status::create_status(name, color, if_exist);

//     match status {
//         Ok(value) => println!("Status created {:?}", value),
//         Err(Error::NotFound) => println!("Nothing has been done."),
//         Err(_) => println!("Internal system error.")
//     }
// }

// pub fn update_status(status_id: i32, status_name: String, status_color: String){
//     let status = service::status::update_status(status_id, status_name, status_color);

//     match status {
//         Ok(value) => println!("Status updated {:?}", value),
//         Err(_) => println!("Error updating the status")
//     }
// }

// pub fn get_status_by_name(status_name: String){
//     let status = service::status::get_status_by_name(status_name); 

//     match status {
//         Ok(value) => println!("Status {:?}", value),
//         Err(_) => println!("Error getting the status")
//     }

// }

// pub fn get_status_by_id(status_id: i32){
//     let status = service::status::get_status_by_id(status_id); 

//     match status {
//         Ok(value) => println!("Status {:?}", value),
//         Err(_) => println!("Error getting the status")
//     }

// }

// pub fn get_status(order_by: String, status_name: String){

//     let statuses = service::status::get_status(order_by, status_name);

//     match statuses {
//         Ok(values) => {
//             for status in values {
//                 println!("{:?}", status);
//             }
//         },
//         Err(_) => println!("Error getting statuses")
//     }
// }

// pub fn delete_status(status_id: i32, cascade: bool){
//     let size = service::status::delete_status(status_id, cascade);

//     match size {
//         Ok(_) => println!("Status deleted successfully"),
//         Err(_) => println!("Error deleting the task.")
//     }
// }