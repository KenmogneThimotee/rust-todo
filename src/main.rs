use view::get_input_command ;

mod view;
mod controller;
mod model;
mod service;
mod data;

fn main() {
    get_input_command();
    println!("Hello, world!");
}
