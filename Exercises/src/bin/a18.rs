// Topic: Result
//
// Requirements:
// * Determine if a customer is able to enter in  a theater 
// * Restricted entrns  require that the age of the customer
//   is at least 21 
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted entry
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase






fn main() {
    let customer = Customer {
        age: 22,
    };

    let result: Result<String, String> = entry(customer);
    println!("{:?}", result);
}

struct Customer {
    age: i32,
}

fn entry(customer: Customer) -> Result<String, String> {
    if customer.age >= 21 {
        Ok("The user is allowed since age is at least 21".to_string())
    } else {
        Err("The user is not allowed as customer age is below 21".to_string())
    }
}
