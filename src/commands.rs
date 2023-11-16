use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "ss")]
#[clap(name = "StringSeeker", about = "The all powerful String Seeker searches for text based on a set of options.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(name = "find", about = "Searches recursively under the current directory for all occurrences of the given text.")]
    Find {
        #[clap(value_parser)]
        text: String,

        #[clap(value_enum)]
        search_direction: SearchDirection,

        #[clap(value_parser, default_value_t = 2)]
        context_window_size: usize,

        #[clap(value_parser, long = "no-highlight")]
        no_highlight: bool,

        #[clap(value_parser, long = "output-file")]
        output_file: bool,

        #[clap(value_parser, long = "include-hidden-directories")]
        include_hidden_directories: bool,
    }
}

pub trait CommandTrait {
    fn execute(&self);
}

#[derive(clap::ValueEnum, Clone)]
pub enum SearchDirection {
    Under,
    UnderInclusive,
    Over,
    OverInclusive,
    In,
}