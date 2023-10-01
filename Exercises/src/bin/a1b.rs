// write a function which returns a addition of two numbers 
// write a function which returns a Multiplication of two numbers 
// write a function which returns a division of two numbers 

fn main() {
    println!("The addition of two number is {}",add(2,3));
    println!("The subraction of two number is {}",sub(2,3));
    println!("The multiplication of two number is {}",mul(2,3));
}

fn add(a:i32,b:i32)->i32{
    a+b
}
fn sub(a:i32,b:i32)->i32{
    a-b
}
fn mul(a:i32,b:i32)->i32{
    a*b
}