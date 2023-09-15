// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print


fn main() {
    let num = 120;
    let bool_value =  num>100; // why i have written this 
    // * Because assignment is on expressions
    check_num(bool_value)


}

fn check_num(result:bool){
    match result{
        true=> println!("true"),
        false => println!("false")
    }
}



