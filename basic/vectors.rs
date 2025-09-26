// Topic: Vectors
// Concept: Vector creation with vec! macro, for..in loops for iteration, vector methods like .len()
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

struct Test {
    score: i32,
}

fn main() {
    let my_vec = vec![10, 20, 30, 40];

    println!("Total Elem {:?}", my_vec.len());

    for v in my_vec {
        if (v == 30) {
            println!("{:?}", "Thirty")
        } else {
            println!("Num is {:?}", v);
        }
    }

    let my_score: Vec<Test> = vec![Test { score: 200 }, Test { score: 100 }];
    let item_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    println!("{:?}", item_list);

    // let mut groceries = Vec::new();
    // groceries.push(3);
    // groceries.push(5);
    //
    // println!("Groceries List {:?}", groceries);

    for item in item_list {
        println!("{}", item);
    }

    for score in my_score {
        println!("Score is {:?}", score.score)
    }
}
