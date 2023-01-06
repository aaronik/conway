use clap::{Parser, Subcommand};

/// An evolutionary solver to conway's game of life, in color!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Sqlite database file
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("database.db"))]
    pub db: String,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the program in evolve mode
    Evolve {
        /// How many threads to use in evolve mode
        #[arg(short, long, default_value_t = 4)]
        threads: u32,
    },

    /// Display one of the evolved boards
    Display
}


