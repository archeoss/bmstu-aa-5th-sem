use csv::ReaderBuilder;
use std::fs::File;

#[derive(Default, Debug, Clone)]
pub struct Dataset {
    data: Vec<Vec<f64>>,
    labels: Vec<String>,
}

impl Dataset {
    pub fn input_dataset(filename: String, mut cols_to_read: Vec<usize>, delimiter: u8) -> Dataset {
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                panic!("Error: {}", e);
            }
        };

        let mut dataset = Dataset::default();
        let mut reader = ReaderBuilder::new().delimiter(delimiter).from_reader(file);

        println!("{}, {:?}", reader.headers().unwrap().len(), reader.headers().unwrap());
        if *cols_to_read.iter().max().unwrap() >= reader.headers().unwrap().len() {
            panic!("Maximum index column to read is greater then number of columns");
        }

        // Decide, which columns to read if needed
        cols_to_read = if cols_to_read.is_empty() {
            let mut cols: Vec<usize> = vec![];
            let record = reader.records().next().unwrap().unwrap();
            for (i, elem) in record.iter().enumerate() {
                if elem.parse::<f64>().is_ok() {
                    cols.push(i);
                }
            }
            cols.clone()
        } else {
            cols_to_read
        };

        // Read headers
        let mut j = 0;
        for head in reader.headers().unwrap() {
            if cols_to_read.contains(&j) {
                dataset.labels.push(head.to_string());
                dataset.data.push(Vec::new());
            }
            j += 1;
        }

        // Read columns
        for record in reader.records() {
            let record = record.unwrap();
            j = 0;
            if record[0] == *"" {
                continue;
            }
            for (i, elem) in record.iter().enumerate() {
                if cols_to_read.contains(&i) {
                    dataset.data[j].push(elem.parse::<f64>().unwrap());
                    j += 1;
                }
            }
        }

        dataset
    }

    pub fn get_data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<Vec<f64>> {
        &mut self.data
    }

    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    pub fn labels_amount(&self) -> usize {
        self.labels.len()
    }

    pub fn records_amount(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    pub(crate) fn query_neighbors(
        &self,
        index: usize,
        eps: f64,
        focus: (usize, usize)
    ) -> Vec<usize> {
        let mut neighbors: Vec<usize> = vec![];
        let data = self.get_data();
        for i in 0..self.records_amount() {
            let mut s = (data[focus.1][i] - data[focus.1][index]).powi(2);
            s += (data[focus.0][i] - data[focus.0][index]).powi(2);
            s = s.sqrt();
            if s <= eps {
                neighbors.push(i);
            }
        }

        neighbors
    }
}
