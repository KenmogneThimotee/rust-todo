use super::super::schema::{Status, CreateStatus};
use crate::data::establish_connection;
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

pub fn get_status_by_name(status_name: String) -> Result<Option<Status>, Error> {
    use crate::schema::status::status::dsl::* ; 

    let connection = &mut establish_connection();
    let results = status
        .filter(name.eq(&status_name))
        .limit(1)
        .select(Status::as_select())
        .load(connection)?;

    
    if results.len() == 1 {
        let returned_status = Status{
            id: results[0].id,
            name: results[0].name.clone(),
            color: results[0].color.clone()
        };
        Ok(Some(returned_status))
    }else{
        Ok(None)
    }
}