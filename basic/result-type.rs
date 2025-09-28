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

fn main() {
    let locker = LockerAssignment {
        name: String::from("John"),
        assigment: Some(10),
    };
    let locker1 = get_locker_assignment("John");
    match locker1 {
        Ok(Some(num)) => println!("Locker Assign is {:?}", num),
        Ok(None) => println!("Locker Assign is None"),
        Err(e) => println!("Error: {:?}", e),
    }
}
