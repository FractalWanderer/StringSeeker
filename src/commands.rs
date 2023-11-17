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
    #[clap(name = "find", about = "Searches for text depending on a chosen direction. Includes options for searching for hidden files, among other options.")]
    Find {
        #[clap(value_parser, help = "The text to search for.")]
        text: String,

        #[clap(value_enum, help = "Determines the direction of search.")]
        search_direction: SearchDirection,

        #[clap(value_parser, default_value_t = 2, help = "Determines the size of the context window where the text was found.")]
        context_window_size: usize,

        #[clap(value_parser, long = "no-highlight", help = "Disables highlighting of text. This can sometimes be useful for consoles that do not support highlighting.")]
        no_highlight: bool,

        #[clap(value_parser, long = "output-file", help = "Outputs the search results to a file.")]
        output_file: bool,

        #[clap(value_parser, long = "include-hidden-directories", help = "Includes hidden directories/files in the search.")]
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
    In,
}