use clap::Command;

use status::create_status;
mod status;



pub fn get_input_command(){

    let command = Command::new("todo")
    .subcommand(
        Command::new("create")
        .subcommand(create_status())
    ).get_matches();


    match command.subcommand() {
        Some(create) => {
            match create.1.subcommand() {
                Some(status) => {
                    status::process_create_status(status.1);
                },
                _ => println!("Not valid subcommand")
            }
        },
        _ => println!("Not  valid subcommand")
    }


}

