pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Brad", "Mass");

    // Positional argument
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named argument
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic maths
    println!("10 + 10 = {}", 10 + 10);
}
