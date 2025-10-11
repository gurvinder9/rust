/*
IF LET - Concise Pattern Matching in Rust
=========================================

WHAT IS IF LET?
- if let is a concise way to match against a single pattern
- It combines pattern matching with conditional execution
- Reduces boilerplate compared to full match statements
- Perfect when you only care about one specific pattern

REAL-WORLD ANALOGY:
- Like a "smart filter" that only acts if something matches exactly what you want
- Think of it as: "If this envelope contains exactly what I'm looking for, then do something"
- Instead of checking all possible mail types, you only care about one specific type
- More focused than a full sorting system (match) when you only need one check

SYNTAX:
if let PATTERN = EXPRESSION {
    // Code to execute if pattern matches
} else {
    // Optional: code if pattern doesn't match
}

WHEN TO USE IF LET:
✅ When you only care about one variant of an enum
✅ Extracting values from Option<T> or Result<T, E>
✅ Simplifying single-pattern matches
✅ Making code more readable and concise
✅ Avoiding exhaustive match when only one case matters
✅ Working with nested data structures

IF LET vs MATCH:
┌─────────────────┬─────────────────────────────────────┬─────────────────────────────────────┐
│     Feature     │              MATCH                  │             IF LET                  │
├─────────────────┼─────────────────────────────────────┼─────────────────────────────────────┤
│ Exhaustiveness  │ Must handle all cases               │ Only handles one pattern            │
│ Verbosity       │ More verbose for single patterns   │ Concise for single patterns        │
│ Use Case        │ Multiple patterns/complex logic     │ Single pattern matching             │
│ Readability     │ Good for complex branching          │ Excellent for simple cases         │
│ Flexibility     │ Full pattern matching power         │ Limited to one pattern + else       │
└─────────────────┴─────────────────────────────────────┴─────────────────────────────────────┘

RELATED PATTERNS:
- while let - Loop while pattern matches
- let else - Pattern match with early return
- match guards - Additional conditions in patterns
*/

