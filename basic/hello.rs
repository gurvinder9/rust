// Topic: Functions
// Concept: Basic function definitions, function parameters, return types, and calling functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    let sum = 3 + 3;
    let subtract = 10 - 5;
    let division = sum / subtract;
    let multiplier = 5 * 5;
    let remainder = sum % subtract;

    fn add(a:i32, b:i32) -> i32 {
        a + b
    }

    fn display_name() -> String {
        "John Doe".to_owned()
    }
    println!("My name is {}", display_name())
}

