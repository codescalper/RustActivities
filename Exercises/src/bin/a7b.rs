//Topic:Working with enum
//
//

/* 
1-Define a Enum Phones with some brands of phones, 
2-define an enum Headsets with some brands  , 
3-define a struct Shop having two fields Phones and Headsets, 
4-create instance of Shop and print which phone and headsets you have picked 
*/


#[derive(Debug)]
enum Phone{
    Iphone,
    Samsung,
    OnePlus
}

#[derive(Debug)]
enum Headset{
    Sony,
    Jbl,
    Boat
}
#[derive(Debug)]
struct Shop{
    phone:Phone,
    headset:Headset
}

fn main(){


    let shop = Shop{
        phone:Phone::Samsung,
        headset:Headset::Sony

    };

    println!("{:?}",shop)

}




