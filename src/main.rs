extern crate clap;
use clap::{App, SubCommand};

mod day1;
mod day2;

fn main() {
    let matches = App::new("advent")
        .version("1.0")
        .author("Jupp Mueller <jupp0r@gmail.com>")
        .about("Advent of code 2019")
        .subcommand(SubCommand::with_name("day1"))
        .subcommand(SubCommand::with_name("day2"))
        .get_matches();

    if matches.subcommand_matches("day1").is_some() {
        day1::run();
    }
    if matches.subcommand_matches("day2").is_some() {
        day2::run();
    }
}
