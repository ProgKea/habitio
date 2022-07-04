extern crate clap;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;
use std::io::{Read, Write};

fn habit_exists(name: &str) -> bool {
    let mut file = fs::File::open("~/code/habitio/src/habits").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    for habit in content.split("\n") {
        if !habit.is_empty() {
            let habit_name = habit.split_whitespace().nth(1).unwrap();
            if &habit_name == &name {
                return true;
            }
        }
    }

    return false;
}

// TODO: error if habit exists (dont do if habit doesnt exists)
fn add(name: &str, desc: &str) {
    let name = name.trim();
    let desc = desc.trim();

    if !habit_exists(name) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("habits")
            .unwrap();

        write!(
            &mut file,
            "name: {} description: {} streak: 0\n",
            name.trim(),
            desc.trim()
        );
    }
}

fn remove(name: &str) {
    todo!()
}

fn list() {
    let mut file = fs::File::open("~/code/habitio/src/habits").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    for habit in content.split("\n") {
        if !habit.is_empty() {
	    println!("{}", &habit);
        }
    }
}

fn match_subcommands(arg_matches: ArgMatches) {
    if let Some(subcommand) = arg_matches.subcommand() {
        match subcommand.0 {
            "add" => add(
                subcommand.1.value_of("name").unwrap(),
                subcommand.1.value_of("description").unwrap(),
            ),
            "remove" => remove(subcommand.1.value_of("name").unwrap()),
            "list" => list(),
            _ => {}
        }
    } else {
        todo!();
    }
}

fn main() {
    let matches = App::new("Habitio")
        .about("Track your habits from the commandline")
        .subcommand(
            SubCommand::with_name("add")
                .arg(
                    Arg::with_name("name")
                        .help("The Name of the habit you want to track")
                        .required(true),
                )
                .arg(
                    Arg::with_name("description")
                        .help("Description of the habit you want to track")
                        .required(true),
                )
                .about("Add a new habit to track"),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .arg(
                    Arg::with_name("name")
                        .help("The Name of the habit you want to remove")
                        .required(true),
                )
                .about("Remove a habit"),
        )
        .subcommand(SubCommand::with_name("list").about("List habits"))
        .get_matches();

    match_subcommands(matches);
}
