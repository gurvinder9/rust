/*
CLOSURES IN RUST - Anonymous Functions That Capture Environment
==============================================================

WHAT IS A CLOSURE?
- A closure is an anonymous function (function without a name)
- It can "capture" variables from its surrounding environment
- Think of it as a function that "remembers" the context where it was created
- Syntax: |parameters| expression  or  |parameters| { statements }

REAL-WORLD ANALOGY:
- Like a "smart note" that remembers where you wrote it
- A closure is like a function that carries a "backpack" of variables
- It can access variables from the scope where it was defined

CLOSURE vs REGULAR FUNCTION:
┌─────────────────┬─────────────────────────────────────┬─────────────────────────────────────┐
│     Feature     │           Regular Function          │              Closure                │
├─────────────────┼─────────────────────────────────────┼─────────────────────────────────────┤
│ Definition      │ fn name(params) { body }            │ |params| expression                 │
│ Name            │ Must have a name                    │ Anonymous (no name)                 │
│ Environment     │ Cannot capture local variables      │ Can capture surrounding variables   │
│ Usage           │ Called by name                      │ Stored in variable, passed around   │
└─────────────────┴─────────────────────────────────────┴─────────────────────────────────────┘

WHEN TO USE CLOSURES:
✅ Iterator operations (.map(), .filter(), .fold())
✅ Event handling and callbacks
✅ Functional programming patterns
✅ Short, one-time-use functions
✅ When you need to capture local variables
✅ Higher-order functions (functions that take other functions)
✅ Customizing behavior (sorting, searching)

CLOSURE CAPTURE MODES:
1. By Reference (&T) - Borrows the variable
2. By Mutable Reference (&mut T) - Borrows mutably
3. By Value (T) - Takes ownership (use 'move' keyword)
*/

fn main() {
    println!("🚀 === CLOSURE BASICS ===");

    // EXAMPLE 1: Basic closure syntax
    let add = |a: i32, b: i32| a + b;
    println!("Basic closure: {} + {} = {}", 1, 2, add(1, 2));

    // EXAMPLE 2: Closure with type inference (Rust figures out types)
    let multiply = |x, y| x * y; // Types inferred from usage
    println!("Type inference: {} * {} = {}", 3, 4, multiply(3, 4));

    // EXAMPLE 3: Closure with block syntax (multiple statements)
    let complex_calc = |a: i32, b: i32| {
        let sum = a + b;
        let product = a * b;
        println!(
            "  Calculating: {} + {} = {}, {} * {} = {}",
            a, b, sum, a, b, product
        );
        sum + product // Return value
    };
    println!("Complex closure result: {}", complex_calc(5, 3));

    println!("\n📦 === CAPTURING ENVIRONMENT ===");

    // EXAMPLE 4: Capturing by reference (immutable borrow)
    let name = String::from("Alice");
    let age = 25;
    let greet = || {
        // No parameters, captures environment
        println!("Hello, {}! You are {} years old.", name, age);
    };
    greet();
    println!("Original variables still accessible: {} is {}", name, age);

    // EXAMPLE 5: Capturing by mutable reference
    let mut counter = 0;
    let mut increment = || {
        counter += 1; // Mutably borrows counter
        println!("Counter is now: {}", counter);
    };
    increment();
    increment();
    increment();
    println!("Final counter value: {}", counter);

    // EXAMPLE 6: Move closure (takes ownership)
    let message = String::from("Important data");
    let moved_closure = move || {
        println!("Moved closure has: {}", message);
        // message is now owned by the closure
    };
    moved_closure();
    // println!("{}", message); // ❌ This would error! message was moved

    println!("\n🔄 === CLOSURES WITH ITERATORS ===");

    // EXAMPLE 7: Using closures with iterators (most common use case)
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Filter even numbers
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0) // Closure that checks if number is even
        .cloned()
        .collect();
    println!("Even numbers: {:?}", evens);

    // Square all numbers
    let squares: Vec<i32> = numbers
        .iter()
        .map(|&x| x * x) // Closure that squares each number
        .collect();
    println!("Squared numbers: {:?}", squares);

    // Chain operations: filter odds, square them, sum the result
    let sum_of_odd_squares: i32 = numbers
        .iter()
        .filter(|&x| x % 2 == 1) // Keep odd numbers
        .map(|&x| x * x) // Square them
        .sum(); // Sum the results
    println!("Sum of odd squares: {}", sum_of_odd_squares);

    println!("\n🎯 === PRACTICAL EXAMPLES ===");

    // EXAMPLE 8: Custom sorting with closures
    let mut people = vec![("Alice", 30), ("Bob", 25), ("Charlie", 35), ("Diana", 28)];

    // Sort by age (using closure)
    people.sort_by(|a, b| a.1.cmp(&b.1)); // Compare by age (second element)
    println!("Sorted by age: {:?}", people);

    // Sort by name length
    people.sort_by(|a, b| a.0.len().cmp(&b.0.len())); // Compare by name length
    println!("Sorted by name length: {:?}", people);

    // EXAMPLE 9: Function that takes a closure as parameter
    fn apply_operation<F>(numbers: &[i32], operation: F) -> Vec<i32>
    where
        F: Fn(i32) -> i32, // F is a closure that takes i32 and returns i32
    {
        numbers.iter().map(|&x| operation(x)).collect()
    }

    let nums = vec![1, 2, 3, 4, 5];

    // Pass different closures to the same function
    let doubled = apply_operation(&nums, |x| x * 2);
    let squared = apply_operation(&nums, |x| x * x);
    let plus_ten = apply_operation(&nums, |x| x + 10);

    println!("Original: {:?}", nums);
    println!("Doubled: {:?}", doubled);
    println!("Squared: {:?}", squared);
    println!("Plus 10: {:?}", plus_ten);

    // EXAMPLE 10: Closure for configuration/customization
    println!("\n⚙️  === CONFIGURATION WITH CLOSURES ===");

    struct Calculator {
        operation: Box<dyn Fn(i32, i32) -> i32>, // Store closure in struct
    }

    impl Calculator {
        fn new<F>(op: F) -> Self
        where
            F: Fn(i32, i32) -> i32 + 'static,
        {
            Calculator {
                operation: Box::new(op),
            }
        }

        fn calculate(&self, a: i32, b: i32) -> i32 {
            (self.operation)(a, b)
        }
    }

    // Create different calculators with different operations
    let adder = Calculator::new(|a, b| a + b);
    let multiplier = Calculator::new(|a, b| a * b);
    let power = Calculator::new(|a, b| a.pow(b as u32));

    println!("Adder: 5 + 3 = {}", adder.calculate(5, 3));
    println!("Multiplier: 5 * 3 = {}", multiplier.calculate(5, 3));
    println!("Power: 5^3 = {}", power.calculate(5, 3));

    println!("\n📋 === CLOSURE SUMMARY ===");
    println!("✅ Closures are anonymous functions that can capture their environment");
    println!("✅ Syntax: |params| expression  or  |params| {{ statements }}");
    println!("✅ Perfect for: iterators, callbacks, functional programming");
    println!("✅ Can capture by reference, mutable reference, or by value (move)");
    println!("✅ Make code more concise and expressive");
    println!("✅ Enable powerful functional programming patterns");
}
