// Topic: Ownership
// Concept: Ownership and borrowing with references (&), avoiding move semantics by borrowing data
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

/*
RUST OWNERSHIP SYSTEM EXPLAINED

Rust's ownership system is what makes Rust memory-safe without a garbage collector.
It follows three main rules:

1. OWNERSHIP RULES:
   - Each value in Rust has a single owner
   - There can only be one owner at a time
   - When the owner goes out of scope, the value is dropped (memory freed)

2. MOVING VALUES:
   - When you assign a value to another variable or pass it to a function,
     ownership is MOVED (not copied) for types that don't implement Copy trait
   - After a move, the original variable becomes invalid

3. BORROWING:
   - You can "borrow" a value using references (&) without taking ownership
   - Immutable borrows (&T) allow multiple readers
   - Mutable borrows (&mut T) allow one writer, no other readers

EXAMPLE BELOW demonstrates ownership transfer (move):
*/

enum Light {
    Bright,
    Soft
}

struct Book {
    name: &'static str,
    rating: f32,
}

fn display_book(book: Book) {
    println!("Book Name is {:?} and it rating is {:?}", book.name, book.rating);
}

// This function takes OWNERSHIP of the Light value
// Once called, the original variable becomes invalid
fn display_right(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Soft => println!("soft")
    }
    // 'light' is dropped here when function ends
}

// Alternative: This function BORROWS the Light value
// Original variable remains valid after function call
fn display_light_borrowed(light: &Light) {
    match light {
        Light::Bright => println!("bright (borrowed)"),
        Light::Soft => println!("soft (borrowed)")
    }
    // No ownership, so nothing is dropped
}


struct GroceryItem {
    price: f32,
    quantity: i32,
}

enum Name {
    DisplayName(String)
}

fn display_price(item: &GroceryItem) {
    println!("price is {} and quantity is {:?}", item.price, item.quantity);
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity is {}", item.quantity);
}

fn main() {
    let item = GroceryItem {
        price: 12.00,
        quantity: 3,
    };
    let name = Name::DisplayName(String::from("John"));

    display_price(&item);
    display_quantity(&item);
    // display_quantity(item);

        // OWNERSHIP EXAMPLE:
        let light = Light::Bright;          // light owns the Light::Bright value
    
        display_right(light);               // Ownership MOVED to display_right function
        // display_right(light);            // ❌ COMPILE ERROR! light no longer valid
        
        // BORROWING EXAMPLE:
        let light2 = Light::Soft;           // light2 owns the Light::Soft value
        display_light_borrowed(&light2);    // Borrow light2 (ownership stays with light2)
        display_light_borrowed(&light2);    // ✅ Works! light2 still valid
        
        println!("light2 is still accessible here!");
        
        /*
        KEY TAKEAWAYS:
        - Move semantics prevent use-after-free and double-free errors
        - Borrowing allows multiple readers OR one writer (never both simultaneously)
        - Compiler enforces these rules at compile time, preventing runtime crashes
        - No garbage collector needed - memory is freed deterministically
        */
    
        let book = Book {
            name: "Java Programming Language",
            rating: 1.0,
        };
        display_book(book);
}
