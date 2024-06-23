
use diesel::prelude::*;
pub(crate) mod status;
pub(crate) mod task;

#[derive(Queryable, Identifiable, Selectable, PartialEq)]
#[diesel(table_name = status::status )]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
#[derive(Clone)]
pub struct Status {
    pub id: i32,
    pub name: String,
    pub color: String,
}


#[derive(Insertable)]
#[diesel(table_name = status::status )]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
pub struct CreateStatus {
    pub name: String,
    pub color: String,
}


#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(table_name = task::task )]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Status))]
#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub precedence: i32,
    pub status_id: i32,
    pub due_date: String,
    pub archive: bool
}

#[derive(Insertable)]
#[diesel(table_name = task::task )]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
#[derive(Clone)]
pub struct CreateTask {
    pub name: String,
    pub description: String,
    pub precedence: i32,
    pub status_id: i32,
    pub due_date: String
}
