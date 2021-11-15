#[macro_use]
extern crate scanio;

#[test]
fn basic() {
    let age = try_scan!(u8).unwrap();

    println!("{}", age);
}

#[test]
fn basic_read() {
    let age: u8;

    scan!(age);

    println!("{}", age);
}

#[test]
fn read_to_tuple() {
    let person = try_scan!("{} {}", String, u8).unwrap();

    println!("{} of age {}", person.0, person.1);
}

#[test]
fn declare_and_read() {
    let name: String;
    let age: u8;

    scan!("{} {}", name, age);

    println!("{} of age {}", name, age);
}
