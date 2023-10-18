// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum Color{
    Yellow,
    Orange, 
    Red
}

fn print_color_name(color:Color){
    match color{
        Color::Yellow =>println!("Yellow"),
        Color::Orange =>println!("Orange"),
        Color::Red =>println!("Red"),
        _ =>println!("No color found"),
    }
}




fn main() {

    print_color_name(Color::Yellow)
}
 
