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
Like any *local* cargo library, link it in your `Cargo.toml` file:
```toml
# Example Cargo.toml (replace values with your own)

[dependencies]
matrix-rs = { path = "path/to/matrix-rs" }
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
