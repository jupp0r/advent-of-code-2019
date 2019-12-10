extern crate clap;
extern crate termion;
use clap::{App, SubCommand};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

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
        .subcommand(SubCommand::with_name("day6"))
        .subcommand(SubCommand::with_name("day7"))
        .subcommand(SubCommand::with_name("day8"))
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
    if matches.subcommand_matches("day6").is_some() {
        day6::run();
    }
    if matches.subcommand_matches("day7").is_some() {
        day7::run();
    }
    if matches.subcommand_matches("day8").is_some() {
        day8::run();
    }
}
