use clap::Command;

mod status;
mod task;


pub fn get_input_command(){

    let command = Command::new("todo")
    .subcommand(
        Command::new("create")
        .subcommand(status::create_status())
        .subcommand(task::create_task())
    )
    .subcommand(
        Command::new("update")
        .subcommand(status::update_status())
        .subcommand(task::update_task())
    )
    .subcommand(
        Command::new("list")
        .subcommand(status::get_status())
        .subcommand(task::get_task())
    )
    .subcommand(
        Command::new("get")
        .subcommand(status::get_status_by_id())
        .subcommand(task::get_task_by_id())
    )
    .subcommand(
        Command::new("get_n")
        .subcommand(status::get_status_by_name())
        .subcommand(task::get_task_by_name())
    )
    .subcommand(
        Command::new("delete")
        .subcommand(status::deleye_status())
        .subcommand(task::delete_task())
    )
    .get_matches();


    match command.subcommand() {
        Some(("create", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_create_task(task_command),
                Some(("status", status_command)) =>  status::process_create_status(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        Some(("update", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_update_task(task_command),
                Some(("status", status_command)) =>  status::process_update_status(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        Some(("list", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_get_task(task_command),
                Some(("status", status_command)) =>  status::process_get_status(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        Some(("get", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_get_task_by_id(task_command),
                Some(("status", status_command)) =>  status::process_get_status_by_id(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        Some(("get_n", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_get_task_by_name(task_command),
                Some(("status", status_command)) =>  status::process_get_status_by_name(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        Some(("delete", sub_command)) => {
            match sub_command.subcommand() {
                Some(("task", task_command)) => task::process_delete_task(task_command),
                Some(("status", status_command)) =>  status::process_delete_status(status_command),
                _ => println!("Not valid subcommand")
            }
        },
        _ => println!("Not valid subcommand")
    }


}

