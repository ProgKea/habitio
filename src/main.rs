extern crate clap;

const FILE_PATH: &str = "/home/keanu/code/habitio/src/habits";

use clap::{App, AppSettings, Arg, ArgMatches, Error, SubCommand};
use std::process::exit;
use std::{
    fs,
    io::{Read, Write},
};

fn habit_exists(name: &str) -> bool {
    let mut file = fs::File::open(FILE_PATH).unwrap();
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
    for char in name.chars() {
        if char.is_numeric() {
            eprintln!("The name can't contain numbers");
            exit(1);
        }
    }
    for char in desc.chars() {
        if char.is_numeric() {
            eprintln!("The description can't contain numbers");
            exit(1);
        }
    }

    let name = &name.trim().replace(" ", "_");
    let desc = &desc.trim().replace(" ", "_");

    if !habit_exists(name) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("habits")
            .unwrap();

        let _ = write!(
            &mut file,
            "name: {} description: {} streak: 0\n",
            name, desc
        );
    }
}

fn remove(name: &str) {
    let name = name.trim().replace(" ", "_");

    if !habit_exists(&name) {
        eprintln!("The habit doesn't exist");
        exit(1);
    }

    let content = include_str!("/home/keanu/code/habitio/src/habits");
    let mut new_content = String::new();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    for habit in content.split("\n") {
        if !habit.is_empty() {
            let habit_name = habit.split_whitespace().nth(1).unwrap().trim();
            if name != habit_name {
                new_content.push_str(habit);
                new_content.push('\n');
            }
        }
    }

    let _ = write!(file, "{}", new_content);
}

fn list() {
    let mut file = fs::File::open(FILE_PATH).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    for habit in content.split("\n") {
        if !habit.is_empty() {
            println!("{}", &habit);
        }
    }
}

fn streak(name: &str) {
    let name = name.trim().replace(" ", "_");

    if !habit_exists(&name) {
        eprintln!("The habit doesn't exist");
        exit(1);
    }

    let content = include_str!("/home/keanu/code/habitio/src/habits");
    let mut new_content = String::new();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(FILE_PATH)
        .unwrap();

    for habit in content.split("\n") {
        if !habit.is_empty() {
            let habit_name = habit.split_whitespace().nth(1).unwrap().trim();
            if &name == habit_name {
                let habit = habit
                    .chars()
                    .filter(|x| !x.is_numeric())
                    .collect::<String>()
                    + &habit
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .unwrap()
                        .checked_add(1)
                        .unwrap()
                        .to_string()
                    + "\n";

                new_content.push_str(&habit);
            } else {
                new_content.push_str(habit);
                new_content.push('\n');
            }
        }
    }

    let _ = write!(file, "{}", new_content);
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
            "streak" => streak(subcommand.1.value_of("name").unwrap()),
            _ => {}
        }
    }
}

fn main() {
    let matches = App::new("Habitio")
	.about("Track your habits from the commandline")
	.subcommand(
	    SubCommand::with_name("add")
		.arg(
		    Arg::with_name("name")
			.help("The Name of the habit you want to track (all whitespace will be replaced with underscore)")
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
	.subcommand(SubCommand::with_name("list").about("List all habits"))
	.subcommand(
	    SubCommand::with_name("streak")
		.about("Increase the streak count of a habit")
		.arg(
		    Arg::with_name("name")
			.help("Name of the habit where you want to increase the streak")
			.required(true),
		),
	)
	.setting(AppSettings::ArgRequiredElseHelp)
	.get_matches();

    match_subcommands(matches);
}
