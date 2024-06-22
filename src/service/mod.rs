use std::os::linux::raw::stat;

use crate::schema::Status;
use crate::data;

pub fn create_status(name: String, color: String, if_exist: String){


    let status = data::get_status_by_name(name.clone()).unwrap();

    match status {
        Some(value) => {
            if if_exist == "donothing" {
                println!("The status already exist")
            }else{
                let status = data::update_status(value.id, value.name, value.color).unwrap();
                println!("Updated Status {:?}", status);

            }
        },
        None => {
            let status = data::create_status(name, color).unwrap();
            println!("Status {:?}", status);
        }
    }

}