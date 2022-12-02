pub use crate::dbscan::dataset::Dataset;
use colored::Colorize;
use std::sync::{Arc, Mutex};
use super::utils::Focus;

#[derive(Debug, Clone)]
pub struct Model {
    dataset: Arc<Mutex<Dataset>>,
    eps: f64,
    min_pts: usize,
    clusters: Vec<(Vec<Vec<usize>>, usize, usize)>,
    visited: Vec<bool>,
    noises: Vec<usize>,
    focus: Focus
}

impl Model {
    pub fn new(dataset: Arc<Mutex<Dataset>>) -> Model {
        let amount = dataset.lock().unwrap().records_amount();
        Model {
            eps: 0.0,
            min_pts: 0,
            clusters: vec![],
            visited: vec![false; amount],
            noises: vec![],
            dataset,
            focus: Focus::default()
        }
    }

    pub fn input_params(&mut self) {
        // TODO: eps recommended search
        println!("{}", format!("Input eps (Recommended: {}):", "--").yellow());
        let mut eps = String::new();
        std::io::stdin()
            .read_line(&mut eps)
            .expect("Can't read a line!");
        self.eps = eps
            .trim()
            .parse::<f64>()
            .expect("Input Error, not a number!");

        println!(
            "{}",
            format!(
                "Input min_pts (Recommended: {}):",
                self.dataset.lock().unwrap().labels_amount() * 2
            )
            .yellow()
        );
        let mut min_pts = String::new();
        std::io::stdin()
            .read_line(&mut min_pts)
            .expect("Can't read a line!");
        self.min_pts = min_pts
            .trim()
            .parse::<usize>()
            .expect("Input Error, not a number!");
    }

    pub fn get_clusters(&self) -> &Vec<(Vec<Vec<usize>>, usize, usize)> {
        &self.clusters
    }

    pub fn get_dataset(&self) -> Arc<Mutex<Dataset>> {
        self.dataset.clone()
    }

    pub fn get_eps(&self) -> f64 {
        self.eps
    }

    pub fn get_min_pts(&self) -> usize {
        self.min_pts
    }

    pub fn run(&mut self) {
        let len = self.dataset.lock().unwrap().labels_amount();
        for col_a in 0..len {
            for col_b in col_a + 1..len {
                self.focus = Focus(col_a, col_b);
                self.visited = vec![false; self.dataset.lock().unwrap().records_amount()];
                self.clusters.push((vec![], col_a, col_b));
                for i in 0..self.visited.len() {
                    if self.visited[i] {
                        continue;
                    }
                    self.visited[i] = true;
                    let mut neighbors = self
                        .dataset
                        .lock()
                        .unwrap()
                        .query_neighbors(i, self.eps, self.focus.clone().into());

                    if neighbors.len() < self.min_pts {
                        self.noises.push(i);
                    } else {
                        self.expand_cluster(i, &mut neighbors);
                    }
                }
            }
        }
    }

    fn expand_cluster(
        &mut self,
        index: usize,
        neighbors: &mut Vec<usize>
    ) {
        let n = self.clusters.len() - 1;
        self.clusters[n].0.push(vec![index]);
        let len = self.clusters[n].0.len() - 1;
        while !neighbors.is_empty() {
            let k = neighbors.pop().unwrap();
            if !self.visited[k] {
                self.visited[k] = true;
                let new_neighbors = self
                    .dataset
                    .lock()
                    .unwrap()
                    .query_neighbors(k, self.eps, self.focus.clone().into());
                if new_neighbors.len() >= self.min_pts {
                    neighbors.extend(new_neighbors);
                }
            }
            if self.not_in_clusters(k) {
                self.clusters[n].0[len].push(k);
            }
        }
    }

    fn not_in_clusters(&self, index: usize) -> bool {
        let mut res = true;
        let n = self.clusters.len() - 1;
        for cluster in self.clusters[n].0.iter() {
            if cluster.contains(&index) {
                res = false;
                break;
            }
        }

        res
    }
}
