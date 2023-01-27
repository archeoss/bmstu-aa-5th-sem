use colored::Colorize;
use lab_04::dbscan::model::{Dataset, Model};
use lab_04::dbscan::para_model::ParallelModel;
use lab_04::dbscan::plotter::{plot_model_2d, plot_parallel_model_2d};
use lab_04::dbscan::utils::{input_cols, input_threads, select_file};
use std::sync::{Arc, Mutex};

fn main()
{
    // let dataset = Dataset::input_dataset("data/PAMAP2_d=4.csv".to_string(), vec![3, 4]);
    let path = select_file();
    let cols = input_cols();

    let dataset = Dataset::input_dataset(path, cols, b',');
    let mut serial_model = Model::new(Arc::new(Mutex::new(dataset.clone())));
    let mut parallel_model =
        ParallelModel::new(Arc::new(Mutex::new(dataset.clone())), input_threads());
    serial_model.input_params();
    parallel_model.set_eps(serial_model.get_eps());
    parallel_model.set_min_pts(serial_model.get_min_pts());

    serial_model.run();
    parallel_model.run();
    println!("{}", "Computations finished! Start plotting...".purple());

    plot_parallel_model_2d(parallel_model);
    plot_model_2d(serial_model);
}
