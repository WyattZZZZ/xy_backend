// STRINGS IN RUST
// Two main string types: &str (string slice) and String (owned string)
// Understanding the difference is crucial for writing Rust code

pub fn run() {
    println!("\n=== STRINGS ===\n");

    // STRING SLICE (&str)
    // - Immutable reference to string data
    // - Stored on the stack (just a pointer and length)
    // - String literals are &str
    println!("--- String Slices (&str) ---");
    let greeting: &str = "Hello, World!";  // String literal
    println!("String slice: {}", greeting);

    // STRING (String)
    // - Growable, heap-allocated string
    // - Owns its data
    // - Can be modified
    println!("\n--- String Type (String) ---");
    let mut owned_string = String::from("Hello");
    println!("Original: {}", owned_string);

    owned_string.push_str(", Rust!");  // Append string slice
    owned_string.push('!');             // Append single character
    println!("Modified: {}", owned_string);

    // CREATING STRINGS
    println!("\n--- Creating Strings ---");
    let s1 = String::from("Using from");
    let s2 = "String literal".to_string();
    let s3 = String::new();  // Empty string
    println!("s1: {}, s2: {}, s3 is empty: {}", s1, s2, s3.is_empty());

    // STRING CONCATENATION
    println!("\n--- String Concatenation ---");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");

    // Using + operator (takes ownership of s1, borrows s2)
    let s3 = s1 + &s2;  // s1 is moved here and can no longer be used
    println!("Concatenated: {}", s3);
    // println!("{}", s1);  // This would cause an error - s1 was moved!

    // Using format! macro (doesn't take ownership)
    let s4 = String::from("Rust");
    let s5 = String::from("Programming");
    let s6 = format!("{} {}", s4, s5);
    println!("Using format!: {}", s6);
    println!("s4 still valid: {}, s5 still valid: {}", s4, s5);

    // STRING SLICING
    println!("\n--- String Slicing ---");
    let text = String::from("Hello, Rust!");
    let hello = &text[0..5];   // First 5 bytes
    let rust = &text[7..11];    // Bytes 7-10
    println!("Full: {}, Slice1: {}, Slice2: {}", text, hello, rust);

    // Be careful with UTF-8! Slicing in the middle of a character will panic
    // let bad = &text[0..1];  // This might panic if first char is multi-byte!

    // STRING ITERATION
    println!("\n--- String Iteration ---");

    // Iterating over characters
    print!("Characters: ");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();

    // Iterating over bytes
    print!("Bytes: ");
    for b in "Hello".bytes() {
        print!("{} ", b);
    }
    println!();

    // STRING METHODS
    println!("\n--- String Methods ---");
    let text = String::from("  Rust Programming  ");

    println!("Length: {}", text.len());
    println!("Trimmed: '{}'", text.trim());
    println!("Uppercase: {}", text.to_uppercase());
    println!("Lowercase: {}", text.to_lowercase());
    println!("Contains 'Rust': {}", text.contains("Rust"));
    println!("Starts with '  Rust': {}", text.starts_with("  Rust"));
    println!("Replace: {}", text.replace("Rust", "Go"));

    // Split strings
    println!("\n--- Splitting Strings ---");
    let data = "apple,banana,cherry";
    for item in data.split(',') {
        println!("  - {}", item);
    }

    // Split whitespace
    let sentence = "Rust is awesome";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Words: {:?}", words);

    // STRING CAPACITY
    println!("\n--- String Capacity ---");
    let mut s = String::with_capacity(10);  // Pre-allocate capacity
    println!("Capacity: {}, Length: {}", s.capacity(), s.len());

    s.push_str("hello");
    println!("After 'hello' - Capacity: {}, Length: {}", s.capacity(), s.len());

    s.push_str(" world and more text");
    println!("After more text - Capacity: {}, Length: {}", s.capacity(), s.len());

    // CONVERSION BETWEEN &str AND String
    println!("\n--- Conversion ---");
    let string = String::from("owned");
    let slice: &str = &string;  // String to &str (cheap, just a reference)
    println!("String: {}, Slice: {}", string, slice);

    let slice2 = "borrowed";
    let string2: String = slice2.to_string();  // &str to String (allocates)
    println!("Slice: {}, String: {}", slice2, string2);

    // FORMATTING
    println!("\n--- String Formatting ---");
    let name = "Alice";
    let age = 30;
    let formatted = format!("Name: {}, Age: {}", name, age);
    println!("{}", formatted);

    // Positional arguments
    let formatted2 = format!("{0} likes {1}, and {0} also likes {2}", "Bob", "Rust", "coding");
    println!("{}", formatted2);

    // Named arguments
    let formatted3 = format!("{name} is {age} years old", name="Charlie", age=25);
    println!("{}", formatted3);
}
