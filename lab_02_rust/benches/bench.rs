use std::mem::size_of_val;
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, PlotConfiguration, AxisScale, black_box};
use lab_02::matrix::algos::*;
use lab_02::matrix::util::*;


fn matmut_even(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);

    let mut group = c.benchmark_group("Matrix multiplication (of even size)");
    group.plot_config(plot_config);
    group.sample_size(12);
    for size in (50..1150).step_by(100) {
        let (m1, m2) = (generate_matrix(size,size), generate_matrix(size,size));
        // group.throughput(Throughput::Bytes(size_of_val(&*m1) as u64 + size_of_val(&*m2) as u64));
        group.bench_with_input(BenchmarkId::new("Naive",size), &size, |b, size | {
            b.iter(|| black_box(m1.naive_mut(black_box(&m2))))
        });
        group.bench_with_input(BenchmarkId::new("Winograd",size), &size, |b, size | {
            b.iter(|| black_box(m1.winograd_mut(black_box(&m2))))
        });
        group.bench_with_input(BenchmarkId::new("Winograd Imp.",size), &size, |b, size | {
            b.iter(|| black_box(m1.winograd_mut_improved(black_box(&m2))))
        });
    }
    group.finish();
}

fn matmut_odd(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);

    let mut group = c.benchmark_group("Matrix multiplication (of odd size)");
    group.plot_config(plot_config);
    group.sample_size(12);
    for size in (51..1151).step_by(100) {
        let (m1, m2) = (generate_matrix(size,size), generate_matrix(size,size));
        // group.throughput(Throughput::Bytes(size_of_val(&*m1) as u64 + size_of_val(&*m2) as u64));
        group.bench_with_input(BenchmarkId::new("Naive",size), &size, |b, size | {
            b.iter(|| black_box(m1.naive_mut(black_box(&m2))))
        });
        group.bench_with_input(BenchmarkId::new("Winograd",size), &size, |b, size | {
            b.iter(|| black_box(m1.winograd_mut(black_box(&m2))))
        });
        group.bench_with_input(BenchmarkId::new("Winograd Imp.",size), &size, |b, size | {
            b.iter(|| black_box(m1.winograd_mut_improved(black_box(&m2))))
        });
    }
    group.finish();
}


criterion_group!(benches, matmut_odd, matmut_even);
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
