// Variables hold primitive data or references to data
// Variablesare immutable by default
// Rust is block scoped language

pub fn run() {
    let name = "Ben";
    let mut mutable_value = 10;

    println!("Hello {} Age {}", name, mutable_value);
    // attempt to change
    mutable_value = 11;
    println!("Hello {} Age {}", name, mutable_value);

    // trial constants
    const UNCHANGING: &str = "value";

    println!("Const value {}", UNCHANGING)
}
