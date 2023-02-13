use log::{debug, info};
use std::time::Instant;

pub mod conveyor;
mod dataset;
mod model;
mod para_model;
mod plotter;
mod sort;
pub mod task;
pub mod utils;
use conveyor::Conveyor3;
use task::DBSCANTask;
use utils::show_log;

// pub fn run_tests()
// {
//     let data = utils::generate_data();
//     run(data);
// }

pub fn run_comparison()
{
    let data = utils::generate_data(utils::get_n());
    compare(data);
}

pub fn run_interactive()
{
    let data = utils::generate_data(utils::get_n());
    run(data);
}

fn compare(mut data: Vec<DBSCANTask>)
{
    let time = Instant::now();
    for task in data.iter_mut() {
        task.run_all();
    }
    println!("Linear time: {}", time.elapsed().as_nanos());
    let conv_parall = Conveyor3::new(data.clone());
    let time = Instant::now();
    while let Some(_) = conv_parall.recv() {}
    println!("Parallel time: {}", time.elapsed().as_nanos());
}

fn run(data: Vec<DBSCANTask>)
{
    let conv = Conveyor3::new(data);

    let mut i = 0;
    let mut times_arr = Vec::new();
    while let Some(res) = conv.recv() {
        i += 1;
        // show_result(&res.result, &res.data);
        times_arr.push(res.times);
    }
    println!("{}", times_arr.len());

    show_log(times_arr);

    info!("КОНЕЦ ТЕСТОВ");
}
