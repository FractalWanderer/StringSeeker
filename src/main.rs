use std::{env, fs};
use std::borrow::Cow;
use clap::Parser;
use colored::{Color, Colorize};
use indicatif::{ProgressBar, ProgressStyle};
use crate::commands::{Cli, Commands, CommandTrait, SearchDirection};
use walkdir::{DirEntry, WalkDir};

mod commands;

fn main() {
    let cli = Cli::parse();

    cli.command.execute();
}


impl CommandTrait for Commands {
    fn execute(&self) {
        match &self {
            Commands::Find { text, context_window_size, no_highlight, output_file, include_hidden_directories: include_hidden_files, search_direction } => {

                let search_results = match search_direction {
                    SearchDirection::Under => search_under(text, *context_window_size, *include_hidden_files, 2, usize::MAX),
                    SearchDirection::UnderInclusive => search_under(text, *context_window_size, *include_hidden_files, 1, usize::MAX),
                    SearchDirection::In => search_under(text, *context_window_size, *include_hidden_files, 1, 1)
                };

                print_search_results_to_console(search_results, *no_highlight);
            }
        }
    }
}

fn print_search_results_to_console(search_results: Vec<FileSearchResult>, no_highlight: bool) {

    if search_results.len() < 1 {
        println!("No results found.");
        return;
    }

    for search_result in search_results {

        if no_highlight {
            println!("Text '{}' found in file {}. Number of occurrences: {}", search_result.found_text, search_result.file_name, search_result.number_of_occurrences);
            println!("Full File Path: {}", search_result.file_path);
        } else {
            println!("Text '{}' found in file {}. Number of occurrences: {}", search_result.found_text.red(), search_result.file_name.green(), search_result.number_of_occurrences.to_string().yellow());
            println!("Full File Path: {}", search_result.file_path.green());
        }

        for window in search_result.context_windows {
            println!("\nOccurrence in file: {}, File: {}", window.occurrence_number, search_result.file_name);
            println!("------------");

            if no_highlight {
                for line in window.window_contents {
                    println!("{}", line);
                }
            } else{
                for line in window.window_contents {
                    println!("{}", highlight_text(&line, &search_result.found_text, Color::Red));
                }
            }

            println!("------------\n");
        }
    }
}

fn highlight_text(full_text: &str, text_to_highlight: &str, color: Color) -> String {

    let replacement = text_to_highlight.color(color).to_string();

    return full_text.replace(text_to_highlight, &replacement);
}

fn search_under(text: &str, context_window_size: usize, include_hidden_files: bool, min_depth: usize, max_depth: usize) -> Vec<FileSearchResult> {

    let working_directory = env::current_dir().expect("Unable to get the current working directory. Cannot proceed.");

    let mut all_search_results: Vec<FileSearchResult> = Vec::new();

    let progress_bar = create_indeterminate_progress_bar();

    for dir in progress_bar.wrap_iter(WalkDir::new(working_directory).min_depth(min_depth).max_depth(max_depth).into_iter()) {
        match dir {
            Ok(entry) => {
                set_progress_bar_message(entry.file_name().to_str().unwrap_or("Unknown file.").to_string(), &progress_bar);
                if  should_include_dir_entry(&entry, include_hidden_files){
                    if let Some(search_result) = search_directory_entry_for_text(text, context_window_size, entry) {
                        all_search_results.push(search_result);
                    }
                }
            }
            // todo Do we want to display errors/include a flag to determine if we should display errors?
            Err(_) => continue
        }
    }

    all_search_results
}

fn output_to_file(search_results: Vec<FileSearchResult>, file_path: &str) {


}

fn set_progress_bar_message(message: String, progress_bar: &ProgressBar) {
    let cow_string = Cow::Owned(message);

    progress_bar.set_message(cow_string);
}

fn should_include_dir_entry(entry: &DirEntry, include_hidden_files: bool) -> bool {
    return entry.file_type().is_file() && (!is_hidden(&entry) || include_hidden_files);
}

fn search_directory_entry_for_text(text: &str, context_window_size: usize, entry: DirEntry) -> Option<FileSearchResult> {

    let mut number_of_occurrences: u64 = 0;

    let contents: Vec<String> = fs::read_to_string(&entry.path()).unwrap_or_default().split("\n").map(|line| line.to_string()).collect();

    let mut context_windows: Vec<ContextWindow> = Vec::new();

    for (index, line) in contents.iter().enumerate() {

        if line.contains(text) {
            number_of_occurrences += 1;

            let start = if index >= context_window_size {
                index - context_window_size
            } else {
                0
            };

            let end = if index + context_window_size < contents.len() {
                index + context_window_size + 1
            } else {
                contents.len()
            };

            context_windows.push(ContextWindow {
                occurrence_number: number_of_occurrences,
                window_contents: contents[start..end].to_vec()
            });
        }
    }

    if number_of_occurrences > 0 {
        let file_name = entry.file_name().to_str().unwrap_or("Unknown file.").to_string();
        let file_path = entry.into_path().to_str().unwrap_or("Unknown file path.").to_string();

        return Some(FileSearchResult {
            found_text: text.to_string(),
            file_name,
            file_path,
            number_of_occurrences,
            context_windows
        });
    }

    None
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn create_indeterminate_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n🔎 {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] Files searched: {pos:>7}")
        .expect("Unable to set the progress bar template."));

    return pb;
}

struct FileSearchResult {
    found_text: String,
    file_name: String,
    file_path: String,
    number_of_occurrences: u64,
    context_windows: Vec<ContextWindow>
}

struct ContextWindow {
    occurrence_number: u64,
    window_contents: Vec<String>
}