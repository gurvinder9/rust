/*
MAP COMBINATOR - Transforming Option<T> Values
==============================================

WHAT IS MAP COMBINATOR?
- .map() is a method on Option<T> that transforms the value inside if it exists
- If Option is Some(value) → applies function and returns Some(new_value)
- If Option is None → returns None (doesn't apply function)
- It's a safe way to transform optional values without unwrapping

SYNTAX: option.map(|value| transform_function)

REAL-WORLD ANALOGY:
- Like a "conditional transformer" - only works if there's something to transform
- Think of it as: "If there's a gift in the box, transform it; if empty box, keep it empty"

WHEN TO USE MAP:
✅ Transform Option<T> to Option<U> (different type)
✅ Apply operations only when value exists
✅ Chain operations safely without explicit None checking
✅ Functional programming style
✅ Avoid nested if-let or match statements

COMMON PATTERNS:
- option.map(|x| x * 2)           // Transform value
- option.map(|s| s.to_uppercase()) // Transform string
- option.map(|user| user.name)     // Extract field
*/

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

fn find_user(name: String) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "john" => Some(1),
        "jane" => Some(2),
        _ => None,
    }
}

fn main() {
    println!("🗺️  === MAP COMBINATOR EXAMPLES ===");

    // EXAMPLE 1: Basic map usage - SUCCESS CASE
    let name1 = "John".to_owned();
    let user1 = find_user(name1.clone()) // Clone to avoid move
        .map(|user_id| User {
            user_id,
            name: name1, // Now we can use name1 here
        });

    match user1 {
        Some(user) => println!("✅ Found user: {:?}", user),
        None => println!("❌ User not found"),
    }

    // EXAMPLE 2: Basic map usage - FAILURE CASE
    let name2 = "Unknown".to_owned();
    let user2 = find_user(name2.clone()).map(|user_id| User {
        user_id,
        name: name2,
    });

    match user2 {
        Some(user) => println!("✅ Found user: {:?}", user),
        None => println!("❌ User not found"),
    }

    // EXAMPLE 3: Chaining map operations
    println!("\n🔗 === CHAINING MAP OPERATIONS ===");

    let result = find_user("Jane".to_owned())
        .map(|user_id| user_id * 10) // Transform: multiply by 10
        .map(|big_id| format!("ID-{}", big_id)); // Transform: format as string

    match result {
        Some(formatted_id) => println!("Formatted ID: {}", formatted_id),
        None => println!("No user found to format"),
    }

    // EXAMPLE 4: Map vs direct unwrap (showing safety)
    println!("\n🛡️  === MAP SAFETY vs UNWRAP DANGER ===");

    // Safe approach with map
    let safe_result = find_user("john".to_owned())
        .map(|id| format!("User ID: {}", id))
        .unwrap_or_else(|| "No user found".to_string());
    println!("Safe result: {}", safe_result);

    // EXAMPLE 5: Common mistake - trying to use moved value
    println!("\n❌ === COMMON MISTAKES (COMMENTED OUT) ===");
    println!("// This would cause a compile error:");
    println!("// let name = \"John\".to_owned();");
    println!("// let user = find_user(name).map(|id| User {{ id, name }});");
    println!("//                     ^^^^              moved    ^^^^ used after move");

    // EXAMPLE 6: Solutions to the move problem
    println!("\n✅ === SOLUTIONS TO MOVE PROBLEM ===");

    // Solution 1: Clone the value
    let name_clone = "John".to_owned();
    let user_with_clone = find_user(name_clone.clone()).map(|user_id| User {
        user_id,
        name: name_clone, // Use the cloned value
    });
    println!("Solution 1 (clone): {:?}", user_with_clone);

    // Solution 2: Use string literal (implements Copy)
    let user_with_literal = find_user("John".to_owned()).map(|user_id| User {
        user_id,
        name: "John".to_owned(), // Create new String
    });
    println!("Solution 2 (literal): {:?}", user_with_literal);

    // Solution 3: Move into closure with move keyword
    let name_move = "John".to_owned();
    let user_with_move = find_user("John".to_owned()) // Use separate string for function
        .map(move |user_id| User {
            user_id,
            name: name_move, // Move name_move into closure
        });
    println!("Solution 3 (move): {:?}", user_with_move);

    // EXAMPLE 7: Practical real-world usage
    println!("\n🌍 === REAL-WORLD EXAMPLES ===");

    // Parse and transform user input
    let user_input = "42";
    let parsed_and_doubled = user_input
        .parse::<i32>() // Returns Result<i32, ParseIntError>
        .ok() // Convert Result to Option
        .map(|num| num * 2) // Double if parsing succeeded
        .map(|doubled| format!("Result: {}", doubled)); // Format as string

    match parsed_and_doubled {
        Some(result) => println!("Parsed and doubled: {}", result),
        None => println!("Failed to parse input"),
    }

    // Working with optional configuration
    let config_value: Option<String> = Some("debug".to_owned());
    let log_level = config_value
        .map(|level| level.to_uppercase()) // Convert to uppercase
        .map(|level| format!("[{}]", level)); // Add brackets

    println!("Log level: {:?}", log_level);

    println!("\n📋 === MAP COMBINATOR SUMMARY ===");
    println!("✅ .map() transforms Option<T> → Option<U> safely");
    println!("✅ Only applies transformation if Some(value) exists");
    println!("✅ Returns None if original Option was None");
    println!("✅ Chainable for multiple transformations");
    println!("✅ Avoids explicit None checking and unwrapping");
    println!("⚠️  Watch out for moved values in closures!");
}
