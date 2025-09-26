// Topic: Option
// Concept: Option<T> type for handling nullable values, Some/None variants, pattern matching with Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// The Option<T> type in Rust represents a value that might exist (Some(T)) or not exist (None).
// It's Rust's way of handling null values safely, preventing null pointer exceptions.
//
// Use Option<T> when:
// - A function might not return a value (like searching for an item that might not exist)
// - A field might be optional or uninitialized
// - You want to explicitly handle the absence of a value
//
// Pattern matching with match or if let is the idiomatic way to handle Option values:
// - Some(value) => contains a value
// - None => no value present

struct GroceryItem {
    name: String,
    quantity: i32
}

fn display_item(name: &str) -> Option<i32> {
    let item = vec![
        GroceryItem {
            name: "Apple".to_owned(),
            quantity: 10
        },
        GroceryItem {
            name: String::from("Orange"),
            quantity: 15
        }
    ];

    for t in item {
        if t.name == name {
            return Some(t.quantity);
        }
    }

    None

}

/// This three slash comment is called as a documentation comment. cargo doc will generate the documentation of your code.
struct LockerAssignment {
    name: String,
    assigment: Option<i32>
}

fn main() {
    let locker = LockerAssignment {
        name: String::from("John"),
        assigment: Some(10)
    };

    match locker.assigment {
        Some(num) => println!("Locker Assign is {:?}", num),
        None => (),
        _ => ()
    }

    let item = display_item("Apple");
    match item {
        Some(amount) => println!("Grocery Item is {:?}", amount),
        None => (),
        _ => {}
    }
}
