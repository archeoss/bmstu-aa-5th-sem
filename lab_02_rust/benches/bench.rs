use std::mem::size_of_val;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale};
use lab_02::matrix::algos::*;
use lab_02::matrix::util::*;


fn from_elem(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);

    let mut group = c.benchmark_group("Matrix multiplication");
    group.plot_config(plot_config);
    group.sample_size(20);
    for size in (50..1050).step_by(50) {
        let (m1, m2) = (generate_matrix(size,size), generate_matrix(size,size));
        // group.throughput(Throughput::Bytes(size_of_val(&*m1) as u64 + size_of_val(&*m2) as u64));
        group.bench_with_input(BenchmarkId::new("Naive",size), &size, |b, size | {
            b.iter(|| m1.naive_mut(&m2))
        });
        group.bench_with_input(BenchmarkId::new("Winograd",size), &size, |b, size | {
            b.iter(|| m1.winograd_mut(&m2))
        });
        group.bench_with_input(BenchmarkId::new("Winograd Imp.",size), &size, |b, size | {
            b.iter(|| m1.winograd_mut_improved(&m2))
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);

// use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

// fn bench_fibs(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Fibonacci");
//     for i in [20u64, 21u64].iter() {
//         group.bench_with_input(BenchmarkId::new("Recursive", i), i,
//                                |b, i| b.iter(|| fibonacci_slow(*i)));
//         group.bench_with_input(BenchmarkId::new("Iterative", i), i,
//                                |b, i| b.iter(|| fibonacci_fast(*i)));
//     }
//     group.finish();
// }

// criterion_group!(benches, bench_fibs);
// criterion_main!(benches);
