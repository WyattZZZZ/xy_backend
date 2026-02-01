// FUNCTIONS IN RUST
// Functions are defined with fn keyword
// Snake case is the conventional naming style

pub fn run() {
    println!("\n=== FUNCTIONS ===\n");

    // BASIC FUNCTION CALL
    println!("--- Basic Functions ---");
    greet();
    greet_person("Alice");

    // FUNCTIONS WITH RETURN VALUES
    println!("\n--- Return Values ---");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);

    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);

    // EARLY RETURN
    println!("\n--- Early Return ---");
    println!("Is 4 even? {}", is_even(4));
    println!("Is 7 even? {}", is_even(7));

    // EXPRESSIONS VS STATEMENTS
    println!("\n--- Expressions vs Statements ---");
    let y = {
        let x = 3;
        x + 1  // No semicolon = expression, returns value
        // x + 1; would be a statement and return ()
    };
    println!("Value from block expression: {}", y);

    // MULTIPLE RETURN VALUES (using tuples)
    println!("\n--- Multiple Returns ---");
    let (min, max) = min_max(10, 20);
    println!("Min: {}, Max: {}", min, max);

    // CLOSURES (anonymous functions)
    println!("\n--- Closures ---");
    let add_one = |x: i32| x + 1;
    println!("5 + 1 = {}", add_one(5));

    // Closure with multiple parameters
    let multiply_closure = |x: i32, y: i32| x * y;
    println!("3 * 4 = {}", multiply_closure(3, 4));

    // Closure with type inference
    let double = |x| x * 2;
    println!("Double 5 = {}", double(5));

    // Closure capturing environment
    let factor = 10;
    let scale = |x| x * factor;  // Captures 'factor' from environment
    println!("5 * {} = {}", factor, scale(5));

    // HIGHER-ORDER FUNCTIONS
    println!("\n--- Higher-Order Functions ---");
    let result = apply_operation(5, 3, add);
    println!("Apply add: {}", result);

    let result = apply_operation(5, 3, multiply);
    println!("Apply multiply: {}", result);

    // Using closure as argument
    let result = apply_operation(5, 3, |a, b| a - b);
    println!("Apply subtract: {}", result);

    // METHODS (functions associated with types)
    println!("\n--- Methods ---");
    let rect = Rectangle { width: 30, height: 50 };
    println!("Rectangle area: {}", rect.area());
    println!("Rectangle perimeter: {}", rect.perimeter());

    // METHOD CHAINING
    println!("\n--- Method Chaining ---");
    let mut builder = StringBuilder::new();
    let result = builder.append("Hello")
                        .append(" ")
                        .append("World")
                        .build();
    println!("Built string: {}", result);

    // ASSOCIATED FUNCTIONS (like static methods)
    println!("\n--- Associated Functions ---");
    let square = Rectangle::square(10);
    println!("Square area: {}", square.area());

    // FUNCTION POINTERS
    println!("\n--- Function Pointers ---");
    let operation: fn(i32, i32) -> i32 = add;
    println!("Using function pointer: {}", operation(10, 20));

    // DIVERGING FUNCTIONS (never return)
    println!("\n--- Diverging Functions ---");
    println!("Diverging functions have return type '!' (never type)");
    println!("Example: panic!(), loop {}, exit()");
}

// Simple function with no parameters or return value
fn greet() {
    println!("Hello, World!");
}

// Function with parameters
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value
// Return type is specified with ->
fn add(a: i32, b: i32) -> i32 {
    a + b  // Last expression is returned (no semicolon!)
    // return a + b;  // Explicit return also works
}

fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // Explicit return with semicolon
}

// Function with early return
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;  // Early return
    }
    false  // Final return
}

// Function returning multiple values
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// Higher-order function (takes function as parameter)
fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}

// Struct for demonstrating methods
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block - methods associated with Rectangle
impl Rectangle {
    // Method (takes &self as first parameter)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Associated function (doesn't take self)
    // Called with :: syntax: Rectangle::square(10)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Example of method chaining
struct StringBuilder {
    content: String,
}

impl StringBuilder {
    fn new() -> Self {
        StringBuilder {
            content: String::new(),
        }
    }

    // Returns &mut self for chaining
    fn append(&mut self, text: &str) -> &mut Self {
        self.content.push_str(text);
        self
    }

    fn build(&self) -> String {
        self.content.clone()
    }
}

// Example of diverging function (commented to avoid panic)
// fn diverging_function() -> ! {
//     panic!("This function never returns!");
// }
