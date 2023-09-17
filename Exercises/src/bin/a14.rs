// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

fn main() {
    let mut persons_list = vec![
        Person{
            age:9,
            name:"Mike".to_string(),
            color:"Yellow".to_string()
        },
        Person{
            age:29,
            name:"Kylie".to_owned(),
            color:String::from("Pink")
        },
        Person{
            age:9,
            name:"Kim".to_string(),
            color:"White".to_string()
        },
    ];

    for persons in persons_list{
        if persons.age<10{
            print_(&persons.name,&persons.color);
        }
    }
}

fn print_(name: &str, color: &str) {
    println!("The color is {:?} and name is {:?}", color, name);
}


struct Person{
    age:i32,
    name:String,
    color:String // we should pass the string as Owned string as struct beacuse STRUCT takes ownership of thier field
}