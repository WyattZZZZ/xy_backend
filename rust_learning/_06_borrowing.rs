// BORROWING AND REFERENCES IN RUST
// Instead of transferring ownership, you can borrow a reference to a value
//
// BORROWING RULES:
// 1. At any given time, you can have EITHER:
//    - One mutable reference, OR
//    - Any number of immutable references
// 2. References must always be valid (no dangling references)

pub fn run() {
    println!("\n=== BORROWING AND REFERENCES ===\n");

    // IMMUTABLE REFERENCES (&T)
    println!("--- Immutable References ---");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1 (doesn't take ownership)
    println!("String: '{}', Length: {}", s1, len);  // s1 still valid!

    // MULTIPLE IMMUTABLE REFERENCES
    println!("\n--- Multiple Immutable References ---");

    let s = String::from("Rust");
    let r1 = &s;  // First immutable reference
    let r2 = &s;  // Second immutable reference
    let r3 = &s;  // Third immutable reference

    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);  // All valid!

    // MUTABLE REFERENCES (&mut T)
    println!("\n--- Mutable References ---");

    let mut s = String::from("hello");
    change(&mut s);  // Borrow mutably
    println!("Changed string: {}", s);

    // ONLY ONE MUTABLE REFERENCE AT A TIME
    println!("\n--- Mutable Reference Restriction ---");

    let mut s = String::from("test");
    let r1 = &mut s;  // First mutable reference

    // let r2 = &mut s;  // ERROR! Can't have two mutable references
    // println!("{}, {}", r1, r2);

    println!("Mutable ref: {}", r1);

    // MUTABLE REFERENCE AFTER IMMUTABLE ONES FINISH
    println!("\n--- Reference Scope ---");

    let mut s = String::from("hello");

    let r1 = &s;  // Immutable reference
    let r2 = &s;  // Another immutable reference
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;  // OK! Mutable reference after immutable ones finished
    r3.push_str(" world");
    println!("r3: {}", r3);

    // CANNOT MIX IMMUTABLE AND MUTABLE REFERENCES
    println!("\n--- Cannot Mix References ---");

    let mut s = String::from("test");
    let r1 = &s;  // Immutable reference
    // let r2 = &mut s;  // ERROR! Cannot have mutable ref while immutable exists
    // println!("{}, {}", r1, r2);

    println!("Immutable: {}", r1);

    // DANGLING REFERENCES PREVENTED
    println!("\n--- Dangling References (Prevented by Compiler) ---");
    // let reference = dangle();  // Would cause compile error!
    let valid_ref = no_dangle();
    println!("Valid reference: {}", valid_ref);

    // DEREFERENCING
    println!("\n--- Dereferencing ---");

    let x = 5;
    let y = &x;  // Reference to x

    println!("x: {}, y: {}", x, y);
    println!("Dereferenced y: {}", *y);  // Use * to dereference

    // Comparison requires dereferencing
    if *y == 5 {
        println!("y points to 5");
    }

    // REFERENCES IN STRUCTS
    println!("\n--- References in Structs ---");

    let s = String::from("data");
    let wrapper = StringWrapper { content: &s };
    println!("Wrapped: {}", wrapper.content);

    // SLICES - SPECIAL KIND OF REFERENCE
    println!("\n--- Slices ---");

    let s = String::from("hello world");
    let hello = &s[0..5];   // Slice: reference to part of String
    let world = &s[6..11];
    println!("Full: {}, Part1: {}, Part2: {}", s, hello, world);

    // First word example
    let sentence = String::from("hello world rust");
    let first = first_word(&sentence);
    println!("First word: {}", first);

    // ARRAY SLICES
    println!("\n--- Array Slices ---");

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];  // Reference to middle elements
    println!("Array: {:?}, Slice: {:?}", arr, slice);

    // REFERENCE PATTERNS
    println!("\n--- Reference Patterns ---");

    let point = (3, 5);
    let (x, y) = &point;  // Borrow components
    println!("Point: {:?}, x: {}, y: {}", point, x, y);

    // BORROWING IN LOOPS
    println!("\n--- Borrowing in Loops ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // Immutable borrow in loop
    for num in &numbers {  // Borrow each element
        println!("  {}", num);
    }
    println!("Vector still valid: {:?}", numbers);

    // Mutable borrow in loop
    let mut numbers = vec![1, 2, 3];
    for num in &mut numbers {  // Mutable borrow
        *num *= 2;
    }
    println!("Modified: {:?}", numbers);

    // SELF BORROWING IN METHODS
    println!("\n--- Self Borrowing ---");
    let rect = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect.area());  // Borrows &self
    println!("Original still valid: {}x{}", rect.width, rect.height);

    let mut rect = Rectangle { width: 10, height: 20 };
    rect.double();  // Borrows &mut self
    println!("After doubling: {}x{}", rect.width, rect.height);

    // REFERENCE COERCION
    println!("\n--- Reference Coercion ---");
    let s = String::from("coercion");
    print_str(&s);  // &String coerced to &str
    print_str("literal");  // &str literal
}

// Function that borrows a String
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't own the data, so nothing happens

// Function that mutably borrows
fn change(s: &mut String) {
    s.push_str(", world");
}

// This would create a dangling reference (compiler prevents this!)
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ERROR! s will be dropped, creating dangling reference
// }

// Correct way: return ownership, not reference
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Return ownership
}

// Function to find first word using slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // Return entire string if no space found
}

// Struct with lifetime (simplified version)
struct StringWrapper<'a> {
    content: &'a str,
}

// Struct for method examples
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Immutable borrow of self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Mutable borrow of self
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

// Function demonstrating deref coercion
fn print_str(s: &str) {
    println!("  String: {}", s);
}
