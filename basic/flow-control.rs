// Topic: Flow control using if..else if..else
// Concept: Multi-way conditional branching with if/else if/else chains and match expressions vs if/else comparison
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let msg = 5;
    if msg >5 {
        println!("Message is greater than 5");
    } else if msg < 5 {
        println!("Message is less than 5");
    } else {
        println!("Message is equal to 5");
    }

    let bool_val = true;
    // Match vs If/Else in Rust:
    //
    // Match:
    // - Pattern matching: can destructure and bind values from complex data types
    // - Exhaustive: compiler ensures all possible cases are handled
    // - More powerful: can match on types, ranges, guards, and complex patterns
    // - Evaluates to a value (expression-based)
    // - Better for enums and complex data matching
    //
    // If/Else:
    // - Boolean-based: only works with true/false conditions
    // - Non-exhaustive: compiler doesn't enforce handling all cases
    // - Simpler syntax for basic conditional logic
    // - Also evaluates to a value in Rust
    // - Better for simple true/false decisions
    //
    // Example: match can destructure Option<T> and bind the inner value,
    // while if/else would require separate unwrapping steps.
    match bool_val {
        true => println!("bool_val is true"),
        false => println!("bool_val is false"),
        _ => println!("Something Else"),
    }

    let is_testing = false;
    match is_testing {
        true => println!("Is Testing"),
        false => println!("Not Testing")

    }

    let num:i32 = 1;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => println!("Something else")
    }
}

