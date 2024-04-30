extern crate clap;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    re: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Map(String),
    Filter(String),
    Reduce(String),
}

fn main() {
    let args = Args::parse();
  
    match args.cmd {
        Commands::Map(value) => cmd_map(value),
        Commands::Filter(value) => cmd_filter(value),
        Commands::Reduce(value) => cmd_reduce(value),
    }
}

fn cmd_map(value: String) {
    println!("Map: {}", value);
}

fn cmd_filter(value: String) {
    println!("Filter: {}", value);
}

fn cmd_reduce(value: String) {
    println!("Reduce: {}", value);
}
