use colored::Colorize;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Default)]
pub struct Focus(pub usize, pub usize);
impl From<Focus> for (usize, usize)
{
    fn from(e: Focus) -> (usize, usize)
    {
        let Focus { 0: a, 1: b } = e;
        (a, b)
    }
}

pub fn input_cols() -> Vec<usize>
{
    println!(
        "{}",
        "Select columns to parse (print with whitespaces as delimiter)".yellow()
    );
    let reader = io::stdin();

    reader
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn input_threads() -> usize
{
    println!("{}", "Input Number of threads".yellow());
    let mut threads_count = String::new();
    let _ = io::stdin()
        .read_line(&mut threads_count)
        .expect("Can't read a line!");
    threads_count
        .trim()
        .parse::<usize>()
        .expect("Input Error, not a number!")
}

pub fn select_file() -> String
{
    println!("{}", "Select file to model".yellow());

    let path = std::env::current_dir().unwrap();
    let task = rfd::FileDialog::new().set_directory(&path).pick_file();

    let path = match task {
        Some(path) => path.to_str().unwrap().to_string(),
        None => {
            panic!();
        }
    };
    println!("{}", path);

    path
}

pub fn pairs_compare_by_x(a: (f64, f64), b: (f64, f64)) -> std::cmp::Ordering
{
    if a.0 < b.0 {
        std::cmp::Ordering::Less
    } else if a.0 > b.0 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn pairs_compare_by_y(a: (f64, f64), b: (f64, f64)) -> std::cmp::Ordering
{
    if a.1 < b.1 {
        std::cmp::Ordering::Less
    } else if a.1 > b.1 {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}
