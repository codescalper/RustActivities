// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut count = 5;
    //Why mut?
    //* Since by default variables are immutable and we are decreasing the value by 1 so we'll have to add mut keyword to make it mutable */
    while count > 0 {
        println!("{:?}", count);
        count -= 1;
    }
    println!("done!")
}
