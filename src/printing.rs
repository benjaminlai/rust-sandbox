pub fn run() {
    // Printing to console
    println!("Hello World");

    // Named Arguments
    println!("Hello {name}", name = "Named!");

    // String interpolation
    println!("Hello {}", "Formating!");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 20, 10);

    // Placeholder debug traits
    println!("{:?}", (10, true, "hello"));

    // Positional Arguments
    println!("Hello {0} from the {0}", "World!");
}
