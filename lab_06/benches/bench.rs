use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration,
};
use lab_06::ants::ant_solver::AntSolver;
use lab_06::ants::brute_solver::BruteSolver;
use lab_06::ants::constants;
use lab_06::ants::utils::{generate_data, read_config};
fn dbscan_bench(c: &mut Criterion)
{
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut config = read_config(constants::CONFIG_FILE);
    let mut group = c.benchmark_group("RIVERS");
    group.plot_config(plot_config);
    // group.sample_size(25);
    for size in constants::TIME_FROM..=constants::TIME_TO {
        let data = generate_data(
            size,
            constants::VALS_FROM,
            constants::VALS_TO,
            constants::RIVER_FROM,
            constants::RIVER_TO,
        );
        config.m = data.len();
        config.start = 0;
        config.end = config.m - 1;
        let brute = BruteSolver::new(&data, config.start, config.end);

        let ant = AntSolver::new(&data, config.clone());
        group.bench_with_input(BenchmarkId::new("Ant", size), &size, |b, _size| {
            b.iter(|| {
                black_box(ant.solve());
            });
        });
        group.bench_with_input(BenchmarkId::new("Brute", size), &size, |b, _size| {
            b.iter(|| {
                black_box(brute.solve());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, dbscan_bench);
criterion_main!(benches);
