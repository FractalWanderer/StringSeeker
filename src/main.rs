use std::path::PathBuf;
use clap::Parser;
use crate::commands::{Cli, Commands, CommandTrait, SearchDirection};

mod commands;

fn main() {
    let cli = Cli::parse();

    cli.command.execute();
}


impl CommandTrait for Commands {
    fn execute(&self) {
        match &self {
            Commands::Find { text, context_window_size, no_highlight, output_file, search_direction } => {


            }
        }
    }
}

fn get_path_buffers(direction: SearchDirection) -> Vec<PathBuf> {

    let path_buffers = match direction {
        SearchDirection::Under => search_under(),
        SearchDirection::UnderInclusive => search_under_inclusive(),
        SearchDirection::Over => search_over(),
        SearchDirection::OverInclusive => search_over_inclusive(),
        SearchDirection::In => search_in()
    };

    path_buffers
}


fn search_over() -> Vec<PathBuf> {


    todo!();
}

fn search_over_inclusive() -> Vec<PathBuf> {

    todo!()
}

fn search_in() -> Vec<PathBuf> {

    todo!()
}

fn search_under() -> Vec<PathBuf> {

    todo!()
}

fn search_under_inclusive() -> Vec<PathBuf> {

    todo!()
}