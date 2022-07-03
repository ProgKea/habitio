use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct HabitioArgs {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a new habit to track
    Add(AddCommand),
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The name of the habit you want to track
    pub name: String,

    /// Description of the habit you want to track
    pub description: String,
}
