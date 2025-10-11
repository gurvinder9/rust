/*
ITERATORS IN RUST - Powerful, Zero-Cost Data Processing
======================================================

WHAT ARE ITERATORS?
- Iterators are objects that can traverse through collections of data
- They process elements one at a time in a lazy, efficient manner
- Rust iterators are "zero-cost abstractions" - no runtime overhead!
- They follow functional programming patterns - immutable, composable

REAL-WORLD ANALOGY:
- Like an assembly line worker who processes items one by one
- Each worker (iterator method) does one specific job and passes to next
- The line only runs when you need the final product (lazy evaluation)
- Very efficient - no wasted work or temporary storage

ITERATOR vs FOR LOOP:
┌─────────────────┬─────────────────────────────────────┬─────────────────────────────────────┐
│     Feature     │            For Loop                 │            Iterator                 │
├─────────────────┼─────────────────────────────────────┼─────────────────────────────────────┤
│ Style           │ Imperative (how to do)             │ Functional (what to do)             │
│ Performance     │ Good                                │ Often faster (zero-cost)           │
│ Composability   │ Limited                             │ Highly composable                   │
│ Readability     │ Verbose for complex operations      │ Expressive and concise              │
│ Memory          │ May need temporary collections      │ Lazy - no intermediate collections  │
└─────────────────┴─────────────────────────────────────┴─────────────────────────────────────┘

WHEN TO USE ITERATORS:
✅ Data transformation pipelines
✅ Filtering and mapping operations
✅ Mathematical computations on collections
✅ Functional programming style
✅ Performance-critical code
✅ Complex data processing workflows
✅ When you want expressive, readable code

ITERATOR TYPES:
1. iter()       - Borrows elements (&T)
2. into_iter()  - Takes ownership (T)
3. iter_mut()   - Mutable references (&mut T)

KEY METHODS:
- Transformers: map(), filter(), enumerate(), zip()
- Consumers: collect(), fold(), reduce(), for_each()
- Finders: find(), any(), all(), position()
- Takers: take(), skip(), take_while(), skip_while()
*/

