use super::args::HabitioArgs;
use clap::Parser;

pub fn run_commands() {
    let args = HabitioArgs::parse();

    match &args.commands {
	_ => println!("Hello World"),
    }
}
