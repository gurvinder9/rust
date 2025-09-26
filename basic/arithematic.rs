// Topic: Basic arithmetic
// Concept: Basic arithmetic operations, if/else control flow, and debug printing with {:?}
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let age = 15;
    if age > 20 {
        println!("What would you like to drink?");
    } else {
        println!("Sorry, you don't have enough years?");
    }
    let msg = true;
    if msg {
        println!("hello");
    } else {
        println!("goodbye");
    }

    println!("Sums is {:?}", add(1, 2));
}
