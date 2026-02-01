// OWNERSHIP IN RUST
// Ownership is Rust's most unique feature and enables memory safety without a garbage collector
//
// OWNERSHIP RULES:
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value is dropped (memory freed)

pub fn run() {
    println!("\n=== OWNERSHIP ===\n");

    // STACK VS HEAP
    println!("--- Stack vs Heap ---");
    // Stack: fixed size, LIFO, fast
    let x = 5;  // Stored on stack
    let y = x;  // Copied on stack (i32 implements Copy trait)
    println!("x = {}, y = {} (both valid - Copy trait)", x, y);

    // Heap: dynamic size, slower, managed by ownership
    let s1 = String::from("hello");  // Allocated on heap
    let s2 = s1;  // s1 is MOVED to s2 (not copied!)
    // println!("{}", s1);  // ERROR! s1 is no longer valid
    println!("s2 = {} (s1 moved to s2)", s2);

    // OWNERSHIP TRANSFER (MOVE)
    println!("\n--- Ownership Transfer (Move) ---");
    let s1 = String::from("Rust");
    println!("Before move: s1 = {}", s1);

    let s2 = s1;  // Ownership moves from s1 to s2
    println!("After move: s2 = {}", s2);
    // println!("s1 = {}", s1);  // ERROR! s1 no longer valid

    // CLONE - deep copy
    println!("\n--- Clone (Deep Copy) ---");
    let s1 = String::from("Programming");
    let s2 = s1.clone();  // Explicit deep copy
    println!("s1 = {}, s2 = {} (both valid after clone)", s1, s2);

    // COPY TRAIT
    println!("\n--- Copy Trait ---");
    // Types that implement Copy trait are copied, not moved
    // Integers, floats, booleans, char, tuples of Copy types

    let x = 42;
    let y = x;  // Copy, not move
    println!("x = {}, y = {} (both valid)", x, y);

    // OWNERSHIP AND FUNCTIONS
    println!("\n--- Ownership and Functions ---");

    let s = String::from("hello");
    takes_ownership(s);  // s is moved into function
    // println!("{}", s);  // ERROR! s is no longer valid

    let x = 5;
    makes_copy(x);  // x is copied (i32 has Copy trait)
    println!("x still valid: {}", x);  // OK! x is still valid

    // RETURN VALUES AND SCOPE
    println!("\n--- Return Values and Scope ---");

    let s1 = gives_ownership();  // Function gives ownership
    println!("Received ownership: {}", s1);

    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);  // s2 moved in, returned as s3
    // println!("{}", s2);  // ERROR! s2 moved
    println!("Got back: {}", s3);

    // RETURNING MULTIPLE VALUES
    println!("\n--- Returning Multiple Values ---");

    let s1 = String::from("test");
    let (s2, len) = calculate_length_and_return(s1);
    println!("String: {}, Length: {}", s2, len);

    // SCOPE AND DROP
    println!("\n--- Scope and Drop ---");
    {
        let s = String::from("inner scope");
        println!("  Inside scope: {}", s);
    }  // s goes out of scope here, memory is freed (Drop is called)
    // println!("{}", s);  // ERROR! s no longer exists

    // OWNERSHIP WITH VECTORS
    println!("\n--- Ownership with Vectors ---");

    let v1 = vec![1, 2, 3];
    let v2 = v1;  // Ownership moved
    // println!("{:?}", v1);  // ERROR! v1 no longer valid
    println!("v2: {:?}", v2);

    // OWNERSHIP WITH STRUCTS
    println!("\n--- Ownership with Structs ---");

    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
    };

    // Moving the entire struct
    let user2 = user1;  // user1 moved to user2
    // println!("{}", user1.username);  // ERROR! user1 moved
    println!("user2: {}", user2.username);

    // PARTIAL MOVES
    println!("\n--- Partial Moves ---");

    let user3 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
    };

    let username = user3.username;  // Partial move: username moved out
    // println!("{}", user3.username);  // ERROR! username moved
    println!("Email still valid: {}", user3.email);  // OK! email not moved

    // AVOIDING MOVES
    println!("\n--- Avoiding Moves ---");
    println!("1. Clone the data");
    println!("2. Use references (borrowing) - see next module!");
    println!("3. Use types that implement Copy");
}

// Function that takes ownership
fn takes_ownership(s: String) {
    println!("  Took ownership: {}", s);
}  // s goes out of scope and is dropped here

// Function that copies value
fn makes_copy(x: i32) {
    println!("  Made copy: {}", x);
}

// Function that gives ownership
fn gives_ownership() -> String {
    let s = String::from("yours");
    s  // s is returned and ownership moves to caller
}

// Function that takes and gives back ownership
fn takes_and_gives_back(s: String) -> String {
    println!("  Temporarily took: {}", s);
    s  // s is returned and ownership moves to caller
}

// Function that takes ownership and returns value with length
fn calculate_length_and_return(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // Return ownership with additional value
}

// Example struct
struct User {
    username: String,
    email: String,
}