fn main() {
    println!("🔄 === ITERATOR FUNDAMENTALS ===");

    // ========================================================================
    // 1. BASIC ITERATOR TYPES
    // ========================================================================

    let numbers = vec![1, 2, 3, 4, 5];

    // Method 1: iter() - borrows elements (&i32)
    println!("\n1️⃣ Using .iter() - borrows elements:");
    for item in numbers.iter() {
        println!("  Borrowed: {} (type: &i32)", item);
    }
    println!("  ✅ Original vector still available: {:?}", numbers);

    // Method 2: Cloning for demonstration
    let numbers_clone = numbers.clone();

    // Method 3: into_iter() - takes ownership (i32)
    println!("\n2️⃣ Using .into_iter() - takes ownership:");
    for item in numbers_clone.into_iter() {
        println!("  Owned: {} (type: i32)", item);
    }
    println!("  ⚠️  numbers_clone is no longer available (moved)");

    // Method 4: iter_mut() - mutable references (&mut i32)
    let mut numbers_mut = vec![1, 2, 3, 4, 5];
    println!("\n3️⃣ Using .iter_mut() - mutable references:");
    for item in numbers_mut.iter_mut() {
        *item *= 10; // Modify in place
        println!("  Modified: {} (type: &mut i32)", item);
    }
    println!("  Modified vector: {:?}", numbers_mut);

    // ========================================================================
    // 2. ITERATOR TRANSFORMATIONS
    // ========================================================================
    println!("\n🔄 === ITERATOR TRANSFORMATIONS ===");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // MAP - Transform each element
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", data);
    println!("Doubled:  {:?}", doubled);

    // FILTER - Keep elements that match condition
    let evens: Vec<&i32> = data.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens:    {:?}", evens);

    // CHAINING - Combine multiple operations
    let processed: Vec<i32> = data
        .iter()
        .filter(|&x| x % 2 == 1) // Keep odd numbers
        .map(|x| x * x) // Square them
        .filter(|&x| x > 10) // Keep if > 10
        .collect();
    println!("Odd squares > 10: {:?}", processed);

    // ========================================================================
    // 3. ITERATOR CONSUMERS
    // ========================================================================
    println!("\n🎯 === ITERATOR CONSUMERS ===");

    let numbers = vec![1, 2, 3, 4, 5];

    // FOLD - Accumulate values (like reduce)
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Sum: {}, Product: {}", sum, product);

    // REDUCE - Similar to fold but uses first element as initial value
    let sum_reduce = numbers.iter().cloned().reduce(|acc, x| acc + x);
    println!("Sum (reduce): {:?}", sum_reduce);

    // FOR_EACH - Execute closure on each element (side effects)
    print!("For each: ");
    numbers.iter().for_each(|x| print!("{} ", x));
    println!();

    // FIND - Find first element matching condition
    let found = numbers.iter().find(|&x| *x > 3);
    println!("First > 3: {:?}", found);

    // ANY/ALL - Check conditions
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("Has even: {}, All positive: {}", has_even, all_positive);

    // ========================================================================
    // 4. ADVANCED ITERATOR METHODS
    // ========================================================================
    println!("\n🚀 === ADVANCED ITERATOR METHODS ===");

    let data = vec![10, 20, 30, 40, 50];

    // ENUMERATE - Add index to each element
    let with_index: Vec<(usize, &i32)> = data.iter().enumerate().collect();
    println!("With index: {:?}", with_index);

    // ZIP - Combine two iterators
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    let zipped: Vec<(&i32, &char)> = data.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", zipped);

    // TAKE/SKIP - Take or skip elements
    let first_three: Vec<&i32> = data.iter().take(3).collect();
    let skip_two: Vec<&i32> = data.iter().skip(2).collect();
    println!("First 3: {:?}, Skip 2: {:?}", first_three, skip_two);

    // TAKE_WHILE/SKIP_WHILE - Conditional taking/skipping
    let take_while_small: Vec<&i32> = data.iter().take_while(|&x| *x < 35).collect();
    let skip_while_small: Vec<&i32> = data.iter().skip_while(|&x| *x < 35).collect();
    println!("Take while < 35: {:?}", take_while_small);
    println!("Skip while < 35: {:?}", skip_while_small);

    // ========================================================================
    // 5. REAL-WORLD EXAMPLES
    // ========================================================================
    println!("\n🌍 === REAL-WORLD EXAMPLES ===");

    // Example 1: Processing student grades
    #[derive(Debug)]
    struct Student {
        name: String,
        grade: f64,
    }

    let students = vec![
        Student {
            name: "Alice".to_string(),
            grade: 85.5,
        },
        Student {
            name: "Bob".to_string(),
            grade: 92.0,
        },
        Student {
            name: "Charlie".to_string(),
            grade: 78.5,
        },
        Student {
            name: "Diana".to_string(),
            grade: 96.5,
        },
        Student {
            name: "Eve".to_string(),
            grade: 82.0,
        },
    ];

    // Find honor students (grade >= 90)
    let honor_students: Vec<&Student> = students
        .iter()
        .filter(|student| student.grade >= 90.0)
        .collect();

    println!("Honor students:");
    for student in &honor_students {
        println!("  {} - {:.1}", student.name, student.grade);
    }

    // Calculate class statistics
    let total_students = students.len();
    let average_grade = students.iter().map(|s| s.grade).sum::<f64>() / total_students as f64;
    let highest_grade = students.iter().map(|s| s.grade).fold(0.0, f64::max);

    println!(
        "Class stats: {} students, avg: {:.1}, highest: {:.1}",
        total_students, average_grade, highest_grade
    );

    // Example 2: Text processing
    let text = "The quick brown fox jumps over the lazy dog";
    let word_lengths: Vec<usize> = text.split_whitespace().map(|word| word.len()).collect();

    let long_words: Vec<&str> = text
        .split_whitespace()
        .filter(|word| word.len() > 4)
        .collect();

    println!("Word lengths: {:?}", word_lengths);
    println!("Long words (>4 chars): {:?}", long_words);

    // Example 3: Number processing pipeline
    let raw_data = vec![1, -2, 3, -4, 5, -6, 7, -8, 9, -10];

    let result: Vec<String> = raw_data
        .iter()
        .filter(|&x| *x > 0) // Keep positive numbers
        .map(|x| x * x) // Square them
        .filter(|&x| x > 10) // Keep if > 10
        .map(|x| format!("{}²", (x as f64).sqrt() as i32)) // Format as "n²"
        .collect();

    println!("Processing pipeline result: {:?}", result);

    // ========================================================================
    // 6. PERFORMANCE COMPARISON
    // ========================================================================
    println!("\n⚡ === PERFORMANCE COMPARISON ===");

    let large_data: Vec<i32> = (1..=1000).collect();

    // Traditional for loop approach
    let start = std::time::Instant::now();
    let mut sum_loop = 0;
    for &num in &large_data {
        if num % 2 == 0 {
            sum_loop += num * num;
        }
    }
    let loop_time = start.elapsed();

    // Iterator approach
    let start = std::time::Instant::now();
    let sum_iter: i32 = large_data
        .iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    let iter_time = start.elapsed();

    println!("For loop result: {}, time: {:?}", sum_loop, loop_time);
    println!("Iterator result: {}, time: {:?}", sum_iter, iter_time);
    println!("Both should be equally fast due to zero-cost abstractions!");

    // ========================================================================
    // 7. LAZY EVALUATION DEMONSTRATION
    // ========================================================================
    println!("\n😴 === LAZY EVALUATION DEMO ===");

    let numbers = vec![1, 2, 3, 4, 5];

    // This creates an iterator but doesn't execute anything yet!
    let lazy_iter = numbers
        .iter()
        .inspect(|x| println!("  Processing: {}", x)) // Debug what's being processed
        .map(|x| {
            println!("    Squaring: {}", x);
            x * x
        })
        .filter(|&x| {
            println!("    Filtering: {} > 10? {}", x, x > 10);
            x > 10
        });

    println!("Iterator created but nothing executed yet!");
    println!("Now collecting results:");

    // Only when we call collect() does the work happen
    let results: Vec<i32> = lazy_iter.collect();
    println!("Final results: {:?}", results);

    println!("\n📋 === ITERATOR SUMMARY ===");
    println!("✅ Iterators are zero-cost abstractions - no runtime overhead");
    println!("✅ Lazy evaluation - work only happens when consumed");
    println!("✅ Highly composable - chain operations together");
    println!("✅ More expressive than traditional loops");
    println!("✅ Functional programming style - immutable, side-effect free");
    println!("✅ Often faster than hand-written loops due to optimizations");
    println!("🎯 Use iterators for data transformation, filtering, and processing!");
}
