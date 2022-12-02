use std::cmp::Ordering;
use std::sync::Arc;

fn merge<T: Copy, F: Fn(T, T) -> std::cmp::Ordering + Send>(x1: &[T], x2: &[T], y: &mut [T], compare_fn: &F) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if compare_fn(x1[i], x2[j]).is_lt() {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
}

fn merge_sort<T: Copy, F: Fn(T, T) -> std::cmp::Ordering + Send>(x: &mut [T], compare_fn: &F) {
    let n = x.len();
    let m = n / 2;

    if n <= 1 {
        return;
    }

    merge_sort(&mut x[0..m], compare_fn);
    merge_sort(&mut x[m..n], compare_fn);

    let mut y: Vec<T> = x.to_vec();

    merge(&x[0..m], &x[m..n], &mut y[..], compare_fn);

    x.copy_from_slice(&y);
}

pub fn parallel_sort<T: Copy + Send, F: Copy + Fn(T, T) -> std::cmp::Ordering + Send>(data: &mut [T], compare_fn: F, threads: usize) {
    // let compare_fn = Arc::new(compare_fn);
    let chunks = std::cmp::min(data.len(), threads);
    let _ = crossbeam::scope(|scope| {
        for slice in data.chunks_mut(data.len() / chunks) {
            scope.spawn(move |_| merge_sort(slice, &compare_fn));
        }
    });

    let n = data.len();
    let parts = n / chunks;
    for slice in (parts..n).step_by(parts) {
        let mut y: Vec<T> = data[0..std::cmp::min(slice+parts, n)].to_vec();
        merge(&data[0..slice], &data[slice..std::cmp::min(slice+parts, n)], &mut y[..], &compare_fn);
        data[0..std::cmp::min(slice+parts, n)].copy_from_slice(&y);
    }
}
