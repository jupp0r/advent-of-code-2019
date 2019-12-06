extern crate clap;
use clap::{App, SubCommand};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let matches = App::new("advent")
        .version("1.0")
        .author("Jupp Mueller <jupp0r@gmail.com>")
        .about("Advent of code 2019")
        .subcommand(SubCommand::with_name("day1"))
        .subcommand(SubCommand::with_name("day2"))
        .subcommand(SubCommand::with_name("day3"))
        .subcommand(SubCommand::with_name("day4"))
        .subcommand(SubCommand::with_name("day5"))
        .get_matches();

    if matches.subcommand_matches("day1").is_some() {
        day1::run();
    }
    if matches.subcommand_matches("day2").is_some() {
        day2::run();
    }
    if matches.subcommand_matches("day3").is_some() {
        day3::run();
    }
    if matches.subcommand_matches("day4").is_some() {
        day4::run();
    }
    if matches.subcommand_matches("day5").is_some() {
        day5::run();
    }
}
