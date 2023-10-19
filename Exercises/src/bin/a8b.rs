// Topic: Getting Used to with struct
//
//Requirements:
/*
 1-Define  struct Shape having  square:Square and rectangle:Rectangle 
 2-Square has one field as side:i32
 3-Rectangle has two length:i32 and width:i32
 4-Create a function which returns a new Rectangle
 5-Create a function which returns a new Square
 6-Create a function which returns a new Shape 
 7-Create a function which takes Shape and prints the dimension of all shapes 
*/
#[derive(Debug,Copy,Clone)] //Added this so that structs can be used multiple time without facing move error
struct Shape{
    square:Square,
    rectangle:Rectangle
}

#[derive(Debug,Copy,Clone)]
struct Square{
    side:i32
}

#[derive(Debug,Copy,Clone)]
struct Rectangle{
    length:i32,
    breadth:i32
}

fn new_rectangle (l:i32,b:i32) -> Rectangle{
    let rect = Rectangle{
        length:l,
        breadth:b
    };
    
    return rect
}

fn new_square (s:i32) -> Square{
    let sq = Square{
        side:s
    };
    
    return sq
}


fn new_shape(sq:Square,rect:Rectangle) ->Shape{
    let shape = Shape{
        square:sq,
        rectangle:rect
    };
    return shape
}

fn print_dimension(shape:Shape){
    println!("The dimension of the square are {:?} the dimension of rectangle are{:?} {:?}",shape.square.side,shape.rectangle.length,shape.rectangle.breadth);
}




fn main(){

    let print_square = new_square(12);
    let print_rect = new_rectangle(10,12);
    let sq = Square{
        side:12
    };
    let rect = Rectangle{
        length:12,
        breadth:10
    };
    
    let shape = Shape{
        square : sq,
        rectangle:rect
    };
    let print_shape = new_shape(sq,rect);
    
    
    println!("{:?}",print_square);
    println!("{:?}",print_rect);
    println!("{:?}",print_shape);
    
    print_dimension(shape);

    
}