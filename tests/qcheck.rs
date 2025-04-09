use quickcheck::{Arbitrary, Gen, quickcheck};
use simple_matrix::MatrixVec;
use simple_matrix::ops::{DotProduct, Transpose};

const RANGE: i32 = 100000; // No over/under-flow checking for now

#[derive(Debug, Clone)]
struct AMatrix<T>(MatrixVec<T>);

#[derive(Debug, Clone)]
struct A2Matrix<T>(MatrixVec<T>, MatrixVec<T>);

#[derive(Debug, Clone)]
struct A3Matrix<T>(MatrixVec<T>, MatrixVec<T>, MatrixVec<T>);

impl Arbitrary for AMatrix<i32> {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = (g.size() % s) + 1; // Ensure at least 1
        let rows = (g.size() % s) + 1; // Ensure at least 1

        AMatrix(MatrixVec::<i32>::from_iter(
            rows,
            cols,
            (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
        ))
    }
}

impl Arbitrary for A2Matrix<i32> {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = (g.size() % s) + 1; // Ensure at least 1
        let rows = (g.size() % s) + 1; // Ensure at least 1

        A2Matrix(
            MatrixVec::<i32>::from_iter(
                rows,
                cols,
                (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
            ),
            MatrixVec::<i32>::from_iter(
                rows,
                cols,
                (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
            ),
        )
    }
}

impl Arbitrary for A3Matrix<i32> {
    fn arbitrary(g: &mut Gen) -> Self {
        let s = std::cmp::max(1, g.size()); // rows & cols != 0

        let cols = (g.size() % s) + 1; // Ensure at least 1
        let rows = (g.size() % s) + 1; // Ensure at least 1

        A3Matrix(
            MatrixVec::<i32>::from_iter(
                rows,
                cols,
                (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
            ),
            MatrixVec::<i32>::from_iter(
                rows,
                cols,
                (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
            ),
            MatrixVec::<i32>::from_iter(
                rows,
                cols,
                (0..rows * cols).map(|_| i32::arbitrary(g) % (2 * RANGE) - RANGE),
            ),
        )
    }
}

fn neg(m: MatrixVec<i32>) -> MatrixVec<i32> {
    let zero = MatrixVec::<i32>::new(m.rows(), m.cols());
    &zero - &m
}

fn identity(length: usize) -> MatrixVec<i32> {
    MatrixVec::<i32>::identity(length, 1)
}

quickcheck! {
    fn qcheck_get(t: AMatrix<i32>) -> bool {
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

    fn qcheck_set(t: AMatrix<i32>, v: i32) -> bool {
        let mut a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                if a.set(r, c, v).is_none() || a.get(r, c) != Some(&v) {
                    return false;
                }
            }
        }

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            if a.set(r, a.cols(), 0).is_some() {
                return false;
            }
        }

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            if a.set(a.rows(), c, 0).is_some() {
                return false;
            }
        }

        true
    }

