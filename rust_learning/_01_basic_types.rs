// BASIC DATA TYPES IN RUST
// Rust is a statically typed language, meaning it must know the types of all variables at compile time

pub fn run() {
    println!("\n=== BASIC DATA TYPES ===\n");

    // INTEGERS
    // Signed integers: i8, i16, i32, i64, i128, isize (pointer-sized)
    // Unsigned integers: u8, u16, u32, u64, u128, usize (pointer-sized)
    println!("--- Integers ---");
    let x: i32 = 42;              // 32-bit signed integer (default)
    let y: u64 = 100;             // 64-bit unsigned integer
    let z = 10;                    // Type inference (defaults to i32)
    println!("i32: {}, u64: {}, inferred i32: {}", x, y, z);

    // Integer ranges
    println!("i8 range: {} to {}", i8::MIN, i8::MAX);
    println!("u8 range: {} to {}", u8::MIN, u8::MAX);

    // FLOATS
    // f32 (32-bit) and f64 (64-bit, default)
    println!("\n--- Floats ---");
    let a: f64 = 3.14159;          // 64-bit float (default)
    let b: f32 = 2.71828;          // 32-bit float
    println!("f64: {}, f32: {}", a, b);

    // BOOLEAN
    println!("\n--- Boolean ---");
    let is_rust_fun: bool = true;
    let is_difficult = false;
    println!("Is Rust fun? {}, Is it difficult? {}", is_rust_fun, is_difficult);

    // CHARACTERS
    // 4 bytes in size, represents a Unicode Scalar Value
    println!("\n--- Characters ---");
    let letter: char = 'A';
    let emoji: char = '🦀';        // Rust's mascot, Ferris the crab!
    let chinese: char = '中';
    println!("Letter: {}, Emoji: {}, Chinese: {}", letter, emoji, chinese);

    // TUPLES
    // Fixed-size collection of values of different types
    println!("\n--- Tuples ---");
    let person: (&str, i32, f64) = ("Alice", 30, 5.6);
    println!("Name: {}, Age: {}, Height: {}", person.0, person.1, person.2);

    // Destructuring tuples
    let (name, age, height) = person;
    println!("Destructured - Name: {}, Age: {}, Height: {}", name, age, height);

    // ARRAYS
    // Fixed-size collection of values of the same type
    println!("\n--- Arrays ---");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", numbers);
    println!("First element: {}, Length: {}", numbers[0], numbers.len());

    // Array with same value
    let zeros = [0; 3];  // [0, 0, 0]
    println!("Zeros array: {:?}", zeros);

    // TYPE CONVERSION
    println!("\n--- Type Conversion ---");
    let int_value = 10i32;
    let float_value = int_value as f64;  // Explicit casting
    println!("i32: {} -> f64: {}", int_value, float_value);

    // MUTABILITY
    println!("\n--- Mutability ---");
    let immutable = 5;
    // immutable = 6;  // This would cause a compile error!

    let mut mutable = 5;
    println!("Before: {}", mutable);
    mutable = 10;
    println!("After: {}", mutable);

    // CONSTANTS
    // Must be type annotated, always immutable, can be declared in any scope
    const MAX_POINTS: u32 = 100_000;  // Underscores for readability
    println!("\n--- Constants ---");
    println!("Max points: {}", MAX_POINTS);

    // SHADOWING
    // You can declare a new variable with the same name
    println!("\n--- Shadowing ---");
    let value = 5;
    let value = value + 1;  // Shadows the previous value
    let value = value * 2;  // Shadows again
    println!("Shadowed value: {}", value);

    // Shadowing can change type
    let spaces = "   ";
    let spaces = spaces.len();  // Changed from &str to usize
    println!("Number of spaces: {}", spaces);
}
