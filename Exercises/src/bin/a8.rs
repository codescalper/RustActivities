// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


fn main() { 

    let orange_drink = Drinks{
        flavor:Flavor::Orange,
        fluid:12
    };
    let mango_drink = Drinks{
        flavor:Flavor::Mango,
        fluid:15
    };
    
    print_drink_info(orange_drink);
    print_drink_info(mango_drink);
    
    }
    
    enum Flavor{
        Orange,
        Mango,
        Pineapple,
        Guava
    }
    
    struct Drinks{
        flavor:Flavor,
        fluid:i32
    }
    
    fn print_drink_info(drink: Drinks) {
        match drink.flavor { //Since match is exhaustive we will have to use all th testcases
            Flavor::Orange => println!("Flavor: Orange, Fluid Ounces: {}", drink.fluid),
            Flavor::Mango => println!("Flavor: Mango, Fluid Ounces: {}", drink.fluid),
            Flavor::Pineapple => println!("Flavor: Pineapple, Fluid Ounces: {}", drink.fluid),
            Flavor::Guava => println!("Flavor: Guava, Fluid Ounces: {}", drink.fluid),
        }
    }
    
    