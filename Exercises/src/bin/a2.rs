// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    println!("The sum of the two number is {:?}",sum(2,3));
}

fn sum(a:i32,b:i32)->i32{
    a+b
}
