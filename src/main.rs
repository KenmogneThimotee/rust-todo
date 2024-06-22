use view::get_input_command ;
use data::establish_connection;

mod view;
mod controller;
mod schema;
mod service;
mod data;

fn main() {

    // let runtime = tokio::runtime::Runtime::new().unwrap();

    // runtime.block_on(async {
    //     let _ = init_data().await;
    //     get_input_command();
    //     println!("Hello, world!");
    // });
        get_input_command();
        println!("Hello, world!");
}
