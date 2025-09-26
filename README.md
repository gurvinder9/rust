# Rust Learning Project

This project is set up for learning Rust programming language.

## Project Structure

- `src/main.rs` - Main Cargo project entry point
- `basic/` - Directory for basic Rust examples
- `Cargo.toml` - Project configuration and dependencies

## Getting Started

### Running the main project
```bash
cargo run
```

### Building the project
```bash
cargo build
```

### Running basic examples
```bash
# Navigate to basic directory
cd basic

# Compile and run a basic example
rustc hello.rs
./hello
```

### Useful Cargo commands
- `cargo check` - Check if code compiles without building
- `cargo build --release` - Build optimized release version
- `cargo test` - Run tests
- `cargo doc --open` - Generate and open documentation

### Pre-commit Hooks 🚀
This project includes pre-commit hooks that automatically:
- Format your code with `rustfmt`
- Lint your code with `clippy`
- Check compilation before commits
- Fix common formatting issues

```bash
# Run hooks manually
pre-commit run --all-files

# Hooks run automatically on every commit
git commit -m "your message"
```

See `PRE_COMMIT_GUIDE.md` for detailed information.

## Resources for Learning Rust

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)

Happy coding! 🦀
