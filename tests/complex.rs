#[macro_use]
extern crate scanio;

#[test]
fn complex() {
    println!("complex! enter your name and age:");

    let person = try_scan!("name: {}, age: {}", String, u8).expect("Invalid input!");

    println!("Hello {}! You are {} years old!", person.0, person.1);
}
