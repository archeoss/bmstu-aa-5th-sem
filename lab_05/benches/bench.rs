use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use lab_05::dbscan::conveyor::Conveyor3;
use lab_05::dbscan::task::DBSCANTask;
use lab_05::dbscan::utils::generate_data;
fn dbscan_bench(c: &mut Criterion)
{
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("DBSCAN CHANNEL");
    group.plot_config(plot_config);
    group.sample_size(25);
    for size in (2..=102).step_by(10) {
        let mut data = generate_data(size);
        let mut lin_data = data.clone();
        group.bench_with_input(
            BenchmarkId::new("Линейный", size),
            &size,
            |b, _size| {
                b.iter(|| {
                    black_box(());
                    for task in lin_data.iter_mut() {
                        task.run_all();
                        task.reset();
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Конвеер", size),
            &size,
            |b, _size| {
                b.iter(|| {
                    black_box(());
                    let conv_parall = Conveyor3::new(data.clone());
                    while let Some(_) = conv_parall.recv() {}
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, dbscan_bench);
criterion_main!(benches);
