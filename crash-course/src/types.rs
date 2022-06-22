// Primitive types
// Integer: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
// Floats: f32, f64
// Boolean: (bool)
// Character: (char)
// Tuples
// Arrays (fixed size)
//
// Rust is a statically typed language, which means that it must know every type of variable at
// compile time
// However, compiler can usually infer what type the variable is based on the value and how we use
// it

pub fn run() {
    // Defaults: i32
    let x = 1;

    // Defaults: f64
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4567890;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 < 5;

    // Character
    let a1 = 'a';
    // All unicode is a character
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
