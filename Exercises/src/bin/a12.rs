// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


fn main() {
    let dim = Dimensions {
        length: 11.2,
        breadth: 22.3,
        depth: 33.4,
    };

    let new_box = Box::new(dim, 65.4, Color::White);

    new_box.print_data();
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color, 
}

struct Dimensions {
    length: f64,
    breadth: f64,
    depth: f64,
}

impl Box {
    fn new(dim: Dimensions, w: f64, rang: Color) -> Self {
        Self {
            dimensions: dim,
            weight: w,
            color: rang, // rang is color in hindi
        }
    }

    fn print_data(&self) {
        let box_color = match self.color {
            Color::White => "White",
            Color::Black => "Black",
            Color::Green => "Green",
        };
        println!(
            "The dimensions are {} {} {}, the weight is {} and the color is {}",
            self.dimensions.length,
            self.dimensions.breadth,
            self.dimensions.depth,
            self.weight,
            box_color
        );
    }
}

enum Color {
    White,
    Black,
    Green,
}
