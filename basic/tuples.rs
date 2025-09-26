// Topic: Data management using tuples
// Concept: Tuple creation, tuple destructuring, returning tuples from functions, and tuple element access
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn print_coords() -> (i32, i32) {
    let coords = (4, 5);
    let (x, y) = coords;
    if y > 5 {
        (x, y)
    } else if y < 5 {
        (x, y + 1)
    } else {
        (x, y + 2)
    }
}

fn main() {
    let (x, y) = print_coords();
    println!("x: {}, y: {}", x, y);
    let coord = (1, 2);
    let (x, y) = coord;
    println!("{:?}", coord);
    println!("{:?}", coord.0);
    println!("{:?}", coord.1);
    println!("{:?}", x);
    println!("{:?}", y);
}
