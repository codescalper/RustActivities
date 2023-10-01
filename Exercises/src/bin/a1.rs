// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//

// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main(){
    println!("My first name is {:?} and last name is {:?}",first_name("Mayank".to_string()),last_name("Singh".to_string()));
}

fn first_name(name:String)->String{
    name
}

fn last_name(name:String)->String{
    name
}