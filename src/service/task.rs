
use crate::data;
use crate::schema::Task;
use diesel::result::Error;

pub fn create_task(name: String, description: String, due_date: String, precedence: i32, status_id: i32, if_exist: String) -> Result<Task, Error>{


    let task = data::task::get_task_by_name(name.clone());

    match task {
        Ok(value) => {
            if if_exist == "donothing" {
                Err(Error::NotFound)
            }else{
                data::task::update_task(value.id, value.name, value.description, value.due_date, value.precedence, value.archive)
            }
        },
        Err(Error::NotFound) => {
            data::task::create_task(name, description, due_date, precedence, status_id)
        },
        Err(error) => Err(error)
    }

}


pub fn update_task(task_id: i32,task_name: String, task_description: String, task_due_date: String, task_precedence: i32, archive: bool) -> Result<Task, Error> {
    data::task::update_task(task_id,   task_name, task_description, task_due_date, task_precedence, archive)
}


pub fn get_task_by_name(task_name: String) -> Result<Task, Error> {
    data::task::get_task_by_name(task_name.clone())
}

pub fn get_task_by_id(task_id: i32) -> Result<Task, Error> {
    data::task::get_task_by_id(task_id)
}

pub fn get_task(search_value: String) -> Result<Vec<Task>, Error> {
    data::task::get_task(search_value)
}

pub fn delete_task(task_id: i32) -> Result<usize, Error> {
    data::task::delete_task(task_id)
}