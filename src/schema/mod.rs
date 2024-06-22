
use diesel::prelude::*;
pub(crate) mod status;

#[derive(Queryable, Selectable )]
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


