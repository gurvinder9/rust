// Topic: Working with an enum
// Concept: Enum definitions, enum variants, pattern matching with enums, and function parameters with enums
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    White,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::White => println!("white"),
    }
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn route(go: Direction) {
    match go {
        Direction::UP => println!("UP"),
        Direction::DOWN => println!("DOWN"),
        Direction::LEFT => println!("LEFT"),
        Direction::RIGHT => println!("RIGHT"),
    }
}

fn main() {
    print_color(Color::Red);
    print_color(Color::White);
    route(Direction::UP);
    route(Direction::DOWN);
    route(Direction::LEFT);
    route(Direction::RIGHT);
}
