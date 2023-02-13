use super::dataset::Dataset;
use super::para_model::ParallelModel;
use super::utils::Focus;
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
pub const NUMBER_OF_MEASURMENTS: usize = 6;
pub const T1_START: usize = 0;
pub const T1_END: usize = 1;
pub const T2_START: usize = 2;
pub const T2_END: usize = 3;
pub const T3_START: usize = 4;
pub const T3_END: usize = 5;

pub trait ThreeTask<T>
{
    fn part_a(&mut self);
    fn part_b(&mut self);
    fn part_c(&mut self);

    fn run_a(&mut self);
    fn run_b(&mut self);
    fn run_c(&mut self);

    fn result(&self) -> T;
}

#[derive(Debug, Clone)]
pub struct DBSCANTask
{
    filename: String,
    model: Option<ParallelModel>,
    eps: f64,
    min_pts: usize,
    focus: Focus,
    output_file: Option<String>,
    // result: Option<Vec<usize>>,
    times: [DateTime<Utc>; NUMBER_OF_MEASURMENTS],
}

impl DBSCANTask
{
    pub fn new(string: String, eps: f64, min_pts: usize, focus: (usize, usize)) -> Self
    {
        let current_time = Utc::now();
        Self {
            filename: string,
            model: None,
            eps,
            min_pts,
            focus: Focus(focus.0, focus.1),
            output_file: None,
            times: [current_time; NUMBER_OF_MEASURMENTS],
        }
    }

    fn input(&mut self)
    {
        self.model = Some(ParallelModel::new(
            Arc::new(Mutex::new(Dataset::input_dataset(
                self.filename.clone(),
                vec![self.focus.0, self.focus.1],
                b',',
            ))),
            2,
        ));
        self.model.as_mut().unwrap().set_eps(self.eps);
        self.model.as_mut().unwrap().set_min_pts(self.min_pts);
    }

    fn process(&mut self)
    {
        self.model.as_mut().unwrap().run();
    }

    fn save(&mut self)
    {
        let result = self.model.as_ref().unwrap().get_clusters();
        let filename_save = format!(
            "./results/clusters_of_{}_{}_{}.csv",
            self.eps, self.min_pts, self.times[0]
        );
        let mut file = File::create(filename_save.clone()).unwrap();
        let strings: Vec<String> = result.iter().map(|n| n.to_string()).collect();
        file.write_all(strings.join(", ").as_bytes()).unwrap();
        self.output_file = Some(filename_save);
    }

    pub fn run_all(&mut self)
    {
        self.input();
        self.process();
        self.save();
    }

    pub fn reset(&mut self)
    {
        self.model.as_mut().unwrap().reset();
    }
}

#[derive(Debug)]
pub struct DBSCANTaskResult
{
    pub result: Vec<i32>,
    pub times: [DateTime<Utc>; NUMBER_OF_MEASURMENTS],
}

impl ThreeTask<DBSCANTaskResult> for DBSCANTask
{
    fn part_a(&mut self)
    {
        self.times[T1_START] = Utc::now();
        self.run_a();
        self.times[T1_END] = Utc::now();
    }

    fn part_b(&mut self)
    {
        self.times[T2_START] = Utc::now();
        self.run_b();
        self.times[T2_END] = Utc::now();
    }

    fn part_c(&mut self)
    {
        self.times[T3_START] = Utc::now();
        self.run_c();
        self.times[T3_END] = Utc::now();
    }

    fn run_a(&mut self)
    {
        self.input();
    }

    fn run_b(&mut self)
    {
        self.process();
    }

    fn run_c(&mut self)
    {
        self.save();
    }

    fn result(&self) -> DBSCANTaskResult
    {
        DBSCANTaskResult {
            result: self.model.as_ref().unwrap().get_clusters().clone(),
            times: self.times.clone(),
        }
    }
}
