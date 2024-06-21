use clap::{Arg, Command};
use regex::Regex;
use crate::controller::create_status;


fn parse_hex_color(s: &str) -> Result<String, String> {
    let re = Regex::new(r"^#?([0-9a-fA-F]{6}|[0-9a-fA-F]{3})$")
        .map_err(|_| "Failed to compile regex")?;
    if re.is_match(s) {
        Ok(s.to_string())
    } else {
        Err(format!("'{}' is not a valid hexadecimal color value", s))
    }
}

pub fn get_input_command(){

    let command = Command::new("todo")
    .subcommand(
        Command::new("create")
        .subcommand(Command::new("status")
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
        ]))
    ).get_matches();


    match command.subcommand() {
        Some(create) => {
            match create.1.subcommand() {
                Some(status) => {
                    let name: &String = status.1.get_one("name").expect("Name is required");
                    let color: &String = status.1.get_one("color").expect("Color is required");
                    let if_exist: &String = status.1.get_one("if_exist").expect("if_exist is required");

                    create_status(name.to_string(), color.to_string(), if_exist.to_string());
                    println!("Name: {if_exist}");
                },
                _ => println!("Not valid subcommand")
            }
        },
        _ => println!("Not  valid subcommand")
    }


}

