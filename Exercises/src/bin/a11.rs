// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter




fn main() {

    let items =Grocery{
        item:String::from("Tomato"),
        id:3,
        quantity:123
    };
   
    println!("The quantity of item is {}",display_quantity(&items));//If i would have passed the value without reference i could have not be able to use it in the next function call 
    println!("The id of item is {}",display_id(&items)); //Referecne is borrowing the value not actually moving it 

}

struct Grocery{
    item:String,
    id:i32,
    quantity:i32
}

fn display_quantity(grocery:&Grocery)->i32{ // Pasing the data type as reference struct 
    grocery.quantity
}

fn display_id(grocery:&Grocery)->i32{ 
    grocery.id
}