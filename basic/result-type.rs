/*

Concept: Result<T, E> type for handling errors, Ok/Err variants, pattern matching with Result

OK(variable_name)
The Ok variant is used to return a value when the operation is successful.

ERR(variable_name)
The Err variant is used to return an error when the operation is unsuccessful.

Useful when working with functionality that can potentially return an error.

*/

fn get_locker_assignment(name: &str) -> Result<Option<i32>, String> {
    if name == "John" {
        Ok(Some(10))
    } else {
        Err("Student not found".to_string())
    }
}

struct LockerAssignment {
    name: String,
    assigment: Option<i32>,
}

#[derive(Debug)] // This allows us to print MenuChoice with {:?}
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice".to_string()),
    }
}

// The ? operator (Question Mark Operator)
// ================================================================
//
// The ? operator is Rust's shorthand for handling Result types elegantly.
// It's called the "try operator" and does two things:
//
// 1. If the Result is Ok(value) -> extracts the value and continues
// 2. If the Result is Err(error) -> immediately returns the error from the function
//
// EXAMPLE: What happens with get_choice(input)?
//
// Without ? operator (the long way):
// let choice = match get_choice(input) {
//     Ok(menu_choice) => menu_choice,     // Extract the MenuChoice if successful
//     Err(error) => return Err(error),    // Return error immediately if failed
// };
//
// With ? operator (the short way):
// let choice = get_choice(input)?;  // Does exactly the same thing!
//
// REAL EXAMPLES:
// - If input = "start" -> get_choice returns Ok(MenuChoice::Start)
//   The ? extracts MenuChoice::Start and assigns it to choice
//   Function continues to println!
//
// - If input = "invalid" -> get_choice returns Err("Invalid choice")
//   The ? immediately returns Err("Invalid choice") from pick_choice
//   println! never executes!
//
// WHY USE ? :
// - Much cleaner code (1 line vs 5 lines)
// - Automatic error propagation up the call stack
// - Only works in functions that return Result<T, E> or Option<T>
//
// KEY POINT: The ? operator is "early return" for errors!
// If there's an error, it immediately exits the function with that error.
// If there's success, it unwraps the value and continues.
fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?; // ? extracts MenuChoice or returns error
    println!("User choice is {:?}", choice); // This line only runs if ? succeeded!
    Ok(()) // Return success if we got here (no errors occurred)
}

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age > 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("Sorry you're under age")
        }
    }
}

fn print_msg(age: u8, name: &str) -> Result<(), String> {
    let choice = Adult::new(age, name)?;
    println!("Child is {:?}", choice.name);
    Ok(())
}

fn main() {
    let locker = LockerAssignment {
        name: String::from("John"),
        assigment: Some(10),
    };

    // Original examples using match (the verbose way)
    let user_choice = get_choice("mainmenu");
    match user_choice {
        Ok(choice) => println!("User choice is {:?}", choice),
        Err(e) => println!("Error: {:?}", e),
    }
    let user_choice1 = get_choice("start");
    match user_choice1 {
        Ok(choice) => println!("User choice is {:?}", choice),
        Err(e) => println!("Error: {:?}", e),
    }
    let user_choice2 = get_choice("quit");
    match user_choice2 {
        Ok(choice) => println!("User choice is {:?}", choice),
        Err(e) => println!("Error: {:?}", e),
    }
    let locker1 = get_locker_assignment("John");
    match locker1 {
        Ok(Some(num)) => println!("Locker Assign is {:?}", num),
        Ok(None) => println!("Locker Assign is None"),
        Err(e) => println!("Error: {:?}", e),
    }

    println!("\n=== DEMONSTRATING THE ? OPERATOR ===");

    // Example 1: Valid input - ? operator extracts the value
    println!("Testing with valid input 'start':");
    match pick_choice("start") {
        Ok(()) => println!("✅ pick_choice succeeded!"),
        Err(e) => println!("❌ pick_choice failed: {}", e),
    }

    // Example 2: Invalid input - ? operator propagates the error
    println!("\nTesting with invalid input 'invalid_choice':");
    match pick_choice("invalid_choice") {
        Ok(()) => println!("✅ pick_choice succeeded!"),
        Err(e) => println!("❌ pick_choice failed: {}", e),
    }

    println!("\n💡 Notice: With invalid input, the println! inside pick_choice never executed!");
    println!("   The ? operator returned the error immediately!");

    match print_msg(10, "Sam") {
        Ok(()) => println!("✅ print_msg succeeded for Sam!"),
        Err(e) => println!("❌ print_msg failed for Sam: {}", e),
    }

    match print_msg(23, "John") {
        Ok(()) => println!("✅ print_msg succeeded for John!"),
        Err(e) => println!("❌ print_msg failed for John: {}", e),
    }
}
