/*
OPTION COMBINATORS - Powerful Methods for Working with Option<T>
===============================================================

WHAT ARE OPTION COMBINATORS?
- Combinators are methods that allow you to work with Option<T> values safely
- They let you transform, chain, and combine Optional values without explicit unwrapping
- Think of them as "building blocks" for handling optional data elegantly
- They follow functional programming principles - no side effects, composable

REAL-WORLD ANALOGY:
- Like a "toolkit" for working with boxes that might be empty
- Each combinator is a different tool: transformer, checker, combiner, etc.
- You can chain tools together to build complex operations safely

WHY USE COMBINATORS?
✅ Avoid explicit None checking and nested if-let statements
✅ Write more expressive and readable code
✅ Chain operations safely without panicking
✅ Functional programming style - compose operations
✅ Rust's type system ensures safety at compile time

CORE COMBINATORS COVERED:
1. map()        - Transform the value if Some
2. and_then()   - Chain operations that return Option
3. or_else()    - Provide alternative if None
4. filter()     - Keep value only if condition is true
5. unwrap_or()  - Get value or provide default
6. zip()        - Combine two Options
7. take()       - Take ownership, leave None
8. replace()    - Replace value, return old one
*/

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
    email: Option<String>,
}

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip: String,
}

fn main() {
    println!("🧰 === OPTION COMBINATORS MASTERCLASS ===");

    // ========================================================================
    // 1. MAP - Transform the value inside Option
    // ========================================================================
    println!("\n🗺️  === MAP COMBINATOR ===");

    let some_number = Some(42);
    let none_number: Option<i32> = None;

    // Transform Some(42) -> Some(84), None stays None
    let doubled = some_number.map(|x| x * 2);
    let doubled_none = none_number.map(|x| x * 2);

    println!("Original: {:?} -> Doubled: {:?}", some_number, doubled);
    println!("Original: {:?} -> Doubled: {:?}", none_number, doubled_none);

    // Real example: Transform person's name to uppercase
    let person = Some(Person {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    });

    let uppercase_name = person
        .as_ref() // Borrow instead of moving
        .map(|p| p.name.to_uppercase());
    println!("Uppercase name: {:?}", uppercase_name);

    // ========================================================================
    // 2. AND_THEN - Chain operations that return Option (flatMap)
    // ========================================================================
    println!("\n🔗 === AND_THEN COMBINATOR ===");

    fn parse_age(s: &str) -> Option<u32> {
        s.parse().ok()
    }

    fn validate_adult(age: u32) -> Option<u32> {
        if age >= 18 {
            Some(age)
        } else {
            None
        }
    }

    // Chain: parse string -> validate adult
    let age_input = "25";
    let valid_adult_age = Some(age_input)
        .and_then(|s| parse_age(s)) // Parse string to number
        .and_then(|age| validate_adult(age)); // Validate age >= 18

    println!(
        "Input '{}' -> Valid adult age: {:?}",
        age_input, valid_adult_age
    );

    let invalid_input = "15";
    let invalid_adult_age = Some(invalid_input)
        .and_then(|s| parse_age(s))
        .and_then(|age| validate_adult(age));

    println!(
        "Input '{}' -> Valid adult age: {:?}",
        invalid_input, invalid_adult_age
    );

    // ========================================================================
    // 3. OR_ELSE - Provide alternative if None
    // ========================================================================
    println!("\n🔄 === OR_ELSE COMBINATOR ===");

    fn get_cached_data() -> Option<String> {
        None // Simulate cache miss
    }

    fn fetch_from_database() -> Option<String> {
        Some("Data from database".to_string())
    }

    fn fetch_from_api() -> Option<String> {
        Some("Data from API".to_string())
    }

    // Try cache first, then database, then API
    let data = get_cached_data()
        .or_else(|| fetch_from_database())
        .or_else(|| fetch_from_api());

    println!("Fetched data: {:?}", data);

    // ========================================================================
    // 4. FILTER - Keep value only if condition is true
    // ========================================================================
    println!("\n🔍 === FILTER COMBINATOR ===");

    let numbers = vec![Some(1), Some(15), Some(25), Some(35), None, Some(45)];

    for (i, num_opt) in numbers.iter().enumerate() {
        let filtered = num_opt.filter(|&n| n > 20); // Keep only numbers > 20

        println!(
            "Index {}: {:?} -> Filtered (>20): {:?}",
            i, num_opt, filtered
        );
    }

    // Real example: Filter valid email addresses
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            email: Some("invalid-email".to_string()),
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
            email: None,
        },
    ];

    for person in &people {
        let valid_email = person
            .email
            .as_ref()
            .filter(|email| email.contains('@') && email.contains('.'));

        println!("{}: Valid email: {:?}", person.name, valid_email);
    }

    // ========================================================================
    // 5. UNWRAP_OR and UNWRAP_OR_ELSE - Get value or default
    // ========================================================================
    println!("\n📦 === UNWRAP_OR COMBINATORS ===");

    let some_value = Some("Hello");
    let none_value: Option<&str> = None;

    // unwrap_or - provide static default
    println!(
        "Some: '{}', None: '{}'",
        some_value.unwrap_or("Default"),
        none_value.unwrap_or("Default")
    );

    // unwrap_or_else - compute default lazily
    let expensive_default = || {
        println!("  Computing expensive default...");
        "Computed Default".to_string()
    };

    println!(
        "Some: {}",
        Some("Value".to_string()).unwrap_or_else(expensive_default)
    );
    println!("None: {}", None.unwrap_or_else(expensive_default));

    // ========================================================================
    // 6. ZIP - Combine two Options
    // ========================================================================
    println!("\n🤐 === ZIP COMBINATOR ===");

    let first_name = Some("John");
    let last_name = Some("Doe");
    let missing_name: Option<&str> = None;

    // Both Some -> Some((first, last))
    let full_name = first_name.zip(last_name);
    println!("Full name: {:?}", full_name);

    // One None -> None
    let incomplete_name = first_name.zip(missing_name);
    println!("Incomplete name: {:?}", incomplete_name);

    // Real example: Combine coordinates
    let x = Some(10);
    let y = Some(20);
    let coordinates = x.zip(y).map(|(x, y)| format!("({}, {})", x, y));
    println!("Coordinates: {:?}", coordinates);

    // ========================================================================
    // 7. TAKE and REPLACE - Ownership manipulation
    // ========================================================================
    println!("\n🔄 === TAKE and REPLACE COMBINATORS ===");

    let mut maybe_value = Some("Original".to_string());
    println!("Before take: {:?}", maybe_value);

    // take() removes value, leaves None
    let taken = maybe_value.take();
    println!("Taken: {:?}, Remaining: {:?}", taken, maybe_value);

    // replace() sets new value, returns old one
    let old_value = maybe_value.replace("New Value".to_string());
    println!("Old: {:?}, New: {:?}", old_value, maybe_value);

    // ========================================================================
    // 8. ADVANCED CHAINING - Real-world example
    // ========================================================================
    println!("\n🌟 === ADVANCED CHAINING EXAMPLE ===");

    fn parse_user_input(input: &str) -> Option<Person> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 3 {
            return None;
        }

        let name = parts[0].trim().to_string();
        let age = parts[1].trim().parse().ok()?;
        let email = if parts[2].trim().is_empty() {
            None
        } else {
            Some(parts[2].trim().to_string())
        };

        Some(Person { name, age, email })
    }

    fn validate_person(person: Person) -> Option<Person> {
        if person.age >= 13 && !person.name.is_empty() {
            Some(person)
        } else {
            None
        }
    }

    fn format_person(person: Person) -> String {
        match person.email {
            Some(email) => format!("{} ({}) - {}", person.name, person.age, email),
            None => format!("{} ({})", person.name, person.age),
        }
    }

    // Chain multiple operations
    let inputs = vec![
        "Alice, 30, alice@example.com",
        "Bob, 25, ",
        "Charlie, 12, charlie@example.com", // Too young
        "Invalid input",
    ];

    for input in inputs {
        let result = Some(input)
            .and_then(|s| parse_user_input(s)) // Parse input
            .and_then(|p| validate_person(p)) // Validate person
            .map(|p| format_person(p)) // Format output
            .unwrap_or_else(|| "Invalid person".to_string());

        println!("Input: '{}' -> Result: '{}'", input, result);
    }

    // ========================================================================
    // 9. COMPARISON: Imperative vs Functional Style
    // ========================================================================
    println!("\n⚖️  === IMPERATIVE vs FUNCTIONAL COMPARISON ===");

    let user_data = "John,25,john@example.com";

    // Imperative style (traditional)
    println!("Imperative style:");
    let parts: Vec<&str> = user_data.split(',').collect();
    if parts.len() == 3 {
        if let Ok(age) = parts[1].parse::<u32>() {
            if age >= 18 {
                let name = parts[0].to_uppercase();
                let email = if !parts[2].is_empty() {
                    Some(parts[2])
                } else {
                    None
                };
                println!("  Valid adult: {} ({}), Email: {:?}", name, age, email);
            } else {
                println!("  Too young");
            }
        } else {
            println!("  Invalid age");
        }
    } else {
        println!("  Invalid format");
    }

    // Functional style (with combinators)
    println!("Functional style:");
    let result = Some(user_data)
        .and_then(|s| parse_user_input(s))
        .filter(|p| p.age >= 18)
        .map(|mut p| {
            p.name = p.name.to_uppercase();
            p
        })
        .map(|p| format!("Valid adult: {} ({}), Email: {:?}", p.name, p.age, p.email))
        .unwrap_or_else(|| "Invalid or underage user".to_string());

    println!("  {}", result);

    println!("\n📋 === OPTION COMBINATORS SUMMARY ===");
    println!("✅ map()        - Transform value inside Option");
    println!("✅ and_then()   - Chain operations returning Option (flatMap)");
    println!("✅ or_else()    - Provide alternative computation if None");
    println!("✅ filter()     - Keep value only if predicate is true");
    println!("✅ unwrap_or()  - Get value or static default");
    println!("✅ zip()        - Combine two Options into one");
    println!("✅ take()       - Take ownership, leave None behind");
    println!("✅ replace()    - Replace value, return old one");
    println!("🎯 Combinators make code more readable, safe, and functional!");
}
