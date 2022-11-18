use std::io::{Write};
use colored::Colorize;
use rand::Rng;
use std::fs::File;
use csv::Reader;

#[derive(Default, Debug)]
pub struct Dataset {
    data: Vec<Vec<f64>>,
    labels: Vec<String>,
}

pub struct Model {
    dataset: Dataset,
    eps: f64,
    min_pts: usize,
    clusters: Vec<f64>,
}

impl Dataset {
    pub fn input_dataset(filename: String) -> Dataset {
        let file = match File::open(filename) {
            Ok(f) => {
                f
            }
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        let mut dataset = Dataset::default();
        let mut reader = Reader::from_reader(file);
        for head in reader.headers().unwrap() {
            dataset.labels.push(head.to_string());
            dataset.data.push(Vec::new());
        }

        for record in reader.records() {
            let record  = record.unwrap();
            for (i, elem) in record.iter().enumerate() {
                dataset.data[i].push(elem.parse::<f64>().unwrap())
            }
        }

        dataset
    }

    pub fn get_data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn labels_amount(&self) -> usize {
        self.labels.len()
    }
    
    pub fn records_amount(&self) -> usize {
        self.data[0].len()
    }
}

impl Model {
    pub fn new(dataset: Dataset) -> Model {
        Model {
            dataset,
            eps: 0.0,
            min_pts: 0,
            clusters: Vec::new()
        }
    }

    pub fn input_params(&mut self) {
        // TODO: eps recommended search
        println!("{}", format!("Input eps (Recommended: {}):", 2.5).yellow());
        let mut eps = String::new();
        std::io::stdin().read_line(&mut eps).expect("Can't read a line!");
        self.eps = eps.trim().parse::<f64>().expect("Input Error, not a number!");

        println!("{}", format!("Input min_pts (Recommended: {}):", self.dataset.labels_amount() / 2).yellow());
        let mut min_pts = String::new();
        std::io::stdin().read_line(&mut min_pts).expect("Can't read a line!");
        self.min_pts = min_pts.trim().parse::<usize>().expect("Input Error, not a number!");
    }

    pub fn get_dataset(&self) -> &Dataset {
        &self.dataset
    }

    pub fn get_eps(&self) -> f64 {
        self.eps
    }

    pub fn get_min_pts(&self) -> usize {
        self.min_pts
    }

    pub fn run(&mut self) {
        let mut visited = vec![false; self.dataset.records_amount()];
    }
}