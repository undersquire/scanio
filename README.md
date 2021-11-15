# scanio
**RFC: https://github.com/rust-lang/rfcs/pull/3183**

This crate is the testing/WIP implementation of some simple macros for generic text input scanning (to accompany the `print` family of macros).

This crate currently implements four macros, `scan!`, `try_scan!`, `read!` and `try_read!`.

## The implementation for these macros is experimental.

`scan!` usage:

```rust
#[macro_use]
extern crate scanio;

fn main() {
    let name: String;
    let age: u8;
    
    // reads a String into `name` and a u8 into `age`
    // if it fails, it will simply assign Default::default()
    scan!("{} {}", name, age);
    
    println!("{} of age {}", name, age);
}
```

`try_scan!` usage:

```rust
#[macro_use]
extern crate scanio;

fn main() {
    // returns a Result<(String, u8), ()>, which we unwrap
    let person = try_scan!("{} {}", String, u8).expect("Invalid input!");
    
    // ideally you should `match` on the result but this is an example so :shrug:
    
    println!("{} of age {}", person.0, person.1);
}
```

The `read!` and `try_read!` macros work exactly the same, however their first argument must be a mutable reference to an object that implements the `std::io::Read` trait, such as `std::io::stdin()` or a `File`.
