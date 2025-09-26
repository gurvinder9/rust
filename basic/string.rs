// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

/*
 * Owned String vs Borrowed String (&str) in Rust
 *
 * String (Owned):
 * - Heap-allocated, growable, mutable
 * - Use when you need to modify, own, or build strings dynamically
 * - Created with String::new(), String::from(), .to_string(), or format!()
 * - Example: let mut s = String::from("hello"); s.push_str(" world");
 *
 * &str (String Slice/Borrowed):
 * - Reference to string data, immutable view
 * - Use for function parameters, reading existing strings, string literals
 * - More efficient - no allocation, just points to existing data
 * - String literals ("hello") are &str by default
 * - Example: fn process_text(text: &str) { ... }
 *
 * General Rule:
 * - Function parameters: Use &str (accepts both String and &str via deref coercion)
 * - Return types: Use String if creating new data, &str if returning slice of input
 * - Storage: Use String when you own the data, &str when borrowing
 *
 * Example:
 * fn greet(name: &str) -> String {  // Take &str, return owned String
 *     format!("Hello, {}!", name)   // Creates new owned String``
 * }
 */

 struct LineItem {
    name: String,
    count: i32
}

struct People {
    color: String,
    name: String,
    age: i32
}

fn print_name(n: &str) {
    println!("Name is {:?}", n);
}

fn print_color(n: &str) {
    println!("Name is {:?}", n);
}

fn print(data: String) {
    println!("{:?}", data);
}

fn print_receipt(name: &str) {
    println!("{:?}", name)
}

// fn print(name: String) {
    println!("Printing thought string owned approach {:?}", name);
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "Hello".to_owned(),
            count: 10
        },
        LineItem {
            name: String::from("Hello through from"),
            count: 5
        }
    ];

    let people = vec![
        People {
            color: "Red".to_owned(),
            name:"John".to_owned(),
            age: 10
        },
        People {
            color: "White".to_owned(),
            name: String::from("Sam"),
            age: 12
        },
        People {
            color: "Blue".to_owned(),
            name:"Rita".to_owned(),
            age: 15
        }
    ];

    for p in people {

        if p.age <= 10 {
            print_name(&p.name);
            print_color(&p.color);
            print(p.name)
        }
    }

    for r in receipt {
        print_receipt(&r.name);
        print(r.name)
    }
}
