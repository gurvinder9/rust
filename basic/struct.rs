// Topic: Organizing similar data using structs
// Concept: Struct definitions, struct fields, combining enums with structs, and accessing struct fields
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Sparkling,
    Sweets
}

struct Drinks {
    flavor: Flavors,
    ounce: f64,
}

fn show_flavor(drinks: Drinks) {

    match drinks.flavor {
        Flavors::Sparkling => println!("Coke"),
        Flavors::Sweets => println!("Juice"),
    }

    println!("ounce: {}", drinks.ounce);
}


fn main() {
    let dr = Drinks {
        flavor: Flavors::Sparkling,
        ounce: 10.00,
    };
    show_flavor(dr);
}

