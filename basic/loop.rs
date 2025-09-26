// Topic: Looping using the loop statement
// Concept: Infinite loops with explicit break condition, mutable variables, and loop control flow
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

// Topic: Looping using the while statement
// Concept: Conditional loops with while statements, condition-based loop termination
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut num = 1;
    loop {
        println!("{:?}", num);
        if num == 4 {
            break;
        }
        num += 1;
    }

    let mut count = 5;
    while count > 0 {
        println!("{:?}", count);
        count -= 1;
    }
}
