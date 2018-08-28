use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matrix_rs::Matrix;

fn bench_get_row_small(c: &mut Criterion) {
    let m: Matrix<i32> = Matrix::new(3, 3);
    c.bench_function("get_row_small", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                black_box(m.get_row(r));
            }
        })
    });
}

fn bench_get_row_large(c: &mut Criterion) {
    let m: Matrix<i32> = Matrix::new(100, 100);
    c.bench_function("get_row_large", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                black_box(m.get_row(r));
            }
        })
    });
}

fn bench_get_col_small(c: &mut Criterion) {
    let m: Matrix<i32> = Matrix::new(3, 3);
    c.bench_function("get_col_small", move |b| {
        b.iter(|| {
            for c in 0..m.cols() {
                black_box(m.get_col(c));
            }
        })
    });
}

fn bench_get_col_large(c: &mut Criterion) {
    let m: Matrix<i32> = Matrix::new(100, 100);
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
