use clap::{Arg, ArgMatches, Command};

use super::View;


impl View {
    pub fn create_task(&self) -> Command {

        Command::new("task")
            .args([
                Arg::new("name")
                .long("name")
                .short('n')
                .required(true),
                Arg::new("description")
                .long("description")
                .short('d')
                .required(true),
                Arg::new("due_date")
                .long("due_date")
                .short('t')
                .required(true),
                Arg::new("status_id")
                .long("status_id")
                .short('s')
                .value_parser(clap::value_parser!(i32))
                .required(true),
                Arg::new("precedence")
                .long("precedence")
                .short('p')
                .value_parser(clap::value_parser!(i32))
                .required(true),
                Arg::new("if_exist")
                .long("if_exist")
                .required(false)
                .default_value("donothing")
                .value_parser(["update", "donothing"])
            ])
    }
    
    pub fn process_create_task(&mut self, task: &ArgMatches){

        let name: &String = task.get_one("name").expect("Name is required");
        let description: &String = task.get_one("description").expect("Description is required");
        let due_date: &String = task.get_one("due_date").expect("Due date is required");
        let status_id: &i32 = task.get_one("status_id").expect("Status id is required");
    
        let precedence: &i32 = task.get_one("precedence").expect("Precedence is required");
        let if_exist: &String = task.get_one("if_exist").expect("if_exist is required");
    
    
        self.controller.create_task(name.to_string(), description.to_string(), due_date.to_string(), precedence.clone(), status_id.clone() ,if_exist.to_string());
    }

    pub fn update_task(&self) -> Command {
        Command::new("task")
        .args([
            Arg::new("id")
            .long("id")
            .value_parser(clap::value_parser!(i32))
            .required(true),
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true),
            Arg::new("description")
            .long("description")
            .short('d')
            .required(true),
            Arg::new("due_date")
            .long("due_date")
            .short('t')
            .required(true),
            Arg::new("precedence")
            .long("precedence")
            .short('p')
            .value_parser(clap::value_parser!(i32))
            .required(true),
            Arg::new("archive")
            .long("archive")
            .short('a')
            .value_parser(clap::value_parser!(bool))
            .required(true),
        ])
    }
    
    pub fn process_update_task(&mut self, task: &ArgMatches){

        let id: &i32 = task.get_one("id").expect("Id is required");
        let name: &String = task.get_one("name").expect("Name is required");
        let description: &String = task.get_one("description").expect("Description is required");
        let due_date: &String = task.get_one("due_date").expect("Due date is required");
    
        let precedence: &i32 = task.get_one("precedence").expect("Precedence is required");
        let archive: &bool = task.get_one("archive").expect("Archive is required");
    
    
        self.controller.update_task(id.clone(), name.to_string(), description.to_string(), due_date.to_string(), precedence.clone(), archive.clone())
    }
    
    pub fn get_task_by_name(&self) -> Command {
        Command::new("task")
        .args([
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true)
        ])
    
    }
    
    pub fn process_get_task_by_name(&mut self, task: &ArgMatches){
        let name: &String = task.get_one("name").expect("Name is required");
        self.controller.get_task_by_name(name.to_string());
    } 

    pub fn get_task_by_id(&self) -> Command {
        Command::new("task")
        .args([
            Arg::new("id")
            .long("id")
            .short('i')
            .required(true)
        ])
    
    }

    pub fn process_get_task_by_id(&mut self, task: &ArgMatches){
        let id: &i32 = task.get_one("id").expect("Id is required");
        self.controller.get_task_by_id(id.clone());
    } 
    
    pub fn get_task(&self) -> Command {
        Command::new("task")
        .args([
            Arg::new("search")
            .long("search")
            .short('s')
            .required(false)
            .default_value("")
        ])
    
    }

    pub fn process_get_task(&mut self, task: &ArgMatches){
        let search: &String = task.get_one("search").expect("Query is required");
        self.controller.get_task(search.to_string());
    } 
    
    pub fn delete_task(&self) -> Command {
        Command::new("task")
        .args([
            Arg::new("id")
            .long("id")
            .short('i')
            .required(true)
        ])
    
    }

    pub fn process_delete_task(&mut self, task: &ArgMatches){
        let id: &i32 = task.get_one("id").expect("Id is required");
        self.controller.delete_task(id.clone());
    } 
}
