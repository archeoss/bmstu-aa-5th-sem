use lab_04::dbscan::util::{Dataset, Model};

fn main() {
    let dataset = Dataset::input_dataset("data/tsne_scores.csv".to_string());
    // println!("{:?}", dataset);
    let mut model = Model::new(dataset);
    model.input_params();
    model.run();
}
