use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rand::distributions::{Distribution, Standard};
use rand::prelude::random;
use simple_matrix::ops::{DotProduct, Transpose};
use simple_matrix::{MatrixArray, MatrixVec};

// Randomize a vector-based matrix
fn randomize<T>(m: &mut MatrixVec<T>)
where
    Standard: Distribution<T>,
    T: Default,
    T: Copy,
{
    for val in m.iter_mut() {
        *val = random();
    }
}

// Randomize an array-based matrix
fn randomize_array<T, const R: usize, const C: usize>(m: &mut MatrixArray<T, R, C>)
where
    Standard: Distribution<T>,
    T: Default,
    T: Copy,
{
    for val in m.iter_mut() {
        *val = random();
    }
}

// Create small vector matrix
fn matrix_small() -> MatrixVec<i32> {
    let mut m = MatrixVec::new(3, 3);
    randomize(&mut m);
    m
}

// Create large vector matrix
fn matrix_large() -> MatrixVec<i32> {
    let mut m = MatrixVec::new(100, 100);
    randomize(&mut m);
    m
}

// Create small array matrix (fixed size of 3x3)
fn array_matrix_small() -> MatrixArray<i32, 3, 3> {
    let mut m = MatrixArray::<i32, 3, 3>::new();
    randomize_array(&mut m);
    m
}

// Create large array matrix (fixed size of 100x100)
fn array_matrix_large() -> MatrixArray<i32, 100, 100> {
    let mut m = MatrixArray::<i32, 100, 100>::new();
    randomize_array(&mut m);
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

fn bench_vec_get_element(c: &mut Criterion) {
    let m = matrix_small();
    c.bench_function("vec_get_element", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                for c in 0..m.cols() {
                    black_box(m[[r, c]]);
                }
            }
        })
    });
}

fn bench_array_get_element(c: &mut Criterion) {
    let m = array_matrix_small();
    c.bench_function("array_get_element", move |b| {
        b.iter(|| {
            for r in 0..m.rows() {
                for c in 0..m.cols() {
                    black_box(m[[r, c]]);
                }
            }
        })
    });
}

fn bench_vec_transpose_small(c: &mut Criterion) {
    let m = matrix_small();
    c.bench_function("vec_transpose_small", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_array_transpose_small(c: &mut Criterion) {
    let m = array_matrix_small();
    c.bench_function("array_transpose_small", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_vec_transpose_large(c: &mut Criterion) {
    let m = matrix_large();
    c.bench_function("vec_transpose_large", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_array_transpose_large(c: &mut Criterion) {
    let m = array_matrix_large();
    c.bench_function("array_transpose_large", move |b| {
        b.iter(|| {
            black_box(m.transpose());
        })
    });
}

fn bench_vec_add_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("vec_add_small", move |b| {
        b.iter(|| {
            black_box(&m1 + &m2);
        })
    });
}

fn bench_array_add_small(c: &mut Criterion) {
    let m1 = array_matrix_small();
    let m2 = array_matrix_small();
    c.bench_function("array_add_small", move |b| {
        b.iter(|| {
            black_box(&m1 + &m2);
        })
    });
}

fn bench_vec_mul_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("vec_mul_small", move |b| {
        b.iter(|| {
            black_box(&m1 * &m2);
        })
    });
}

fn bench_array_mul_small(c: &mut Criterion) {
    let m1 = array_matrix_small();
    let m2 = array_matrix_small();
    c.bench_function("array_mul_small", move |b| {
        b.iter(|| {
            black_box(&m1 * &m2);
        })
    });
}

fn bench_vec_dot_small(c: &mut Criterion) {
    let m1 = matrix_small();
    let m2 = matrix_small();
    c.bench_function("vec_dot_small", move |b| {
        b.iter(|| {
            black_box(m1.dot(&m2));
        })
    });
}

fn bench_array_dot_small(c: &mut Criterion) {
    let m1 = array_matrix_small();
    let m2 = array_matrix_small();
    c.bench_function("array_dot_small", move |b| {
        b.iter(|| {
            black_box(m1.dot(&m2));
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

// New benchmark group for comparing storage types
criterion_group!(
    bench_storage_comparison,
    bench_vec_get_element,
    bench_array_get_element,
    bench_vec_transpose_small,
    bench_array_transpose_small,
    bench_vec_transpose_large,
    bench_array_transpose_large,
    bench_vec_add_small,
    bench_array_add_small,
    bench_vec_mul_small,
    bench_array_mul_small,
    bench_vec_dot_small,
    bench_array_dot_small,
);

criterion_main!(bench_basic, bench_std_ops, bench_storage_comparison);
