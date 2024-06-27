use clap::Command;

use crate::controller::Controller;

mod status;
mod task;

pub struct View {
    controller: Controller
}

impl View {
    
    pub fn new(database_url: String) -> Self {

        let controller = Controller::new(database_url);

        View{
            controller: controller
        }
    }

    pub fn get_input_command(&mut self){

        let command = Command::new("todo")
        .subcommand(
            Command::new("create")
            .subcommand(self.create_status())
            .subcommand(self.create_task())
        )
        .subcommand(
            Command::new("update")
            .subcommand(self.update_status())
            .subcommand(self.update_task())
        )
        .subcommand(
            Command::new("list")
            .subcommand(self.get_status())
            .subcommand(self.get_task())
        )
        .subcommand(
            Command::new("get")
            .subcommand(self.get_status_by_id())
            .subcommand(self.get_task_by_id())
        )
        .subcommand(
            Command::new("get_n")
            .subcommand(self.get_status_by_name())
            .subcommand(self.get_task_by_name())
        )
        .subcommand(
            Command::new("delete")
            .subcommand(self.deleye_status())
            .subcommand(self.delete_task())
        )
        .get_matches();
    
    
        match command.subcommand() {
            Some(("create", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_create_task(task_command),
                    Some(("status", status_command)) =>  self.process_create_status(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            Some(("update", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_update_task(task_command),
                    Some(("status", status_command)) =>  self.process_update_status(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            Some(("list", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_get_task(task_command),
                    Some(("status", status_command)) =>  self.process_get_status(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            Some(("get", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_get_task_by_id(task_command),
                    Some(("status", status_command)) =>  self.process_get_status_by_id(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            Some(("get_n", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_get_task_by_name(task_command),
                    Some(("status", status_command)) =>  self.process_get_status_by_name(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            Some(("delete", sub_command)) => {
                match sub_command.subcommand() {
                    Some(("task", task_command)) => self.process_delete_task(task_command),
                    Some(("status", status_command)) =>  self.process_delete_status(status_command),
                    _ => println!("Not valid subcommand")
                }
            },
            _ => println!("Not valid subcommand")
        }
    
    
    }
    
    
}

