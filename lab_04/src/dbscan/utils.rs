use std::io::{self, BufRead};
use colored::Colorize;

pub fn input_cols() -> Vec<usize> {
    println!("{}", "Select columns to parse (print with whitespaces as delimiter)".yellow());
    let reader = io::stdin();

    reader.lock()
            .lines().next().unwrap().unwrap()
            .split(' ').map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
}

pub fn select_file() -> String {
    println!("{}", "Select file to model".yellow());

    let path = std::env::current_dir().unwrap();
    let task = rfd::FileDialog::new().set_directory(&path).pick_file();

    let path = match task {
        Some(path) => {path.to_str().unwrap().to_string()}
        None => {panic!();}
    };
    println!("{}", path);

    path
}