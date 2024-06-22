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