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

        #[clap(value_parser, default_value_t = 2)]
        context_window_size: u32,

        #[clap(value_parser, long = "no-highlight")]
        no_highlight: bool,

        #[clap(value_parser, long = "output-file")]
        output_file: bool,

        #[clap(arg_enum)]
        search_direction: SearchDirection
    }
}

pub trait CommandTrait {
    fn execute(&self);
}


#[derive(Clone)]
pub enum SearchDirection {
    Under,
    UnderInclusive,
    Over,
    OverInclusive,
    In,
}