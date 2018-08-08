# Matrix-rs: A generic matrix library in Rust

## Who, What & Why?
- *Who?*
	- I am a French student that is interested in programming (and in Rust since a couple months).
- *What?*
	- It is a matrix library in Rust that has no intention to be *the best*.
- *Why?*
	- To be better in Rust and discover some of its numerous aspects.

### Disclaimer
This has no intention to be a somewhat *professional library*, so expect:
- Bad commit messages & (non-existent?) changelog
- Breaking-changes in the API
- Not *top-of-the-line* optimization techniques
- No guaranted regularity

If you are still interested, feel free to continue!

## Usage
This crate **is not** on crates.io and must be copied to local storage.  
Then, link it in your project's `Cargo.toml` file:
```toml
# Example Cargo.toml (replace values with your own)

[dependencies]
matrix-rs = { path = "path/to/matrix-rs" }
```

### Example: Basic matrix usage
```rust
// Create a matrix of default cells
let zero: Matrix<u32> = Matrix::new(3, 3);

// Create a 2x4 matrix from an iterator (fill it row by row)
let mat1: Matrix<u32> = Matrix::from_iter(2, 4, 0..);

// Clone a matrix
let mat2 = mat1.clone();


// Add by reference (do not consume them)
let mut add = &mat1 + &mat2;

// Subtract by value (consume them)
let mut sub = mat1 - mat2;

// OpAssign are also available
sub += &zero;
sub -= zero;


// Get cells
let val: &u32 = add.get(0, 3).unwrap();

// Set cells
add.set(0, 3, 0);

// Iterate through the matrix (row by row)
for val in add {
    print!("{} ", val);
}
```

### Example: Dot product
```rust
let mat: Matrix<usize> = Matrix::from_iter(2, 4, 0..);

// Construct the transposed matrix
let mat_t = mat.transpose();

// Construct the dot product
let dot = mat * mat_t;
```

### Features
- *Features are extensions of the library left to opt-in by the user.*
- *They can increase compilation time and/or library size.*

To include a feature, add it to your `Cargo.toml` file:
```toml
# Example Cargo.toml with added feature (replace values with your own)

[dependencies]
matrix-rs = { path = "path/to/matrix-rs", features = ["impl_from"] }
```

Current available features are listed below with a little description:
- *impl_from*: Implements the *From* Trait for basic numeric types

### Tests
- Run `cargo test` in the root of the project
- Documentation tests are disabled because rustdoc does not seem to work with edition 2018

## Want to participate?
Create a new issue or a pull request and I will look at them (as soon as I can).
