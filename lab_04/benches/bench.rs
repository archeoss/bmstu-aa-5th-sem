use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use lab_04::dbscan::dataset::Dataset;
use lab_04::dbscan::model::Model;
use lab_04::dbscan::para_model::ParallelModel;
use lab_04::dbscan::utils::*;
use std::sync::{Arc, Mutex};
const EPS: f64 = 4.0;
const MIN_PTS: usize = 4;
fn dbscan_bench(c: &mut Criterion)
{
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);

    let mut group = c.benchmark_group("DBSCAN");
    group.plot_config(plot_config);
    let path = select_file();
    let cols = input_cols();

    let dataset = Dataset::input_dataset(path.clone(), cols.clone(), b',');
    group.sample_size(25);
    let mut serial_model = Model::new(Arc::new(Mutex::new(dataset.clone())));
    serial_model.set_eps(EPS);
    serial_model.set_min_pts(MIN_PTS);
    for size in 0..=6 {
        // group.throughput(Throughput::Bytes(size_of_val(&*m1) as u64 + size_of_val(&*m2) as u64));
        let size = 2usize.pow(size);
        let mut parallel_model = ParallelModel::new(Arc::new(Mutex::new(dataset.clone())), size);
        parallel_model.set_eps(EPS);
        parallel_model.set_min_pts(MIN_PTS);
        group.bench_with_input(BenchmarkId::new("Parallel", size), &size, |b, _size| {
            b.iter(|| {
                parallel_model.run();
                parallel_model.reset();
                black_box(());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, dbscan_bench);
criterion_main!(benches);
