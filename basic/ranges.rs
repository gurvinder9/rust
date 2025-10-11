/*
RANGES IN RUST - Efficient Sequence Generation
==============================================

WHAT ARE RANGES?
- Ranges represent a sequence of values between two bounds
- They are lazy iterators - generate values on demand, not stored in memory
- Extremely memory efficient - O(1) space regardless of range size
- Built into Rust's syntax with special operators

REAL-WORLD ANALOGY:
- Like a "number generator" that knows start and end points
- Instead of printing all numbers on paper, it generates them as needed
- Like a ticket dispenser - gives you the next number when you ask
- Very efficient - doesn't waste memory storing all values

RANGE SYNTAX:
1. start..end       - Exclusive end (start to end-1)
2. start..=end      - Inclusive end (start to end)
3. ..end            - From beginning to end-1
4. ..=end           - From beginning to end
5. start..          - From start to infinity (endless)
6. ..               - Full range (all values)

WHEN TO USE RANGES:
✅ Loop iteration with known bounds
✅ Generating sequences of numbers
✅ Array/vector indexing and slicing
✅ Creating test data
✅ Mathematical computations
✅ Performance-critical iteration (no memory allocation)
✅ Functional programming patterns

BENEFITS:
- Zero memory overhead for the range itself
- Lazy evaluation - compute values as needed
- Composable with iterator methods
- Type-safe bounds checking
- Readable and expressive syntax
*/

