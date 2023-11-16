use std::{env, fs};
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;
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
                    SearchDirection::Under => search_under(text, *context_window_size, *include_hidden_files),
                    SearchDirection::UnderInclusive => search_under_inclusive(*include_hidden_files),
                    SearchDirection::Over => search_over(*include_hidden_files),
                    SearchDirection::OverInclusive => search_over_inclusive(*include_hidden_files),
                    SearchDirection::In => search_in(*include_hidden_files)
                };

            }
        }
    }
}

fn output_search_results(search_results: Vec<FileSearchResult>, no_highlight: bool) {

    for search_result in search_results {
        if no_highlight {
            println!("\n Text '{}' found in file: {}\n", search_result.found_text, search_result.file_name)
        } else {
            println!("\nText '{}' found in file: {}\n", search_result.found_text.red(), search_result.file_name.green())
        }

        for window in search_result.context_windows {


        }
    }

}

fn search_over(include_hidden_directories: bool) -> Vec<FileSearchResult> {


    todo!();
}

fn search_over_inclusive(include_hidden_directories: bool) -> Vec<FileSearchResult> {

    todo!()
}

fn search_in(include_hidden_directories: bool) -> Vec<FileSearchResult> {

    todo!()
}

fn search_under(text: &str, context_window_size: usize, include_hidden_files: bool) -> Vec<FileSearchResult> {

    let working_directory = env::current_dir().expect("Unable to get the current working directory. Cannot proceed.");

    let mut all_search_results: Vec<FileSearchResult> = Vec::new();

    let progress_bar = create_progress_bar();

    let mut context_windows: Vec<ContextWindow> = Vec::new();

    for (index, dir) in WalkDir::new(working_directory).min_depth(1).into_iter().enumerate() {

        progress_bar.set_message("Processing...");
        progress_bar.set_position(index as u64);

        match dir {
            Ok(entry) => {
                if entry.file_type().is_file() && (!is_hidden(&entry) || include_hidden_files) {
                    let search_results = search_entry_for_text(text, context_window_size, entry);
                    match search_results {
                        None => {}
                        Some(result) => {
                            all_search_results.push(result);
                        }
                    }

                }
            }
            // todo Do we want to display errors/include a flag to determine if we should display errors?
            Err(_) => continue
        }
    }

    all_search_results
}

fn search_entry_for_text(text: &str, context_window_size: usize, entry: DirEntry) -> Option<FileSearchResult> {

    let mut number_of_occurrences: u64 = 0;

    let contents: Vec<String> = fs::read_to_string(&entry.path()).unwrap().split("\n").map(|line| line.to_string()).collect();

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

fn search_under_inclusive(include_hidden_directories: bool) -> Vec<FileSearchResult> {

    todo!()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
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