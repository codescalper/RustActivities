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

enum Color {
    Red,
    White,
    Yellow,
}
fn color_name(color: Color) {
    match color {
        Color::Red => println!("Its red"),
        Color::White => println!("Its White"),
        Color::Yellow => println!("Its Yellow"),
        _ => println!("Any other color"),
    }
}
fn main() {
    color_name(Color::Red);
}
