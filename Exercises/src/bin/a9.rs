// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print


fn main() {

    //Now to destructure it i can create a variable and store the value os tuple in it 
    
        let (_x,y)=get_cartesian_coordinates();// _x because in rust if we don;t use a variable we can write a underscore to it    
        if y>5{
            println!("y is greater than 5");
        }else if y<5{
            println!("y is smaller than 5")
        }else{
            println!("y is equal to 5")
        }
    
    }
    
    
    
    fn get_cartesian_coordinates() -> (i32, i32) {
        // This function returns a tuple of two integers representing (x, y) coordinates.
        (3, 5)
    }