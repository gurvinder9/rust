# Rust Documentation with Triple Slash Comments (`///`)

## What are Triple Slash Comments?

In Rust, triple slash comments (`///`) are special documentation comments that are used to generate API documentation. These comments are different from regular comments (`//`) because they are processed by Rust's documentation generator (`rustdoc`) to create HTML documentation.

## Types of Documentation Comments

### 1. Outer Documentation Comments (`///`)
Used to document the item that follows the comment:

```rust
/// This function adds two numbers together
///
/// # Arguments
///
/// * `a` - The first number to add
/// * `b` - The second number to add
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Returns
///
/// The sum of `a` and `b`
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 2. Inner Documentation Comments (`//!`)
Used to document the enclosing item (usually modules or crates):

```rust
//! This is a math utilities module
//!
//! It provides basic mathematical operations like addition,
//! subtraction, multiplication, and division.

/// Adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Common Documentation Sections

### Standard Sections
- **Arguments/Parameters**: Describe function parameters
- **Returns**: Describe what the function returns
- **Examples**: Show how to use the code
- **Panics**: Describe when the function might panic
- **Errors**: Describe possible error conditions
- **Safety**: For unsafe functions, describe safety requirements

### Example with All Sections:

```rust
/// Divides two numbers
///
/// # Arguments
///
/// * `dividend` - The number to be divided
/// * `divisor` - The number to divide by
///
/// # Returns
///
/// Returns `Ok(result)` if division is successful, or `Err(String)` if divisor is zero
///
/// # Examples
///
/// ```
/// let result = divide(10, 2).unwrap();
/// assert_eq!(result, 5);
///
/// let error = divide(10, 0);
/// assert!(error.is_err());
/// ```
///
/// # Errors
///
/// This function will return an error if `divisor` is zero
///
/// # Panics
///
/// This function does not panic under normal circumstances
fn divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(dividend / divisor)
    }
}
```

## How to Generate Documentation

### 1. Generate Documentation for Your Crate
```bash
cargo doc
```

This command:
- Processes all `///` and `//!` comments
- Generates HTML documentation
- Places output in `target/doc/` directory

### 2. Generate and Open Documentation
```bash
cargo doc --open
```

This will generate the documentation and automatically open it in your default web browser.

### 3. Include Dependencies' Documentation
```bash
cargo doc --no-deps
```

Use `--no-deps` to exclude dependency documentation (faster generation).

### 4. Generate Documentation with Examples
```bash
cargo doc --document-private-items
```

Include documentation for private items as well.

## Markdown Support

Rust documentation comments support Markdown formatting:

```rust
/// # This is a heading
///
/// This function does **important** work with *emphasis*.
///
/// ## Code blocks
///
/// ```rust
/// let x = example_function();
/// println!("Result: {}", x);
/// ```
///
/// ## Lists
///
/// - Item 1
/// - Item 2
/// - Item 3
///
/// ## Links
///
/// See also: [`other_function`] or [external link](https://example.com)
fn example_function() -> i32 {
    42
}
```

## Testing Documentation Examples

Rust can test code examples in documentation comments:

```bash
cargo test --doc
```

This runs all code blocks marked with `rust` in your documentation as tests.

## Best Practices

1. **Write documentation for public APIs**: Always document public functions, structs, and modules
2. **Include examples**: Examples are the most helpful part of documentation
3. **Use consistent formatting**: Follow the standard sections format
4. **Link related items**: Use `[`item_name`]` to link to other documented items
5. **Keep it concise but complete**: Include all necessary information without being verbose

## Example: Complete Module Documentation

```rust
//! # Math Utilities
//!
//! This module provides basic mathematical operations with error handling.
//!
//! # Examples
//!
//! ```
//! use math_utils::Calculator;
//!
//! let calc = Calculator::new();
//! let result = calc.add(5, 3);
//! assert_eq!(result, 8);
//! ```

/// A simple calculator struct
///
/// # Examples
///
/// ```
/// let calc = Calculator::new();
/// ```
pub struct Calculator;

impl Calculator {
    /// Creates a new calculator instance
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// ```
    pub fn new() -> Self {
        Calculator
    }

    /// Adds two numbers together
    ///
    /// # Arguments
    ///
    /// * `a` - First number
    /// * `b` - Second number
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new();
    /// assert_eq!(calc.add(2, 3), 5);
    /// ```
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
```

## Commands Summary

| Command | Description |
|---------|-------------|
| `cargo doc` | Generate documentation |
| `cargo doc --open` | Generate and open documentation |
| `cargo doc --no-deps` | Generate without dependencies |
| `cargo test --doc` | Test documentation examples |
| `rustdoc src/lib.rs` | Generate docs for specific file |

Documentation is a crucial part of Rust development, and triple slash comments make it easy to create comprehensive, testable documentation that stays in sync with your code.
