#![feature(custom_test_frameworks)]
use clap::{Args as _, Command};

mod model;
mod challenges;
use model::args::Args;

fn main() {
    let command = Command::new("lil");
    let command: Command = Args::augment_args(command);

    let matches = command.get_matches();
    if let Ok(c) = matches.value_of_t::<String>("name") {
        println!("Found name: {c}");
    }
}