fn main() {
    println!("📏 === RUST RANGES MASTERCLASS ===");

    // ========================================================================
    // 1. BASIC RANGE SYNTAX
    // ========================================================================
    println!("\n🔢 === BASIC RANGE SYNTAX ===");

    // Exclusive range (start..end) - doesn't include end
    println!("Exclusive range 1..5:");
    for i in 1..5 {
        print!("{} ", i); // Prints: 1 2 3 4
    }
    println!("(end 5 not included)");

    // Inclusive range (start..=end) - includes end
    println!("Inclusive range 1..=5:");
    for i in 1..=5 {
        print!("{} ", i); // Prints: 1 2 3 4 5
    }
    println!("(end 5 included)");

    // Range from beginning (..end)
    println!("Range from 0 to 4 (..5):");
    for i in ..5 {
        print!("{} ", i); // Prints: 0 1 2 3 4
    }
    println!();

    // Inclusive range from beginning (..=end)
    println!("Range from 0 to 5 (..=5):");
    for i in ..=5 {
        print!("{} ", i); // Prints: 0 1 2 3 4 5
    }
    println!();

    // Open-ended range (start..) - careful with these!
    println!("Open-ended range 10.. (first 5 values):");
    for (count, i) in (10..).enumerate() {
        if count >= 5 {
            break;
        }
        print!("{} ", i); // Prints: 10 11 12 13 14
    }
    println!("(continues infinitely)");

    // ========================================================================
    // 2. RANGES WITH DIFFERENT TYPES
    // ========================================================================
    println!("\n🎯 === RANGES WITH DIFFERENT TYPES ===");

    // Character ranges
    println!("Character range 'a'..='z':");
    let lowercase: Vec<char> = ('a'..='z').collect();
    println!("First 10: {:?}", &lowercase[0..10]);
    println!("Total letters: {}", lowercase.len());

    // Uppercase letters
    let uppercase: Vec<char> = ('A'..='Z').collect();
    println!("Uppercase: {:?}", &uppercase[0..5]);

    // Floating point ranges (need step_by or custom logic)
    println!("Simulated float range 0.0 to 2.0 by 0.5:");
    let float_range: Vec<f64> = (0..=4).map(|i| i as f64 * 0.5).collect();
    println!("{:?}", float_range);

    // ========================================================================
    // 3. RANGES WITH ITERATOR METHODS
    // ========================================================================
    println!("\n🔄 === RANGES WITH ITERATOR METHODS ===");

    // Map - transform range values
    let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();
    println!("Squares 1-10: {:?}", squares);

    // Filter - keep only matching values
    let even_numbers: Vec<i32> = (1..=20).filter(|&x| x % 2 == 0).collect();
    println!("Even numbers 1-20: {:?}", even_numbers);

    // Step by - skip values
    let every_third: Vec<i32> = (0..30).step_by(3).collect();
    println!("Every 3rd number 0-30: {:?}", every_third);

    // Reverse ranges
    let countdown: Vec<i32> = (1..=10).rev().collect();
    println!("Countdown: {:?}", countdown);

    // Chain ranges
    let combined: Vec<i32> = (1..=3).chain(10..=12).chain(20..=22).collect();
    println!("Chained ranges: {:?}", combined);

    // ========================================================================
    // 4. PRACTICAL EXAMPLES - ARRAY/VECTOR OPERATIONS
    // ========================================================================
    println!("\n📊 === ARRAY/VECTOR OPERATIONS ===");

    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // Slicing with ranges
    println!("Original data: {:?}", data);
    println!("First 5 elements [..5]: {:?}", &data[..5]);
    println!("Last 5 elements [5..]: {:?}", &data[5..]);
    println!("Middle elements [2..8]: {:?}", &data[2..8]);
    println!("Elements 3 to 6 inclusive [3..=6]: {:?}", &data[3..=6]);

    // Creating indices for iteration
    println!("Indexed iteration:");
    for i in 0..data.len() {
        println!("  data[{}] = {}", i, data[i]);
    }

    // Safe iteration with enumerate (preferred)
    println!("Safe indexed iteration with enumerate:");
    for (i, value) in data.iter().enumerate() {
        println!("  index {} = {}", i, value);
    }

    // ========================================================================
    // 5. MATHEMATICAL COMPUTATIONS
    // ========================================================================
    println!("\n🧮 === MATHEMATICAL COMPUTATIONS ===");

    // Sum of first N natural numbers
    let n = 100;
    let sum_natural: i32 = (1..=n).sum();
    let formula_result = n * (n + 1) / 2;
    println!(
        "Sum 1 to {}: {} (formula: {})",
        n, sum_natural, formula_result
    );

    // Factorial using ranges
    let factorial_5: i32 = (1..=5).product();
    println!("5! = {}", factorial_5);

    // Fibonacci-like sequence using ranges and fold
    let fib_sum: i32 = (1..=10)
        .fold((0, 1, 0), |(prev, curr, sum), _| {
            let next = prev + curr;
            (curr, next, sum + curr)
        })
        .2;
    println!("Sum of first 10 Fibonacci numbers: {}", fib_sum);

    // Prime numbers in a range
    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }
        !(2..=(n as f64).sqrt() as i32).any(|i| n % i == 0)
    }

    let primes: Vec<i32> = (2..=50).filter(|&n| is_prime(n)).collect();
    println!("Primes 2-50: {:?}", primes);

    // ========================================================================
    // 6. REAL-WORLD USE CASES
    // ========================================================================
    println!("\n🌍 === REAL-WORLD USE CASES ===");

    // Use Case 1: Creating test data
    println!("Creating test data:");

    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        age: u32,
    }

    let test_users: Vec<User> = (1..=5)
        .map(|id| User {
            id,
            name: format!("User{}", id),
            age: 20 + (id * 3), // Ages 23, 26, 29, 32, 35
        })
        .collect();

    for user in &test_users {
        println!("  {:?}", user);
    }

    // Use Case 2: Batch processing
    println!("Batch processing (process in chunks of 3):");
    let all_items: Vec<i32> = (1..=20).collect();

    for chunk_start in (0..all_items.len()).step_by(3) {
        let chunk_end = (chunk_start + 3).min(all_items.len());
        let chunk = &all_items[chunk_start..chunk_end];
        println!("  Processing batch: {:?}", chunk);
    }

    // Use Case 3: Time series data simulation
    println!("Time series simulation (hourly data for 24 hours):");

    #[derive(Debug)]
    struct DataPoint {
        hour: u32,
        temperature: f64,
        humidity: f64,
    }

    let weather_data: Vec<DataPoint> = (0..24)
        .map(|hour| DataPoint {
            hour,
            temperature: 20.0 + (hour as f64 * 0.5) + ((hour as f64 * 0.5).sin() * 5.0),
            humidity: 50.0 + ((hour as f64 * 0.3).cos() * 20.0),
        })
        .collect();

    println!("Sample weather data:");
    for data in weather_data.iter().step_by(6) {
        // Every 6 hours
        println!(
            "  Hour {}: {:.1}°C, {:.1}% humidity",
            data.hour, data.temperature, data.humidity
        );
    }

    // Use Case 4: Grid/Matrix operations
    println!("Grid operations (5x5 matrix):");
    let grid_size = 5;

    // Create coordinates for all grid positions
    let coordinates: Vec<(usize, usize)> = (0..grid_size)
        .flat_map(|row| (0..grid_size).map(move |col| (row, col)))
        .collect();

    println!("Grid coordinates:");
    for (i, (row, col)) in coordinates.iter().enumerate() {
        print!("({},{}) ", row, col);
        if (i + 1) % grid_size == 0 {
            println!();
        } // New line every row
    }

    // Use Case 5: Performance benchmarking
    println!("Performance comparison:");

    let large_range = 1_000_000;

    // Range-based approach (memory efficient)
    let start = std::time::Instant::now();
    let sum_range: i64 = (1..=large_range).map(|x| x as i64).sum();
    let range_time = start.elapsed();

    // Vector-based approach (memory intensive)
    let start = std::time::Instant::now();
    let vec_data: Vec<i32> = (1..=large_range).collect();
    let sum_vec: i64 = vec_data.iter().map(|&x| x as i64).sum();
    let vec_time = start.elapsed();

    println!(
        "Range approach: sum = {}, time = {:?}",
        sum_range, range_time
    );
    println!("Vector approach: sum = {}, time = {:?}", sum_vec, vec_time);
    println!(
        "Memory usage: Range ≈ 0 bytes, Vector ≈ {} MB",
        large_range * 4 / 1_000_000
    ); // i32 = 4 bytes

    // ========================================================================
    // 7. ADVANCED RANGE PATTERNS
    // ========================================================================
    println!("\n🚀 === ADVANCED RANGE PATTERNS ===");

    // Custom step patterns
    println!("Custom patterns:");

    // Powers of 2
    let powers_of_2: Vec<i32> = (0..10).map(|exp| 2_i32.pow(exp)).collect();
    println!("Powers of 2: {:?}", powers_of_2);

    // Alternating signs
    let alternating: Vec<i32> = (1..=10).map(|x| if x % 2 == 0 { -x } else { x }).collect();
    println!("Alternating signs: {:?}", alternating);

    // Nested ranges (multiplication table)
    println!("5x5 Multiplication table:");
    for i in 1..=5 {
        for j in 1..=5 {
            print!("{:3} ", i * j);
        }
        println!();
    }

    // Range-based string generation
    let alphabet: String = ('a'..='z').collect();
    println!("Alphabet: {}", alphabet);

    // Password generation pattern
    let numbers: String = ('0'..='9').collect();
    let symbols = "!@#$%^&*";
    println!(
        "Character sets - Numbers: {}, Symbols: {}",
        numbers, symbols
    );

    // ========================================================================
    // 8. RANGE BOUNDS AND SAFETY
    // ========================================================================
    println!("\n🛡️  === RANGE BOUNDS AND SAFETY ===");

    let data = vec![1, 2, 3, 4, 5];

    // Safe range operations
    println!("Safe range operations:");

    // This is safe - range is within bounds
    if data.len() >= 3 {
        println!("Safe slice [1..3]: {:?}", &data[1..3]);
    }

    // Using get() for safe access
    match data.get(1..4) {
        Some(slice) => println!("Safe get [1..4]: {:?}", slice),
        None => println!("Range [1..4] out of bounds"),
    }

    // Demonstrating bounds checking
    let safe_end = data.len().min(10); // Won't exceed actual length
    println!("Safe range [0..{}]: {:?}", safe_end, &data[0..safe_end]);

    println!("\n📋 === RANGES SUMMARY ===");
    println!("✅ start..end     - Exclusive end (1..5 = 1,2,3,4)");
    println!("✅ start..=end    - Inclusive end (1..=5 = 1,2,3,4,5)");
    println!("✅ ..end          - From 0 to end-1");
    println!("✅ ..=end         - From 0 to end");
    println!("✅ start..        - From start to infinity");
    println!("✅ Memory efficient - O(1) space regardless of size");
    println!("✅ Lazy evaluation - values generated on demand");
    println!("✅ Composable with iterator methods");
    println!("✅ Perfect for loops, slicing, and mathematical operations");
    println!("🎯 Use ranges for efficient, readable sequence generation!");
}