    fn qcheck_index_tuple_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                let _ = a[(r, c)];
            }
        }

        true
    }

    #[should_panic]
    fn qcheck_index_tuple_oob_col_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            let _ = a[(r, a.cols())];
        }

        true
    }

    #[should_panic]
    fn qcheck_index_tuple_oob_row_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            let _ = a[(a.rows(), c)];
        }

        true
    }

    fn qcheck_index_tuple_set(t: AMatrix<i32>, v: i32) -> bool {
        let mut a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                a[(r, c)] = v;
            }
        }

        true
    }

    #[should_panic]
    fn qcheck_index_tuple_oob_col_set(t: AMatrix<i32>) -> bool {
        let mut a = t.0;
        let c = a.cols();

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            a[(r, c)] = 0;
        }

        true
    }

    #[should_panic]
    fn qcheck_index_tuple_oob_row_set(t: AMatrix<i32>) -> bool {
        let mut a = t.0;
        let r = a.rows();

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            a[(r, c)] = 0;
        }

        true
    }

    fn qcheck_index_array_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                let _ = a[[r, c]];
            }
        }

        true
    }

    #[should_panic]
    fn qcheck_index_array_oob_col_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            let _ = a[[r, a.cols()]];
        }

        true
    }

    #[should_panic]
    fn qcheck_index_array_oob_row_get(t: AMatrix<i32>) -> bool {
        let a = t.0;

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            let _ = a[[a.rows(), c]];
        }

        true
    }

    fn qcheck_index_array_set(t: AMatrix<i32>, v: i32) -> bool {
        let mut a = t.0;

        // In-bounds
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                a[[r, c]] = v;
            }
        }

        true
    }

    #[should_panic]
    fn qcheck_index_array_oob_col_set(t: AMatrix<i32>) -> bool {
        let mut a = t.0;
        let c = a.cols();

        // Out-of-bounds (column)
        for r in 0..a.rows() {
            a[[r, c]] = 0;
        }

        true
    }

    #[should_panic]
    fn qcheck_index_array_oob_row_set(t: AMatrix<i32>) -> bool {
        let mut a = t.0;
        let r = a.rows();

        // Out-of-bounds (row)
        for c in 0..a.cols() {
            a[[r, c]] = 0;
        }

        true
    }

    fn qcheck_add(t: A3Matrix<i32>) -> bool {
        let a = t.0.clone();
        let b = t.1.clone();
        let c = t.2.clone();
        let zero = MatrixVec::<i32>::new(a.rows(), a.cols());

        (&a + &b) == (&b + &a)
        && (&a + &(&b + &c)) == (&(&a + &b) + &c)
        && (&a + &zero) == a
    }

    fn qcheck_sub(t: A2Matrix<i32>) -> bool {
        let a = t.0.clone();
        let b = t.1.clone();
        let zero = MatrixVec::<i32>::new(a.rows(), a.cols());

        // First check: a - b = -(b - a)
        let a_minus_b = &a - &b;
        let b_minus_a = &b - &a;
        let neg_b_minus_a = neg(b_minus_a);

        // Second check: a - a = 0
        let a_minus_a = &a - &a;

        // Third check: a - 0 = a
        let a_minus_zero = &a - &zero;

        a_minus_b == neg_b_minus_a
        && a_minus_a == zero
        && a_minus_zero == a
    }

    fn qcheck_mul(t: AMatrix<i32>) -> bool {
        let a = &t.0;

        // Create properly sized identity matrices that match the matrix dimensions
        let ident_cols = identity(a.cols());
        let ident_rows = identity(a.rows());

        // Create a properly sized zero matrix
        let zero_matrix = MatrixVec::<i32>::new(a.cols(), 2);
        let zero_result = MatrixVec::<i32>::new(a.rows(), 2);

        // Test that a × 0 = 0
        let a_times_zero = a.dot(&zero_matrix);

        // Test matrix multiplication properties
        a_times_zero == zero_result
        && a.dot(&ident_cols) == *a
        && ident_rows.dot(a) == *a
    }

    fn qcheck_transpose_trait(t: AMatrix<i32>) -> bool {
        let a = &t.0;

        // Test using the trait implementation
        let transposed_trait = Transpose::transpose(a);
        let transposed_method = a.transpose();

        transposed_trait == transposed_method
    }

    fn qcheck_dot_trait(t: AMatrix<i32>) -> bool {
        let a = &t.0;

        // Create an identity matrix for the dot product test
        let ident = identity(a.cols());

        // Test using the trait implementation vs the method
        let dot_trait = DotProduct::dot(a, &ident);
        let dot_method = a.dot(&ident);

        dot_trait == dot_method
    }

    fn qcheck_iter_methods(t: AMatrix<i32>) -> bool {
        let a = &t.0;

        let mut sum1 = 0;
        let mut sum2 = 0;

        // Sum using indices
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                sum1 += a[(r, c)];
            }
        }

        // Sum using iterator
        for val in a.iter() {
            sum2 += *val;
        }

        sum1 == sum2
    }

    fn qcheck_iter_mut_methods(t: AMatrix<i32>) -> bool {
        // Create new matrices instead of cloning
        let original = &t.0;
        let mut a = MatrixVec::<i32>::from_iter(original.rows(), original.cols(),
                                      (0..original.rows()*original.cols())
                                          .map(|i| {
                                              let r = i / original.cols();
                                              let c = i % original.cols();
                                              original[(r, c)]
                                          }));
        let mut b = MatrixVec::<i32>::from_iter(original.rows(), original.cols(),
                                      (0..original.rows()*original.cols())
                                          .map(|i| {
                                              let r = i / original.cols();
                                              let c = i % original.cols();
                                              original[(r, c)]
                                          }));

        // Double values with normal indexing
        for r in 0..a.rows() {
            for c in 0..a.cols() {
                a[(r, c)] *= 2;
            }
        }

        // Double values with mutable iterator
        for val in b.iter_mut() {
            *val *= 2;
        }

        a == b
    }

    fn qcheck_transpose_properties(t: AMatrix<i32>) -> bool {
        let a = &t.0;

        // Transposing twice gets back the original matrix
        let transposed_twice = a.transpose().transpose();

        // Check the mathematical property
        a.rows() == transposed_twice.rows() &&
        a.cols() == transposed_twice.cols() &&
        (0..a.rows()).all(|r| (0..a.cols()).all(|c| a[(r, c)] == transposed_twice[(r, c)]))
    }
}
