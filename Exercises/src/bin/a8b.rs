//Topic: Getting used to with Struct 

//
//Requirements
//
/*
1- Define a struct Book having two fields author:Author and price:i32, 
2- Define a struct Author having two fields one is name and stars, 
3- you gonna purchase a book only if author has rating >= 4.9 stars .

*/

#[derive(Debug)]
struct Book{
    author:Author,
    price:i32
}


#[derive(Debug)]
struct Author{
    name:String,
    stars:f32
}

fn purchase_book(book:Book) {
    if book.author.stars >=4.9 {
        println!("Purchasing this book");
        
    }else{
    println!("Not purchasing this book")
        
    }
}



fn main() {
    
    let author1 = Author{
           name:"Mayank".to_string(),
           stars:5.2
    };
    let author2 = Author{
           name:"Elvish".to_string(),
           stars:4.2
    };
    
    let book1 = Book{
      author:author1,
        price:100
    };
    let book2 = Book{
        author:author2,
        price:200
    };
    
    purchase_book(book1);
    purchase_book(book2);
    
    
    

}

