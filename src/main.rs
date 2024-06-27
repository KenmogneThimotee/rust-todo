use view::View ;

mod view;
mod controller;
mod schema;
mod service;
mod data;
use dotenvy::dotenv;
use std::env;
fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut view = View::new(database_url);

    view.get_input_command();
}