fn main() {
    println!("🔍 === IF LET PATTERN MATCHING MASTERCLASS ===");

    // ========================================================================
    // 1. BASIC IF LET WITH OPTION
    // ========================================================================
    println!("\n📦 === BASIC IF LET WITH OPTION ===");

    let some_value = Some(42);
    let none_value: Option<i32> = None;

    // Traditional match approach (verbose)
    println!("Traditional match approach:");
    match some_value {
        Some(x) => println!("  Found value: {}", x),
        None => println!("  No value found"),
    }

    // if let approach (concise)
    println!("if let approach:");
    if let Some(x) = some_value {
        println!("  Found value: {}", x);
    } else {
        println!("  No value found");
    }

    // if let without else (only care about Some case)
    println!("if let without else:");
    if let Some(x) = some_value {
        println!("  Processing value: {}", x);
    }
    // No else needed - we don't care about None case

    if let Some(x) = none_value {
        println!("  This won't print: {}", x);
    }
    println!("  (No output for None case)");

    // ========================================================================
    // 2. IF LET WITH RESULT
    // ========================================================================
    println!("\n✅ === IF LET WITH RESULT ===");

    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse()
    }

    let valid_input = "42";
    let invalid_input = "not_a_number";

    // Check for successful parsing only
    if let Ok(number) = parse_number(valid_input) {
        println!("Successfully parsed: {}", number);
        println!("Double the number: {}", number * 2);
    }

    // Check for errors only
    if let Err(error) = parse_number(invalid_input) {
        println!("Parsing failed: {}", error);
        println!("Error kind: {:?}", error.kind());
    }

    // Both success and error cases
    if let Ok(number) = parse_number("123") {
        println!("Valid number: {}", number);
    } else {
        println!("Invalid number format");
    }

    // ========================================================================
    // 3. IF LET WITH CUSTOM ENUMS
    // ========================================================================
    println!("\n🎭 === IF LET WITH CUSTOM ENUMS ===");

    #[derive(Debug)]
    enum Message {
        Text(String),
        Image {
            url: String,
            width: u32,
            height: u32,
        },
        Video {
            url: String,
            duration: u32,
        },
        Audio(String, u32), // url, duration
    }

    let messages = vec![
        Message::Text("Hello, world!".to_string()),
        Message::Image {
            url: "photo.jpg".to_string(),
            width: 800,
            height: 600,
        },
        Message::Video {
            url: "video.mp4".to_string(),
            duration: 120,
        },
        Message::Audio("song.mp3".to_string(), 180),
    ];

    for (i, message) in messages.iter().enumerate() {
        println!("Message {}:", i + 1);

        // Only handle text messages
        if let Message::Text(content) = message {
            println!("  📝 Text message: '{}'", content);
            println!("  📊 Length: {} characters", content.len());
        }

        // Only handle images
        if let Message::Image { url, width, height } = message {
            println!("  🖼️  Image: {}", url);
            println!("  📐 Dimensions: {}x{}", width, height);
            println!("  📊 Aspect ratio: {:.2}", *width as f64 / *height as f64);
        }

        // Handle both video and audio (media files)
        if let Message::Video { url, duration } = message {
            println!("  🎬 Video: {} ({}s)", url, duration);
        } else if let Message::Audio(url, duration) = message {
            println!("  🎵 Audio: {} ({}s)", url, duration);
        }
    }

    // ========================================================================
    // 4. NESTED IF LET PATTERNS
    // ========================================================================
    println!("\n🪆 === NESTED IF LET PATTERNS ===");

    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,
        profile: Option<Profile>,
    }

    #[derive(Debug)]
    struct Profile {
        bio: Option<String>,
        avatar: Option<String>,
        social_links: Vec<String>,
    }

    let users = vec![
        User {
            name: "Alice".to_string(),
            email: Some("alice@example.com".to_string()),
            profile: Some(Profile {
                bio: Some("Software developer".to_string()),
                avatar: Some("alice.jpg".to_string()),
                social_links: vec!["twitter.com/alice".to_string()],
            }),
        },
        User {
            name: "Bob".to_string(),
            email: None,
            profile: None,
        },
        User {
            name: "Charlie".to_string(),
            email: Some("charlie@example.com".to_string()),
            profile: Some(Profile {
                bio: None,
                avatar: None,
                social_links: vec![],
            }),
        },
    ];

    for user in &users {
        println!("User: {}", user.name);

        // Check if user has email
        if let Some(email) = &user.email {
            println!("  📧 Email: {}", email);
        }

        // Check if user has profile
        if let Some(profile) = &user.profile {
            println!("  👤 Has profile");

            // Nested: check if profile has bio
            if let Some(bio) = &profile.bio {
                println!("    📝 Bio: {}", bio);
            }

            // Nested: check if profile has avatar
            if let Some(avatar) = &profile.avatar {
                println!("    🖼️  Avatar: {}", avatar);
            }

            // Check social links
            if !profile.social_links.is_empty() {
                println!("    🔗 Social links: {}", profile.social_links.len());
            }
        } else {
            println!("  ❌ No profile");
        }
        println!();
    }

    // ========================================================================
    // 5. IF LET WITH GUARDS AND CONDITIONS
    // ========================================================================
    println!("\n🛡️  === IF LET WITH GUARDS AND CONDITIONS ===");

    let numbers = vec![Some(1), Some(15), Some(25), None, Some(35), Some(5)];

    for (i, num_opt) in numbers.iter().enumerate() {
        print!("Index {}: ", i);

        // if let with additional condition
        if let Some(n) = num_opt {
            if *n > 20 {
                println!("Large number: {}", n);
            } else if *n > 10 {
                println!("Medium number: {}", n);
            } else {
                println!("Small number: {}", n);
            }
        } else {
            println!("No number");
        }
    }

    // Multiple if let checks
    println!("Categorizing numbers:");
    for num_opt in &numbers {
        if let Some(n) = num_opt {
            if *n % 2 == 0 {
                println!("  Even number: {}", n);
            }
        }

        if let Some(n) = num_opt {
            if *n > 10 && *n < 30 {
                println!("  Number in range 10-30: {}", n);
            }
        }
    }

    // ========================================================================
    // 6. WHILE LET PATTERNS
    // ========================================================================
    println!("\n🔄 === WHILE LET PATTERNS ===");

    // while let with iterator
    let mut numbers = vec![1, 2, 3, 4, 5].into_iter();

    println!("Processing numbers with while let:");
    while let Some(num) = numbers.next() {
        println!("  Processing: {}", num);
        if num == 3 {
            println!("    Found special number 3!");
        }
    }

    // while let with stack (LIFO)
    let mut stack = vec!["first", "second", "third", "fourth"];

    println!("Popping from stack:");
    while let Some(item) = stack.pop() {
        println!("  Popped: {}", item);
        if item == "second" {
            println!("    Stopping at 'second'");
            break;
        }
    }
    println!("  Remaining in stack: {:?}", stack);

    // ========================================================================
    // 7. REAL-WORLD EXAMPLES
    // ========================================================================
    println!("\n🌍 === REAL-WORLD EXAMPLES ===");

    // Example 1: Configuration parsing
    use std::collections::HashMap;

    let mut config = HashMap::new();
    config.insert("database_url", "postgres://localhost/mydb");
    config.insert("port", "8080");
    config.insert("debug", "true");

    println!("Configuration parsing:");

    // Check for database URL
    if let Some(db_url) = config.get("database_url") {
        println!("  🗄️  Database URL: {}", db_url);

        // Parse port if available
        if let Some(port_str) = config.get("port") {
            if let Ok(port) = port_str.parse::<u16>() {
                println!("  🌐 Port: {}", port);
            } else {
                println!("  ❌ Invalid port format");
            }
        }
    }

    // Check debug mode
    if let Some(debug_str) = config.get("debug") {
        if let Ok(debug_mode) = debug_str.parse::<bool>() {
            if debug_mode {
                println!("  🐛 Debug mode enabled");
            }
        }
    }

    // Example 2: JSON-like data processing
    #[derive(Debug)]
    enum JsonValue {
        String(String),
        Number(f64),
        Boolean(bool),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
        Null,
    }

    let json_data = JsonValue::Object({
        let mut obj = HashMap::new();
        obj.insert("name".to_string(), JsonValue::String("Alice".to_string()));
        obj.insert("age".to_string(), JsonValue::Number(30.0));
        obj.insert("active".to_string(), JsonValue::Boolean(true));
        obj.insert(
            "tags".to_string(),
            JsonValue::Array(vec![
                JsonValue::String("developer".to_string()),
                JsonValue::String("rust".to_string()),
            ]),
        );
        obj
    });

    println!("JSON data processing:");

    if let JsonValue::Object(obj) = &json_data {
        // Extract name
        if let Some(JsonValue::String(name)) = obj.get("name") {
            println!("  👤 Name: {}", name);
        }

        // Extract age
        if let Some(JsonValue::Number(age)) = obj.get("age") {
            println!("  🎂 Age: {}", age);
        }

        // Extract and process tags array
        if let Some(JsonValue::Array(tags)) = obj.get("tags") {
            println!("  🏷️  Tags:");
            for tag in tags {
                if let JsonValue::String(tag_str) = tag {
                    println!("    - {}", tag_str);
                }
            }
        }
    }

    // Example 3: Error handling chain
    #[derive(Debug)]
    enum ProcessingError {
        InvalidInput(String),
        NetworkError(String),
        DatabaseError(String),
    }

    fn process_data(input: &str) -> Result<String, ProcessingError> {
        if input.is_empty() {
            Err(ProcessingError::InvalidInput("Empty input".to_string()))
        } else if input.contains("network_fail") {
            Err(ProcessingError::NetworkError(
                "Connection failed".to_string(),
            ))
        } else if input.contains("db_fail") {
            Err(ProcessingError::DatabaseError("Query failed".to_string()))
        } else {
            Ok(format!("Processed: {}", input))
        }
    }

    let test_inputs = vec!["valid_data", "", "network_fail_test", "db_fail_test"];

    println!("Error handling with if let:");
    for input in test_inputs {
        match process_data(input) {
            Ok(result) => println!("  ✅ Success: {}", result),
            Err(error) => {
                // Handle specific error types with if let
                if let ProcessingError::InvalidInput(msg) = error {
                    println!("  ❌ Input Error: {}", msg);
                } else if let ProcessingError::NetworkError(msg) = error {
                    println!("  🌐 Network Error: {}", msg);
                } else if let ProcessingError::DatabaseError(msg) = error {
                    println!("  🗄️  Database Error: {}", msg);
                }
            }
        }
    }

    // ========================================================================
    // 8. COMPARISON: IF LET vs MATCH
    // ========================================================================
    println!("\n⚖️  === IF LET vs MATCH COMPARISON ===");

    let option_value = Some(42);

    println!("Same logic, different approaches:");

    // Using match (exhaustive)
    println!("Match approach:");
    match option_value {
        Some(x) if x > 40 => println!("  Large value: {}", x),
        Some(x) => println!("  Small value: {}", x),
        None => println!("  No value"),
    }

    // Using if let (focused)
    println!("if let approach:");
    if let Some(x) = option_value {
        if x > 40 {
            println!("  Large value: {}", x);
        } else {
            println!("  Small value: {}", x);
        }
    } else {
        println!("  No value");
    }

    // When if let is better (single pattern)
    println!("When if let shines (single pattern focus):");
    if let Some(x) = option_value {
        println!("  Just processing the value: {}", x * 2);
    }
    // Much cleaner than:
    // match option_value {
    //     Some(x) => println!("Just processing the value: {}", x * 2),
    //     None => {} // Empty case we don't care about
    // }

    println!("\n📋 === IF LET SUMMARY ===");
    println!("✅ if let PATTERN = EXPRESSION - concise pattern matching");
    println!("✅ Perfect for single-pattern matches with Option/Result");
    println!("✅ Reduces boilerplate compared to full match statements");
    println!("✅ Can be nested and combined with conditions");
    println!("✅ while let - loop while pattern matches");
    println!("✅ More readable than match when you only care about one case");
    println!("✅ Use with custom enums for clean, focused logic");
    println!("🎯 Choose if let for simplicity, match for exhaustive handling!");
}
