// Use a enum with variants as different car names and
//  create a function which accepts an &str, 
// using match statement return the correct enum variant
// define a struct Details 
// car:Car
//price:i32


#[derive(Debug)]
enum Car{
    Mercedes,
    BMW,
    Audi
}

fn car_variant (word:&str)-> Option<Car> {
    match word{
        "Mercedes" =>Some(Car::Mercedes),
        "BMW" =>Some(Car::BMW),
        "Audi" =>Some(Car::Audi),   
        _ =>None,
    }
}

struct Details{
    car:Car,
    price:i32
    
}


fn main(){
    
    let car_name = "BMW";
    let get_car_variant = car_variant(car_name);
    
    if let Some(car) = get_car_variant{
        let car_details = Details{
            car,
            price:2500000
        };
        
        println!("The car name is {:?} ",car_details.car);
        println!("The car price is {:?} ",car_details.price);
    }else{
        println!("Car not found")
    }
    
}