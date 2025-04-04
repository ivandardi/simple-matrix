// An example demonstrating array-based matrix operations
use simple_matrix::ops::{DotProduct, Transpose};
use simple_matrix::{Matrix, MatrixArray};

fn main() {
    println!("Array-Based Matrix Example");
    println!("=========================");

    // Create a 2x3 array-based matrix with default values (zeros)
    let zeros = MatrixArray::<i32, 2, 3>::new();
    println!("Zeros matrix (2x3):");
    print_matrix(&zeros);

    // Create a mutable array matrix and set values manually
    let mut arr_matrix = MatrixArray::<i32, 2, 3>::new();

    // Set values manually
    arr_matrix[[0, 0]] = 1;
    arr_matrix[[0, 1]] = 2;
    arr_matrix[[0, 2]] = 3;
    arr_matrix[[1, 0]] = 4;
    arr_matrix[[1, 1]] = 5;
    arr_matrix[[1, 2]] = 6;

    println!("\nArray Matrix (2x3):");
    print_matrix(&arr_matrix);

    // Transpose the array matrix
    let transposed = arr_matrix.transpose();
    println!("\nArray Matrix Transposed (3x2):");
    print_matrix(&transposed);

    // Matrix multiplication example
    let identity_matrix = MatrixArray::<i32, 3, 3>::identity(1);
    println!("\nIdentity Matrix (3x3):");
    print_matrix(&identity_matrix);

    // Dot product with identity matrix preserves the original matrix
    let result = arr_matrix.dot(&identity_matrix);
    println!("\nArray Matrix · Identity Matrix:");
    print_matrix(&result);

    // Element-wise operations with fixed arrays
    let mut arr_matrix2 = MatrixArray::<i32, 2, 3>::new();
    arr_matrix2
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| *x = *x * i as i32);
    println!("\nArray Matrix 2:");
    print_matrix(&arr_matrix2);

    // Perform operations directly on the array matrices
    // Element-wise multiplication
    let elem_mul = &arr_matrix2 * &arr_matrix;
    println!("\nMatrix 2 * Matrix 1 (element-wise):");
    print_matrix(&elem_mul);

    // Element-wise addition
    let elem_add = &arr_matrix2 + &arr_matrix;
    println!("\nMatrix 2 + Matrix 1 (element-wise):");
    print_matrix(&elem_add);
}

// Helper function to print a matrix
fn print_matrix<T: std::fmt::Display, S>(matrix: &Matrix<T, S>)
where
    S: simple_matrix::storage::Storage<T>,
{
    for r in 0..matrix.rows() {
        print!("[ ");
        for c in 0..matrix.cols() {
            print!("{:4} ", matrix[[r, c]]);
        }
        println!("]");
    }
}
