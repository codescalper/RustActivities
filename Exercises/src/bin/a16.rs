// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32> // locker number




fn main() {
    let stud = Student{
        name:"John".to_string(),
        locker_assignment:Some(5),
    };

    match stud.locker_assignment{
        Some(data) =>{
            println!("{}",data)
        },
        None => {
            println!("THe data is not available")
        }
    }

    
}

struct Student{
    name:String,
    locker_assignment:Option<i32>
}

