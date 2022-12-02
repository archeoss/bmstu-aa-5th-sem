use lab_04::dbscan::sort::parallel_sort;
use lab_04::dbscan::model::{Dataset, Model};
use lab_04::dbscan::plotter::{plot_parallel_model_2d, plot_model_2d};
use lab_04::dbscan::utils::{input_cols, select_file};
use std::sync::{Arc, Mutex};
use colored::Colorize;
use lab_04::dbscan::para_model::ParallelModel;

fn main() {
    // let dataset = Dataset::input_dataset("data/PAMAP2_d=4.csv".to_string(), vec![3, 4]);
    let path = select_file();
    let cols = input_cols();

    let dataset = Dataset::input_dataset(path, cols, b',');
    // let mut model = Model::new(Arc::new(Mutex::new(dataset)));
    let mut model = ParallelModel::new(Arc::new(Mutex::new(dataset)), 8);
    model.input_params();
    model.run();
    println!("{}","Computations finished! Start plotting...".purple());

    plot_parallel_model_2d(model);
    // plot_model_2d(model);
}
//
// // fn main() {
// //     // prepare vector to be shared
// //     let mut data = Vec::new();
//     for i in 0..10_000 {
//         data.push(i);
//     }
//     // make it accessible to multiple threads
//     let data = std::sync::Arc::new(data);
//     // prepare communication channel
//     let (tx, rx) = std::sync::mpsc::channel();
//     // launch the desired number of threads
//     let thread_count = 8;
//     let mut threads = Vec::new();
//     for idx in 0..thread_count {
//         // clone shared ressources in order to move them into the thread
//         let tx = tx.clone();
//         let data = data.clone();
//         threads.push(std::thread::spawn(move || {
//             // determine the part of shared data to be accessed by this thread
//             let begin = data.len() * idx / thread_count;
//             let end = data.len() * (idx + 1) / thread_count;
//             let slc = &data[begin..end];
//             // work on this slice
//             let mut sum = 0;
//             for v in slc {
//                 sum += v;
//             }
//             println!("thread {} --> {}", idx, sum);
//             // send a result to main thread
//             tx.send(sum).unwrap();
//         }));
//     }
//     // wait for threads and collect results
//     let mut result = 0;
//     for th in threads {
//         result += rx.recv().unwrap();
//         th.join().unwrap();
//     }
//     println!("done: {}", result);
// // }
//
// fn main(){
//     //test sort
//
//     let mut data = vec![2, 9, 3, 1, 5, 4, 7, 6, 8, 0];
//     println!("before sort: {:?}", data);
//     parallel_sort(&mut data, |a, b| a.cmp(&b), 4);
//     println!("after sort: {:?}", data);
// }