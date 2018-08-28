use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Standard};
use rand::prelude::random;
use simple_matrix::Matrix;

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

fn bench_transpose_small(c: &mut Criterion) {
    let m = matrix_small();
    c.bench_function("transpose_small", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_transpose_large(c: &mut Criterion) {
    let m = matrix_large();
    c.bench_function("transpose_large", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_add_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("add_small", move |b| {
        b.iter(|| {
            black_box(&m1 + &m2);
        })
    });
}

fn bench_add_large(c: &mut Criterion) {
    let m1 = matrix_large();
    let m2 = matrix_large();
    c.bench_function("add_large", move |b| {
        b.iter(|| {
            black_box(&m1 + &m2);
        })
    });
}

fn bench_sub_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("sub_small", move |b| {
        b.iter(|| {
            black_box(&m1 - &m2);
        })
    });
}

fn bench_sub_large(c: &mut Criterion) {
    let m1 = matrix_large();
    let m2 = matrix_large();
    c.bench_function("sub_large", move |b| {
        b.iter(|| {
            black_box(&m1 - &m2);
        })
    });
}

fn bench_mul_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("mul_small", move |b| {
        b.iter(|| {
            black_box(&m1 * &m2);
        })
    });
}

fn bench_mul_large(c: &mut Criterion) {
    let m1 = matrix_large();
    let m2 = matrix_large();
    c.bench_function("mul_large", move |b| {
        b.iter(|| {
            black_box(&m1 * &m2);
        })
    });
}

criterion_group!(
    bench_basic,
    bench_get_row_small,
    bench_get_row_large,
    bench_get_col_small,
    bench_get_col_large,
    bench_transpose_small,
    bench_transpose_large,
);

criterion_group!(
    bench_std_ops,
    bench_add_small,
    bench_add_large,
    bench_sub_small,
    bench_sub_large,
    bench_mul_small,
    bench_mul_large,
);

criterion_main!(bench_basic, bench_std_ops);
