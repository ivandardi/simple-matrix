use crate::Matrix;
use quickcheck::{quickcheck, Arbitrary, Gen};

const RANGE: i32 = 100000; // No over/under-flow checking for now

#[derive(Debug, Clone)]
struct AMatrix<T>(Matrix<T>);

#[derive(Debug, Clone)]
struct A2Matrix<T>(Matrix<T>, Matrix<T>);

#[derive(Debug, Clone)]
struct A3Matrix<T>(Matrix<T>, Matrix<T>, Matrix<T>);

impl Arbitrary for AMatrix<i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = g.gen_range(1, s);
        let rows = g.gen_range(1, s);

        AMatrix(Matrix::from_iter(
            rows,
            cols,
            (0..).map(|_| g.gen_range(-RANGE, RANGE)),
        ))
    }
}

impl Arbitrary for A2Matrix<i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = g.gen_range(1, s);
        let rows = g.gen_range(1, s);

        A2Matrix(
            Matrix::from_iter(rows, cols, (0..).map(|_| g.gen_range(-RANGE, RANGE))),
            Matrix::from_iter(rows, cols, (0..).map(|_| g.gen_range(-RANGE, RANGE))),
        )
    }
}

impl Arbitrary for A3Matrix<i32> {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = g.gen_range(1, s);
        let rows = g.gen_range(1, s);

        A3Matrix(
            Matrix::from_iter(rows, cols, (0..).map(|_| g.gen_range(-RANGE, RANGE))),
            Matrix::from_iter(rows, cols, (0..).map(|_| g.gen_range(-RANGE, RANGE))),
            Matrix::from_iter(rows, cols, (0..).map(|_| g.gen_range(-RANGE, RANGE))),
        )
    }
}

fn neg(m: Matrix<i32>) -> Matrix<i32> {
    let zero = Matrix::new(m.rows(), m.cols());
    zero - m
}

fn identity(length: usize) -> Matrix<i32> {
    let mut m = Matrix::new(length, length);
    for i in 0..length {
        m.set(i, i, 1);
    }
    m
}

quickcheck! {
    fn get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                if a.get(r, c).is_none() {
                    return false;
                }
            }
        }

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            if a.get(r, a.cols()).is_some() {
                return false;
            }
        }

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            if a.get(a.rows(), c).is_some() {
                return false;
            }
        }

        true
    }

    fn set(t: AMatrix<i32>, v: i32) -> bool {
        let mut a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                if !a.set(r, c, v) || a.get(r, c) != Some(&v) {
                    return false;
                }
            }
        }

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            if a.set(r, a.cols(), 0) {
                return false;
            }
        }

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            if a.set(a.rows(), c, 0) {
                return false;
            }
        }

        true
    }

    fn add(t: A3Matrix<i32>) -> bool {
        let a = &t.0;
        let b = &t.1;
        let c = &t.2;
        let zero = &Matrix::new(a.rows(), a.cols());

        (a + b == b + a)
        && (a + &(b + c) == &(a + b) + c)
        && (&(a + zero) == a)
    }

    fn sub(t: A2Matrix<i32>) -> bool {
        let a = &t.0;
        let b = &t.1;
        let zero = &Matrix::new(a.rows(), a.cols());

        (a - b == neg(b - a))
        && (&(a - a) == zero)
        && (&(a - zero) == a)
    }

    fn mul(t: AMatrix<i32>) -> bool {
        // Incomplete

        let zero = |r,c| Matrix::new(r, c);

        let a = &t.0;
        let ident1 = &identity(a.cols());

        (a * &zero(a.cols(), 2) == zero(a.rows(), 2))
        && (&(a * ident1) == a)
    }
}