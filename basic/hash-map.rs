/*
HASH MAP (HashMap<K, V>) - Key-Value Storage
===========================================

WHAT IS A HASH MAP?
- A HashMap is like a dictionary or phone book
- It stores data in KEY-VALUE pairs
- You use a KEY to quickly find its associated VALUE
- Think of it like: Key = "Name", Value = "Phone Number"

REAL-WORLD ANALOGY:
- Phone Book: Name (key) → Phone Number (value)
- Dictionary: Word (key) → Definition (value)
- Student Records: Student ID (key) → Grade (value)
- Inventory: Product Name (key) → Stock Count (value)

WHEN TO USE HASH MAP:
✅ When you need to look up data by a unique identifier
✅ When you want O(1) average lookup time (very fast!)
✅ When you have pairs of related data
✅ When you need to count occurrences of items
✅ When you want to cache/store computed results

EXAMPLES OF WHEN TO USE:
- User login system: username → user data
- Shopping cart: product_id → quantity
- Word counter: word → count
- Configuration settings: setting_name → value
- Game scores: player_name → score

BASIC OPERATIONS:
- insert(key, value) - Add/update a key-value pair
- get(key) - Retrieve value by key (returns Option)
- remove(key) - Delete a key-value pair
- contains_key(key) - Check if key exists
- iter() - Loop through all key-value pairs

KEY POINTS:
- Keys must be unique (like primary keys in database)
- Keys must implement Hash + Eq traits
- Common key types: String, &str, numbers, tuples
- Values can be any type
- Order is not guaranteed (unlike Vec)
*/

