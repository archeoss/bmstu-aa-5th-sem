use lab_04::dbscan::model::{Dataset, Model};
use lab_04::dbscan::plotter::plot_model_2d;
use lab_04::dbscan::utils::{input_cols, select_file};

fn main() {
    // let dataset = Dataset::input_dataset("data/PAMAP2_d=4.csv".to_string(), vec![3, 4]);
    let path = select_file();
    let cols = input_cols();

    let dataset = Dataset::input_dataset(path, cols, b',');
    let mut model = Model::new(dataset);

    model.input_params();
    model.run();

    plot_model_2d(model);
}
