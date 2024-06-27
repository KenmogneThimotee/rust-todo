use super::super::schema::{Status, CreateStatus};
use crate::data::DataBase;
use crate::schema::Task;
use diesel::result::Error ;
use diesel::prelude::*;


impl DataBase {
    pub fn create_status(&mut self, name: String, color: String) -> Result<Status, Error> {
        use crate::schema::status::status ; 
        let connection = &mut self.conn;
        let new_status = CreateStatus{
            name: name,
            color: color
        };
    
        Ok(diesel::insert_into(status::table)
            .values(&new_status)
            .returning(Status::as_returning())
            .get_result(connection)?)
    }


    pub fn update_status(&mut self, status_id: i32, status_name: String, status_color: String) -> Result<Status, Error> {
        use crate::schema::status::status::dsl::* ; 
        let connection = &mut self.conn;

        let returned_status = diesel::update(status.find(status_id))
            .set((name.eq(status_name), color.eq(status_color)))
            .returning(Status::as_returning())
            .get_result(connection)?;

        Ok(returned_status)
    }

    pub fn get_status_by_name(&mut self, status_name: String) -> Result<Status, Error> {
        use crate::schema::status::status::dsl::* ; 

        let connection = &mut self.conn;
        let result = status
            .filter(name.eq(&status_name))
            .limit(1)
            .select(Status::as_select())
            .first(connection)?;

        

        Ok(result)

    }

    pub fn get_status_by_id(&mut self, status_id: i32) -> Result<Status, Error> {
        use crate::schema::status::status::dsl::* ; 

        let connection = &mut self.conn;
        let result = status
            .find(&status_id)
            .select(Status::as_select())
            .first(connection)?;

        Ok(result)
    }

    pub fn get_status(&mut self, order_by: String, status_name: String) -> Result<Vec<Status>, Error> {
        use crate::schema::status::status::dsl::* ; 

        let connection = &mut self.conn;

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

    pub fn delete_status(&mut self, task_status_id: i32, cascade: bool) -> Result<usize, Error> {
        use crate::schema::status::status::dsl::* ; 


        let connection = &mut self.conn;
        if cascade {
            use crate::schema::task::task::dsl::*; 
            
            let tasks = task.filter(status_id.eq(task_status_id))
            .select(Task::as_select())
            .load(connection)?;
            for task_item in tasks {
                diesel::delete(task.find(task_item.id))
                .execute(connection)?;
            }
        }
        diesel::delete(status.find(task_status_id))
            .execute(connection)

    }

}
