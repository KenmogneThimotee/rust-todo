use super::super::schema::{Task, CreateTask};
use crate::data::establish_connection;
use diesel::result::Error ;
use diesel::prelude::*;


pub fn create_task(name: String, description: String, due_date: String, precedence: i32, status_id: i32) -> Result<Task, Error> {
    use crate::schema::task::task ; 
    let conn = & mut establish_connection();
    let new_status = CreateTask{
        name: name,
        description: description,
        due_date: due_date,
        precedence: precedence,
        status_id: status_id
    };

    Ok(diesel::insert_into(task::table)
        .values(&new_status)
        .returning(Task::as_returning())
        .get_result(conn)?)
}

pub fn update_task(task_id: i32,task_name: String, task_description: String, task_due_date: String, task_precedence: i32, task_archive: bool) -> Result<Task, Error> {
    use crate::schema::task::task::dsl::* ; 
    let connection = &mut establish_connection();

    let returned_status = diesel::update(task.find(task_id))
        .set((name.eq(task_name), description.eq(task_description),
            due_date.eq(task_due_date), precedence.eq(task_precedence), archive.eq(task_archive)
        ))
        .returning(Task::as_returning())
        .get_result(connection)?;

    Ok(returned_status)
}



pub fn get_task_by_name(task_name: String) -> Result<Task, Error> {
    use crate::schema::task::task::dsl::* ; 

    let connection = &mut establish_connection();
    let result = task
        .filter(name.eq(&task_name))
        .limit(1)
        .select(Task::as_select())
        .first(connection)?;

    Ok(result)
}

pub fn get_task_by_status(task_status_id: i32) -> Result<Vec<Task>, Error> {
    use crate::schema::task::task::dsl::* ; 

    let connection = &mut establish_connection();
    let results = task
        .filter(status_id.eq(&task_status_id))
        .select(Task::as_select())
        .load(connection)?;

    Ok(results)
}

pub fn get_task_by_id(task_id: i32) -> Result<Task, Error> {
    use crate::schema::task::task::dsl::* ; 

    let connection = &mut establish_connection();
    let result = task
        .find(&task_id)
        .select(Task::as_select())
        .first(connection)?;

    Ok(result)
}

pub fn get_task(search_value: String) -> Result<Vec<Task>, Error> {

    use crate::schema::task::task::dsl::* ; 

    let connection = &mut establish_connection();

    let search_results: Vec<Task>;

    if search_value != ""{
        let pattern = format!("%{}%", search_value);
        search_results = task
            .filter(name.like(pattern.clone())
            .or(description.like(pattern.clone())))
            .order_by(precedence.asc())
            .select(Task::as_select()).load::<Task>(connection)?;
    }else{
        search_results = task.order_by(precedence.asc()).select(Task::as_select()).load::<Task>(connection)?;
    }


    Ok(search_results)
}


pub fn delete_task(task_id: i32) -> Result<usize, Error> {
    use crate::schema::task::task::dsl::* ; 

    let connection = &mut establish_connection();


    diesel::delete(task.find(task_id))
        .execute(connection)
}