pub use super::dataset::Dataset;
use super::plotter::draw_cells;
use super::sort::parallel_sort;
use super::utils::{pairs_compare_by_x, pairs_compare_by_y, Focus};
use colored::Colorize;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub struct ParallelModel
{
    dataset: Arc<Mutex<Dataset>>,
    eps: f64,
    min_pts: usize,
    clusters: Vec<i32>,
    visited: Vec<bool>,
    noises: Vec<usize>,
    strips: Vec<Strip>,
    cells: Vec<Cell>,
    focus: Focus,
    threads_cnt: usize,
    pairs: Vec<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct Cell
{
    id: usize,
    amount: usize,
    point_id: usize,
    strip_id: usize,
    neighbors: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
struct Strip
{
    id: usize,
    amount: usize,
    point_id: usize,
    cells_amount: usize,
    cell_id: usize,
}

impl ParallelModel
{
    pub fn new(dataset: Arc<Mutex<Dataset>>, threads_cnt: usize) -> ParallelModel
    {
        let amount = dataset.lock().unwrap().records_amount();
        ParallelModel {
            eps: 0.0,
            min_pts: 0,
            clusters: vec![],
            visited: vec![false; amount],
            noises: vec![],
            dataset,
            strips: vec![],
            cells: vec![],
            focus: Focus::default(),
            threads_cnt,
            pairs: vec![],
        }
    }

    pub fn input_params(&mut self)
    {
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

    pub fn get_clusters(&self) -> &Vec<i32>
    {
        &self.clusters
    }

    pub fn get_dataset(&self) -> Arc<Mutex<Dataset>>
    {
        self.dataset.clone()
    }

    pub fn get_eps(&self) -> f64
    {
        self.eps
    }

    pub fn get_min_pts(&self) -> usize
    {
        self.min_pts
    }

    pub fn get_focus(&self) -> &Focus
    {
        &self.focus
    }

    pub fn get_pairs(&self) -> &Vec<(f64, f64)>
    {
        &self.pairs
    }

    pub fn get_cells(&self) -> &Vec<Cell>
    {
        &self.cells
    }

    fn make_cells(&mut self)
    {
        let binding = self.dataset.lock().unwrap();
        let max = binding.get_data()[self.focus.1]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let min = binding.get_data()[self.focus.1]
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let disty = max - min;

        let vx = &binding.get_data()[self.focus.0];
        let vy = &binding.get_data()[self.focus.1];
        // let v = vx.into_iter().zip(vy.into_iter()).collect::<Vec<(f64, f64)>>();
        self.pairs = vx
            .iter()
            .cloned()
            .zip(vy.iter().cloned())
            .collect::<Vec<_>>();

        parallel_sort(&mut self.pairs, &pairs_compare_by_x, self.threads_cnt);

        let minx = self.pairs[0].0;
        let maxx = self.pairs[self.pairs.len() - 1].0;
        let distx = maxx - minx;
        let mut cur_strip = 0;
        self.strips.push(Strip {
            id: 0,
            amount: 0,
            point_id: 0,
            cells_amount: 0,
            cell_id: 0,
        });
        let eps_len = self.eps / 2f64.sqrt();
        for i in 0..self.pairs.len() {
            if self.pairs[i].0 > (cur_strip + 1) as f64 * (eps_len) + minx {
                parallel_sort(
                    &mut self.pairs[self.strips[cur_strip].point_id..i],
                    &pairs_compare_by_y,
                    self.threads_cnt,
                );
                cur_strip += 1;
                self.strips.push(Strip {
                    id: cur_strip,
                    amount: 0,
                    point_id: i,
                    cells_amount: 0,
                    cell_id: 0,
                });
            }
            self.strips[cur_strip].amount += 1;
        }

        let mut cell_id = 0;
        for strip in &mut self.strips {
            self.cells.push(Cell {
                id: cell_id,
                amount: 0,
                point_id: strip.point_id,
                strip_id: strip.id,
                neighbors: vec![],
            });
            strip.cell_id = cell_id;
            strip.cells_amount = 1;
            for i in strip.point_id..(strip.point_id + strip.amount) {
                if self.pairs[i].1 > self.pairs[self.cells[cell_id].point_id].1 + eps_len {
                    cell_id += 1;
                    self.cells.push(Cell {
                        id: cell_id,
                        amount: 0,
                        point_id: i,
                        strip_id: strip.id,
                        neighbors: vec![],
                    });
                    strip.cells_amount += 1;
                }
                self.cells[cell_id].amount += 1;
            }
            cell_id += 1;
        }
        drop(binding);
        self.make_cell_neighbors();
    }

    fn make_cell_neighbors(&mut self)
    {
        let n = self.cells.len();
        let chunk_size = n / self.threads_cnt;
        let neighbors: Vec<Vec<usize>> = vec![vec![]; self.cells.len()];
        let neighbors: Arc<Mutex<Vec<Vec<usize>>>> = Arc::new(Mutex::new(neighbors));
        let mut self_arc = Arc::new(&self);
        let _ = crossbeam::scope(|scope| {
            for slice in (0..n).step_by(chunk_size) {
                let self_arc = self_arc.clone();
                let neighbors = neighbors.clone();
                scope.spawn(move |_| {
                    // println!("{}", slice);
                    for i in slice..std::cmp::min(slice + chunk_size, n) {
                        let ((_, yc1), (_, yc2)) = (
                            self_arc.pairs[self_arc.cells[i].point_id],
                            self_arc.pairs
                                [self_arc.cells[i].point_id + self_arc.cells[i].amount - 1],
                        );
                        let (min_strip, max_strip) = (
                            self_arc.cells[i].strip_id.saturating_sub(1),
                            std::cmp::min(
                                self_arc.cells[i].strip_id + 2,
                                self_arc.strips.len() - 1,
                            ),
                        );
                        for k in min_strip..max_strip {
                            if k == self_arc.cells[i].strip_id {
                                for n in i.saturating_sub(2)..std::cmp::min(i + 2, n) {
                                    let cell2 = &self_arc.cells[n];
                                    let ((_, y1), (_, y2)) = (
                                        self_arc.pairs[self_arc.cells[n].point_id],
                                        self_arc.pairs[self_arc.cells[n].point_id
                                            + self_arc.cells[n].amount
                                            - 1],
                                    );
                                    if (y1 - yc2).abs() < self_arc.eps
                                        || (y2 - yc1).abs() < self_arc.eps
                                        || (y1 - yc1).abs() < self_arc.eps
                                        || (y2 - yc2).abs() < self_arc.eps
                                    {
                                        neighbors.lock().unwrap()[i].push(n);
                                    }
                                }
                                continue;
                            }
                            let mut flag = false;
                            for j in 0..self_arc.strips[k].cells_amount {
                                let c = self_arc.strips[k].cell_id;
                                let ((_, y1), (_, y2)) = (
                                    self_arc.pairs[self_arc.cells[c + j].point_id],
                                    self_arc.pairs[self_arc.cells[c + j].point_id
                                        + self_arc.cells[c + j].amount
                                        - 1],
                                );
                                if (y1 - yc2).abs() < self_arc.eps
                                    || (y2 - yc1).abs() < self_arc.eps
                                    || (y1 - yc1).abs() < self_arc.eps
                                    || (y2 - yc2).abs() < self_arc.eps
                                {
                                    neighbors.lock().unwrap()[i].push(c + j);
                                    flag = true;
                                } else if flag {
                                    break;
                                }
                            }
                        }
                    }
                });
            }
        });
        let neighbors = neighbors.lock().unwrap();
        for i in 0..neighbors.len() {
            self.cells[i].neighbors = neighbors[i].clone();
            self.cells[i].neighbors.sort();
            self.cells[i].neighbors.dedup();
            // println!("{:?}", self.cells[i].neighbors);
        }
    }

    pub fn run(&mut self)
    {
        let len = self.dataset.lock().unwrap().labels_amount();
        for col_a in 0..len {
            for col_b in col_a + 1..len {
                self.focus = Focus(col_a, col_b);
                self.make_cells();
                #[cfg(debug_assertions)]
                {
                    draw_cells(self);
                }
                self.mark_core();
                self.cluster_core();
                // self.cluster_border();
            }
        }
    }

    fn mark_core(&mut self)
    {
        let mut visited = vec![false; self.pairs.len()];
        // let visited = Arc::new(Mutex::new(vec![false; self.pairs.len()]));
        let n = self.cells.len();
        let chunk_size = n / self.threads_cnt;
        let self_arc = Arc::new(&self);
        let _ = crossbeam::scope(|scope| {
            let mut visited = visited.iter_mut();
            for slice in (0..n).step_by(chunk_size) {
                let self_arc = self_arc.clone();
                // let visited = visited.clone();
                let mut visited2 = visited
                    .by_ref()
                    .take(
                        (-(self.cells[slice].point_id as i64)
                            + (self.cells[std::cmp::min(slice + chunk_size, n) - 1].point_id
                                as i64)
                            + (self.cells[std::cmp::min(slice + chunk_size, n) - 1].amount as i64))
                            as usize,
                    )
                    .collect::<Vec<_>>();
                let k = self.cells[slice].point_id;
                scope.spawn(move |_| {
                    println!("{} | {}", slice, visited2.len());
                    for i in slice..std::cmp::min(slice + chunk_size, n) {
                        let cell = &self_arc.cells[i];
                        // println!("{}", cell.amount);
                        if cell.amount >= self_arc.min_pts {
                            for i in cell.point_id..(cell.point_id + cell.amount - 1) {
                                *visited2[i - k] = true;
                            }
                        } else {
                            for i in cell.point_id..(cell.point_id + cell.amount - 1) {
                                let mut amount = cell.amount;
                                for neighbor in &cell.neighbors {
                                    amount += self_arc.range_count(self_arc.pairs[i], *neighbor);
                                }

                                if amount >= self_arc.min_pts {
                                    *visited2[i - k] = true;
                                }
                            }
                        }
                    }
                });
            }
        });
        println!("{:?}", visited);
        self.visited = visited.clone();
    }

    fn range_count(&self, (px, py): (f64, f64), neighbor: usize) -> usize
    {
        let cell = &self.cells[neighbor];
        let mut amount = 0;
        for p in cell.point_id..(cell.point_id + cell.amount) {
            let (x, y) = self.pairs[p];
            if ((x - px).powi(2) + (y - py).powi(2)).sqrt() <= self.eps {
                amount += 1;
            }
        }

        amount
    }

    fn cluster_core(&mut self)
    {
        let mut union_find = UnionFind::new(self.cells.len());
        // parallel_sort(&mut self.cells, |a, b| b.neighbors.len().cmp(&a.neighbors.len()), self.threads_cnt);
        let mut clusters: Vec<i32> = vec![-1; self.pairs.len()];
        // let mut clusters = Arc::new(Mutex::new(clusters));

        let n = self.cells.len();
        let chunk_size = n / self.threads_cnt;
        let self_arc = Arc::new(&self);
        let union_find = Arc::new(Mutex::new(union_find));
        let _ = crossbeam::scope(|scope| {
            for slice in (0..n).step_by(chunk_size) {
                let self_arc = self_arc.clone();
                let union_find = union_find.clone();
                scope.spawn(move |_| {
                    for i in slice..std::cmp::min(slice + chunk_size, n) {
                        let cell = &self_arc.cells[i];
                        if cell.amount >= self_arc.min_pts {
                            for neighbor in &cell.neighbors {
                                // if self_arc.cells[*neighbor].amount >= cell.amount {
                                // if cell.id < self_arc.cells[*neighbor].id && self_arc.cells[*cell].amount >= self_arc.min_pts && union_find.find(cell.id) != union_find.find(self_arc.cells[*neighbor].id) {
                                //     union_find.union(i, *neighbor);
                                // }
                                // continue;
                                // } else {
                                let first = union_find.lock().unwrap().find(cell.id);
                                let second = union_find
                                    .lock()
                                    .unwrap()
                                    .find(self_arc.cells[*neighbor].id);
                                if cell.id > self_arc.cells[*neighbor].id
                                    && self_arc.cells[*neighbor].amount >= self_arc.min_pts
                                    && first != second
                                {
                                    union_find.lock().unwrap().union(i, *neighbor);
                                }
                                // }
                            }
                        }
                    }
                });
            }
        });

        println!("union find done");
        println!("{:?}", union_find.lock().unwrap().array);
        // let mut union_find = Arc::new(Mutex::new(union_find));
        // let mut clusters = &mut clusters[..];
        // for slice in (0..n).step_by(chunk_size) {
        // let visited = visited.clone();
        // let clusters = clusters.clone();
        // for (i, slice) in clusters.chunks_mut(chunk_size).enumerate() {
        for (i, _) in (0..n).step_by(chunk_size).enumerate() {
            let self_arc = self_arc.clone();
            // let union_find = union_find.clone();
            // scope.spawn(move |_| {
            //     for i in slice..std::cmp::min(slice+chunk_size, n) {

            for j in i * chunk_size..std::cmp::min((i + 1) * chunk_size, n) {
                let cell = &self_arc.cells[j];
                if cell.amount >= self_arc.min_pts {
                    for p in cell.point_id..(cell.point_id + cell.amount) {
                        if self_arc.visited[p] {
                            clusters[p] = union_find.lock().unwrap().find(j) as i32;
                            // clusters[p] = union_find.find(i) as i32;
                        }
                    }
                }
            }
            // let cell = &self_arc.cells[i];
            // if cell.amount >= self_arc.min_pts {
            //     for p in cell.point_id..(cell.point_id+cell.amount) {
            //         println!("p: {}", p);
            //         let d = union_find.lock().unwrap().find(i);
            //         slice[p-chunk_size] = d as i32;
            //     }
            // }
            // println!("clustered {}", i);
            // });
        }
        // }
        // });
        println!("clusters done");
        self.clusters = clusters.to_vec();
    }

    fn cluster_border(&mut self)
    {
        let n = self.cells.len();
        let chunk_size = n / self.threads_cnt;
        // let mut clust_arc = Arc::new(Mutex::new(&self.clusters));
        // let mut self_arc = Arc::new(&self);
        // let _ = crossbeam::scope(|scope| {
        for slice in (0..n).step_by(chunk_size) {
            // let self_arc = self_arc.clone();
            // scope.spawn(move |_| {
            let cell = &self.cells[slice];
            if cell.amount < self.min_pts {
                for i in cell.point_id..(cell.point_id + cell.amount) {
                    for neighbor in &cell.neighbors {
                        for p in self.cells[*neighbor].point_id
                            ..(self.cells[*neighbor].point_id + self.cells[*neighbor].amount)
                        {
                            if self.clusters[p] != -1
                                && self.dst(self.pairs[i], self.pairs[p]) <= self.eps
                            {
                                self.clusters[i] = self.clusters[p];
                                break;
                            }
                        }
                    }
                }
            }
            // });
        }
        // });
    }

    fn dst(&self, (x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64
    {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
}

struct UnionFind
{
    array: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind
{
    fn new(n: usize) -> Self
    {
        UnionFind {
            array: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize
    {
        if self.array[x] == x {
            x
        } else {
            let root = self.find(self.array[x]);
            self.array[x] = root;
            root
        }
    }

    fn union(&mut self, x: usize, y: usize)
    {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.size[x_root] < self.size[y_root] {
            self.array[x_root] = y_root;
            self.size[y_root] += self.size[x_root];
        } else {
            self.array[y_root] = x_root;
            self.size[x_root] += self.size[y_root];
        }
    }
}

impl Cell
{
    pub fn get_amount(&self) -> usize
    {
        self.amount
    }

    pub fn get_point_id(&self) -> usize
    {
        self.point_id
    }

    pub fn get_neighbors(&self) -> &Vec<usize>
    {
        &self.neighbors
    }

    pub fn get_id(&self) -> usize
    {
        self.id
    }
}