use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    // EXAMPLE 1: HashMap with custom struct as value
    // Key: i32 (ID number), Value: Contents struct
    let mut map = HashMap::new();
    map.insert(
        1,
        Contents {
            content: "one".to_owned(),
        },
    );
    map.insert(
        2,
        Contents {
            content: "two".to_owned(),
        },
    );
    map.insert(
        3,
        Contents {
            content: "three".to_owned(),
        },
    );

    // EXAMPLE 2: Furniture Store Inventory System
    // Key: &str (furniture type), Value: i32 (stock count)
    // This is like a real inventory management system!
    let mut furniture_store = HashMap::new();
    furniture_store.insert("Chairs", 5); // 5 chairs in stock
    furniture_store.insert("Beds", 3); // 3 beds in stock
    furniture_store.insert("Tables", 2); // 2 tables in stock
    furniture_store.insert("Couches", 0); // 0 couches (out of stock!)

    // ITERATING THROUGH HASH MAP: Calculate total inventory
    let mut total = 0;
    for (key, val) in furniture_store.iter() {
        // NOTE: val is &i32 (reference), so we use *val to get the actual value
        if *val == 0 {
            println!("{} is out of stock!", key); // Alert for empty inventory
        } else {
            println!("{}: {} in stock", key, val);
            total += *val; // Add to running total (*val converts &i32 to i32)
        }
    }

    println!("Total Stock Count: {:?}", total);

    // EXAMPLE 3: Iterating through the first HashMap
    println!("\n=== Contents Map ===");
    for (key, value) in map.iter() {
        println!("ID {}: {}", key, value.content);
    }

    // EXAMPLE 4: Debug print the entire HashMap
    println!("\nFull map contents: {:?}", map);

    // BONUS: Demonstrate common HashMap operations
    println!("\n=== HashMap Operations Demo ===");

    // Check if a key exists
    if furniture_store.contains_key("Chairs") {
        println!("✅ We have chairs in our inventory!");
    }

    // Get a specific value (returns Option<&V>)
    match furniture_store.get("Beds") {
        Some(count) => println!("Beds in stock: {}", count),
        None => println!("Beds not found in inventory"),
    }

    // Update existing value
    furniture_store.insert("Chairs", 10); // Restock chairs!
    println!(
        "Updated chairs stock to: {:?}",
        furniture_store.get("Chairs")
    );

    // ========================================================================
    // DIFFERENT WAYS TO ITERATE THROUGH HASHMAP
    // ========================================================================

    let mut demo_map = HashMap::new();
    demo_map.insert("apple", 5);
    demo_map.insert("banana", 3);
    demo_map.insert("orange", 8);

    println!("\n🔄 === ITERATION METHODS DEMO ===");

    // METHOD 1: .iter() - Iterate over references (&K, &V)
    // Most common method, doesn't consume the HashMap
    println!("\n1️⃣ Using .iter() - References (&key, &value):");
    for (key, value) in demo_map.iter() {
        // key: &&str, value: &i32
        println!("  {} -> {} (types: &&str, &i32)", key, value);
    }
    println!("   ✅ HashMap still exists after iteration");

    // METHOD 2: .iter() with destructuring to get owned values
    println!("\n2️⃣ Using .iter() with destructuring (&key, &value):");
    for (&key, &value) in demo_map.iter() {
        // key: &str, value: i32 (destructured from references)
        println!("  {} -> {} (types: &str, i32)", key, value);
    }

    // METHOD 3: .into_iter() - Takes ownership, consumes HashMap
    // Use this when you don't need the HashMap afterward
    let demo_map_copy = demo_map.clone(); // Clone so we can demo other methods
    println!("\n3️⃣ Using .into_iter() - Owned values (K, V):");
    for (key, value) in demo_map_copy.into_iter() {
        // key: &str, value: i32 (owned values)
        println!("  {} -> {} (types: &str, i32)", key, value);
    }
    println!("   ⚠️  Original HashMap is consumed/moved (can't use demo_map_copy anymore)");

    // METHOD 4: .keys() - Iterate over keys only
    println!("\n4️⃣ Using .keys() - Keys only:");
    for key in demo_map.keys() {
        // key: &&str
        println!("  Key: {} (type: &&str)", key);
    }

    // METHOD 5: .values() - Iterate over values only
    println!("\n5️⃣ Using .values() - Values only:");
    for value in demo_map.values() {
        // value: &i32
        println!("  Value: {} (type: &i32)", value);
    }

    // METHOD 6: .values_mut() - Mutable references to values (for modification)
    println!("\n6️⃣ Using .values_mut() - Mutable values for modification:");
    for value in demo_map.values_mut() {
        // value: &mut i32
        *value *= 2; // Double each value
        println!("  Doubled value: {} (type: &mut i32)", value);
    }

    // METHOD 7: Manual iteration with .get()
    println!("\n7️⃣ Manual iteration using .get() method:");
    let keys_to_check = ["apple", "banana", "orange", "grape"];
    for &key in keys_to_check.iter() {
        match demo_map.get(key) {
            Some(value) => println!("  {} -> {}", key, value),
            None => println!("  {} -> Not found", key),
        }
    }

    // METHOD 8: Functional style with .iter().map()
    println!("\n8️⃣ Functional style with .map() and .collect():");
    let doubled_values: Vec<i32> = demo_map
        .values()
        .map(|&value| value * 2) // Double each value
        .collect();
    println!("  Doubled values: {:?}", doubled_values);

    // METHOD 9: Filtering during iteration
    println!("\n9️⃣ Filtering during iteration:");
    println!("  Items with value > 10:");
    for (key, &value) in demo_map.iter().filter(|(_, &v)| v > 10) {
        println!("    {} -> {}", key, value);
    }

    // METHOD 10: Enumerate for getting index
    println!("\n🔟 Using .enumerate() to get index + key-value:");
    for (index, (key, value)) in demo_map.iter().enumerate() {
        println!("  {}. {} -> {}", index + 1, key, value);
    }

    // ========================================================================
    // SUMMARY: WHEN TO USE EACH METHOD
    // ========================================================================
    println!("\n📋 === WHEN TO USE EACH METHOD ===");
    println!("✅ .iter()        → Most common, when you need both key & value references");
    println!("✅ .into_iter()   → When you're done with HashMap and want owned values");
    println!("✅ .keys()        → When you only need the keys");
    println!("✅ .values()      → When you only need the values");
    println!("✅ .values_mut()  → When you need to modify values in place");
    println!("✅ .get()         → When you want specific keys (not all)");
    println!("✅ .map()/.filter() → Functional programming style");
    println!("✅ .enumerate()   → When you need index numbers");
}
