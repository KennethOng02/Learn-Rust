// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string
// data
//
pub fn run() {
    // Primitive str
    let hey = "hey"; 

    // String
    let mut hello = String::from("Hello ");

    // push: for char type
    hello.push('W');

    // push_str: for String type
    hello.push_str("orld!");

    println!("{}: {}, {}: {}", hello, hello.len(), hey, hey.len());

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check empty
    println!("Is empty: {}", hello.is_empty());

    // Check contains some substring
    println!("Contains world: {}", hello.contains("World"));

    // Replace 
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through String by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create String with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{} {} {}", s, s.len(), s.capacity());

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}
