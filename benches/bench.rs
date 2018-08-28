use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_rs::Matrix;
use rand::distributions::{Distribution, Standard};
use rand::prelude::random;

fn randomize<T>(m: &mut Matrix<T>)
where
    Standard: Distribution<T>,
{
    m.apply_mut(|v| *v = random());
}

fn matrix_small() -> Matrix<i32> {
    let mut m = Matrix::new(3, 3);
    randomize(&mut m);
    m
}

fn matrix_large() -> Matrix<i32> {
    let mut m = Matrix::new(100, 100);
    randomize(&mut m);
    m
}

fn bench_get_row_small(c: &mut Criterion) {
    let m = matrix_small();
    c.bench_function("get_row_small", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                black_box(m.get_row(r));
            }
        })
    });
}

fn bench_get_row_large(c: &mut Criterion) {
    let m = matrix_large();
    c.bench_function("get_row_large", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                black_box(m.get_row(r));
            }
        })
    });
}

fn bench_get_col_small(c: &mut Criterion) {
    let m = matrix_small();
    c.bench_function("get_col_small", move |b| {
        b.iter(|| {
            for c in 0..m.cols() {
                black_box(m.get_col(c));
            }
        })
    });
}

fn bench_get_col_large(c: &mut Criterion) {
    let m = matrix_large();
    c.bench_function("get_col_large", move |b| {
        b.iter(|| {
            for c in 0..m.cols() {
                black_box(m.get_col(c));
            }
        })
    });
}

criterion_group!(
    bench_iter,
    bench_get_row_small,
    bench_get_row_large,
    bench_get_col_small,
    bench_get_col_large
);

criterion_main!(bench_iter);
