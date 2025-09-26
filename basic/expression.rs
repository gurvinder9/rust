// Topic: Working with expressions
// Concept: If/else as expressions that return values, boolean variables from conditional expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_msg(msg: &str) {
    match msg {
        ">100" => println!(">100"),
        "<100" => println!("<100"),
        _ => println!("_"),
    }
}

fn main() {
    let num = 100;
    let confirmed = if num > 100 { ">100" } else { "<100" };
    print_msg(confirmed);
}
