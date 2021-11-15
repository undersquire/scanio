#[macro_use]
extern crate scanio;

#[test]
fn main() {
    println!("input any number:");

    let num: i32;

    scan!(num);

    println!("You entered: {}", num);

    //

    println!("input your name and age:");

    let name: String;
    let age: u8;

    scan!("{} {}", name, age);

    println!("Hello {}! You are {} years old!", name, age);

    //

    println!("input the current year:");

    let year = try_scan!(u16).expect("Invalid year!");

    println!("It is currently the year {}!", year);

    //

    println!("input someone else's name and age:");

    let person = try_scan!("{} {}", String, u8).expect("Invalid input!");

    println!("Name: {}, Age: {}", person.0, person.1);
}
