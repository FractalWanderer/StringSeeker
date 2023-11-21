use clap::Parser;

#[derive(Parser)]
#[command(name = "ss")]
#[clap(name = "StringSeeker", about = "The all powerful String Seeker searches for text based on a set of options.")]
pub struct Cli {

    #[clap(value_parser, help = "The text to search for.")]
    pub text: String,

    #[clap(value_enum, help = "Determines the direction of search.")]
    pub search_direction: SearchDirection,

    #[clap(value_parser, default_value_t = 2, help = "Determines the size of the context window where the text was found.")]
    pub context_window_size: usize,

    #[clap(value_parser, long = "no-highlight", help = "Disables highlighting of text. This can sometimes be useful for consoles that do not support highlighting.")]
    pub no_highlight: bool,

    #[clap(value_parser, long = "include-hidden-directories", help = "Includes hidden directories/files in the search.")]
    pub include_hidden_files: bool,
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