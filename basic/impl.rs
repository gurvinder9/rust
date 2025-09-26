// Topic: Implementing functionality with the impl keyword
// Concept: Associated functions and methods using impl blocks, Self keyword, constructor patterns
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// RUST CONCEPTS EXPLAINED:
//
// impl: Implementation block - used to define methods and associated functions for any type
//       Can be used with structs, enums, traits, and even primitive types
//       Format: impl TypeName { ... } or impl TraitName for TypeName { ... }
//
// Self: Type alias that represents the type being implemented in an impl block
//       Always refers to the concrete type, making code more maintainable
//       Used in return types, parameters, and when creating instances
//
// &self: Immutable borrowed reference to the current instance in instance methods
//        Other variations: self (takes ownership), &mut self (mutable reference)
//        Allows methods to access instance data without consuming the instance

struct Temperature {
    celsius: f32,
}

// impl: Implementation block - defines methods and associated functions for the Temperature struct
impl Temperature {
    // Associated function (like a static method) - called on the type itself, not an instance
    // Self: Refers to the type being implemented (Temperature in this case)
    fn freezing() -> Self {
        Self { celsius: 20.0 } // Self is shorthand for Temperature here
    }

    // Instance method - takes &self (immutable reference to the instance)
    // &self: Borrowed reference to the current instance, allows reading data without taking ownership
    fn celsius_to_fahrenheit(&self) -> f32 {
        self.celsius * 9.0 / 5.0 + 32.0 // self refers to the current Temperature instance
    }

    // Another instance method using &self
    fn display_temp(&self) {
        println!("{:?}", self.celsius); // Access the celsius field of this instance
    }
}

enum BoxColor {
    Red,
    White,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Red => println!("red"),
            BoxColor::White => println!("white"),
        }
    }
}

struct ShippingBox {
    weight: f32,
    dimensions: (f32, f32),
    color: BoxColor,
}

impl ShippingBox {
    fn new() -> Self {
        Self {
            weight: 20.00,
            dimensions: (20.00, 12.00),
            color: BoxColor::Red,
        }
    }
    fn print(&self) {
        self.color.print();
    }
}

fn main() {
    let shipping = ShippingBox {
        weight: 10.00,
        dimensions: (10.00, 34.00),
        color: BoxColor::White,
    };
    let small_box = ShippingBox::new();
    small_box.print();
    shipping.print();
    let t1 = Temperature { celsius: 32.0 };
    t1.display_temp();
    println!("Converting to fahrenheit {:?}", t1.celsius_to_fahrenheit());

    let cold = Temperature::freezing();
    cold.display_temp();
}
