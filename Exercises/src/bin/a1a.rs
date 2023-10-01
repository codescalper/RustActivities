// Write function which accepts borrowed string and print it, write your first and last name using two different functions


fn main() {
    println!("My first name is {:?} and last name is {:?}",first_name("Mayank"),last_name("Singh"));
}


fn first_name(name:&str)->&str{
    name
}
fn last_name(name:&str)->&str{
    name
}
