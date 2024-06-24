use clap::{arg, command, value_parser, ArgAction, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("CLI Tool")
        .version("1.0")
        .author("Rohit <iamawesome@earth.com>")
        .about("Performs some operations")
        .arg(arg!([NAME] "Type your name for greetings").required(false))
        .arg(arg!(-c --cool [cool] "true for a more cooler greeting").required(false))
        .get_matches();

    if let Some(name) = matches.get_one::<String>("NAME") {
        let name = name.trim();
        if let Some(cool) = matches.get_one::<String>("cool"){
            if cool == "true" {
                println!("What a amazing day, {}", name);
            }else if cool == "false" {
                println!("Chill a bit, {}", name);
            }else{
                println!("It's either true or false. Don't be a quantum system!!!");
            }
        } 
    } else {
        println!("Hello, Rustacean!");
    }
}
