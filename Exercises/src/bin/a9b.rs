// Create struct Tuple and try to print the data 

// Define a struct named Tuple
struct Tuple {
    x: i32,
    y: f64,
}

fn main() {
    // Create an instance of the Tuple struct
    let my_tuple = Tuple {
        x: 42,
        y: 3.14,
    };

    // Access and print the data in the struct
    println!("x: {}", my_tuple.x);
    println!("y: {}", my_tuple.y);
}


