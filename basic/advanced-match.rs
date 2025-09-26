// Topic: Advanced match
// Concept: Enums with associated data, pattern matching with data extraction, destructuring enum variants
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(String, f64),
    Vip(String, f64),
    Standard,
}

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let ticket = vec![
        Tickets::Backstage("James".to_owned(), 10.00),
        Tickets::Vip("James".to_owned(), 100.00),
        Tickets::Standard,
    ];

    let n = Discount::Flat(3);

    match n {
        Discount::Flat(3) => println!("Discount is three"),
        Discount::Flat(amount) => println!("Discount is {}", amount),
        _ => (),
    }

    let ticket1 = Ticket {
        event: "Concert".to_owned(),
        price: 30,
    };

    match ticket1 {
        Ticket { price: 20, .. } => println!("Price is $20"),
        Ticket { price, .. } => println!("Price is {}", price),
        // Ticket {price, event} => println!("Price is {} for event {}", price, event);
    }

    for t in ticket {
        match t {
            Tickets::Backstage(name, price) => println!("Name is {} and price is ${}", name, price),
            Tickets::Vip(name, price) => println!("Vip Name is {} and price is ${}", name, price),
            other => (),
        }
    }
}
