use clap::{Arg, ArgMatches, Command};
use regex::Regex;
use crate::controller;

fn parse_hex_color(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^#?([0-9a-fA-F]{6}|[0-9a-fA-F]{3})$")
        .map_err(|_| "Failed to compile regex")?;
    if re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err(format!("'{}' is not a valid hexadecimal color value", s))
    }
}

pub fn create_status() -> Command {

    Command::new("status")
        .args([
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true),
            Arg::new("color")
            .long("color")
            .short('c')
            .value_parser(clap::builder::ValueParser::new(parse_hex_color))
            .required(true),
            Arg::new("if_exist")
            .long("if_exist")
            .required(false)
            .default_value("donothing")
            .value_parser(["update", "donothing"])
        ])
}

pub fn process_create_status(status: &ArgMatches){

    let name: &String = status.get_one("name").expect("Name is required");
    let color: &String = status.get_one("color").expect("Color is required");
    let if_exist: &String = status.get_one("if_exist").expect("if_exist is required");

    controller::status::create_status(name.to_string(), color.to_string(), if_exist.to_string());
}

pub fn update_status() -> Command {

    Command::new("status")
        .args([
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true),
            Arg::new("color")
            .long("color")
            .short('c')
            .value_parser(clap::builder::ValueParser::new(parse_hex_color))
            .required(true),
            Arg::new("id")
            .long("id")
            .short('i')
            .required(false)
            .value_parser(clap::value_parser!(i32))
        ])
}

pub fn process_update_status(status: &ArgMatches){

    let name: &String = status.get_one("name").expect("Name is required");
    let color: &String = status.get_one("color").expect("Color is required");
    let id: &i32 = status.get_one("id").expect("Id is required");

    controller::status::update_status(id.clone(), name.to_string(), color.to_string());
}

pub fn get_status_by_name() -> Command{

    Command::new("status")
        .args([
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true)
        ])
}

pub fn process_get_status_by_name(status: &ArgMatches){
    let name: &String = status.get_one("name").expect("Name is required");

    controller::status::get_status_by_name(name.to_string());
}

pub fn get_status_by_id() -> Command{

    Command::new("status")
        .args([
            Arg::new("id")
            .long("id")
            .short('i')
            .required(true)
        ])
}

pub fn process_get_status_by_id(status: &ArgMatches){
    let id: &i32 = status.get_one("id").expect("Id is required");

    controller::status::get_status_by_id(id.clone());
}


pub fn get_status() -> Command{

    Command::new("status")
        .args([
            Arg::new("order_by")
            .long("order_by")
            .short('o')
            .required(true),
            Arg::new("name")
            .long("name")
            .short('n')
            .required(true)
        ])
}

pub fn process_get_status(status: &ArgMatches){
    let name: &String = status.get_one("name").expect("Name is required");
    let order_by: &String = status.get_one("order_by").expect("Order by is required");

    controller::status::get_status(order_by.to_string(), name.to_string());
}


pub fn deleye_status() -> Command{

    Command::new("status")
        .args([
            Arg::new("id")
            .long("id")
            .short('i')
            .required(true),
            Arg::new("cascade")
            .long("cascade")
            .short('c')
            .required(false)
            .value_parser(clap::value_parser!(bool))
        ])
}

pub fn process_delete_status(status: &ArgMatches){
    let id: &i32 = status.get_one("id").expect("Id is required");
    let cascade: &bool = status.get_one("cascade").unwrap_or(&false);
    controller::status::delete_status(id.clone(), cascade.clone());
}
