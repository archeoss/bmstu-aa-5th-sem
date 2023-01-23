use colored::Colorize;
use lab_04::dbscan::model::{Dataset, Model};
use lab_04::dbscan::para_model::ParallelModel;
use lab_04::dbscan::plotter::{plot_model_2d, plot_parallel_model_2d};
use lab_04::dbscan::utils::{input_cols, select_file};
use std::sync::{Arc, Mutex};

fn main()
{
    // let dataset = Dataset::input_dataset("data/PAMAP2_d=4.csv".to_string(), vec![3, 4]);
    let path = select_file();
    let cols = input_cols();

    let dataset = Dataset::input_dataset(path, cols, b',');
    let mut model = Model::new(Arc::new(Mutex::new(dataset)));
    // let mut model = ParallelModel::new(Arc::new(Mutex::new(dataset)), 8);
    model.input_params();
    model.run();
    println!("{}", "Computations finished! Start plotting...".purple());

    // plot_parallel_model_2d(model);
    plot_model_2d(model);
}
