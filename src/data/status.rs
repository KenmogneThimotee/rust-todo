use super::super::schema::{Status, CreateStatus};
use crate::data::{self, establish_connection};
use diesel::result::Error ;
use diesel::prelude::*;


pub fn create_status(name: String, color: String) -> Result<Status, Error> {
    use crate::schema::status::status ; 
    let conn = & mut establish_connection();
    let new_status = CreateStatus{
        name: name,
        color: color
    };

    Ok(diesel::insert_into(status::table)
        .values(&new_status)
        .returning(Status::as_returning())
        .get_result(conn)?)
}

pub fn update_status(status_id: i32, status_name: String, status_color: String) -> Result<Status, Error> {
    use crate::schema::status::status::dsl::* ; 
    let connection = &mut establish_connection();

    let returned_status = diesel::update(status.find(status_id))
        .set((name.eq(status_name), color.eq(status_color)))
        .returning(Status::as_returning())
        .get_result(connection)?;

    Ok(returned_status)
}

pub fn get_status_by_name(status_name: String) -> Result<Status, Error> {
    use crate::schema::status::status::dsl::* ; 

    let connection = &mut establish_connection();
    let result = status
        .filter(name.eq(&status_name))
        .limit(1)
        .select(Status::as_select())
        .first(connection)?;

    

    Ok(result)

}

pub fn get_status_by_id(status_id: i32) -> Result<Status, Error> {
    use crate::schema::status::status::dsl::* ; 

    let connection = &mut establish_connection();
    let result = status
        .find(&status_id)
        .select(Status::as_select())
        .first(connection)?;

    Ok(result)
}

pub fn get_status(order_by: String, status_name: String) -> Result<Vec<Status>, Error> {
    use crate::schema::status::status::dsl::* ; 

    let connection = &mut establish_connection();

    let pattern = format!("%{}%", status_name);
    let results: Vec<Status>;
    if order_by == "asc" {
        results = status
        .filter(name.like(pattern))
        .select(Status::as_select())
        .order_by(name.asc())
        .load(connection)?;
    
    }else {
        results = status
        .filter(name.like(pattern))
        .select(Status::as_select())
        .order_by(name.desc())
        .load(connection)?;
    
    }

    Ok(results)

}

pub fn delete_status(status_id: i32, cascade: bool) -> Result<usize, Error> {
    use crate::schema::status::status::dsl::* ; 


    let connection = &mut establish_connection();

    if cascade {
        let tasks = data::task::get_task_by_status(status_id)?;
        for task in tasks {
            data::task::delete_task(task.id)?;
        }
    }
    diesel::delete(status.find(status_id))
        .execute(connection)

}