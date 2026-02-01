// STRUCTS IN RUST
// Custom data types that group related values together
// Similar to classes in other languages, but without inheritance

pub fn run() {
    println!("\n=== STRUCTS ===\n");

    // DEFINING AND INSTANTIATING STRUCTS
    println!("--- Basic Struct ---");

    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}, Sign-ins: {}", user1.active, user1.sign_in_count);

    // MUTABLE STRUCT
    println!("\n--- Mutable Struct ---");

    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("bob_new@example.com");
    user2.sign_in_count += 1;
    println!("Updated email: {}, Sign-ins: {}", user2.email, user2.sign_in_count);

    // FIELD INIT SHORTHAND
    println!("\n--- Field Init Shorthand ---");

    let username = String::from("charlie");
    let email = String::from("charlie@example.com");

    let user3 = User {
        username,  // Shorthand: same as username: username
        email,     // Shorthand: same as email: email
        active: true,
        sign_in_count: 1,
    };

    println!("User3: {}", user3.username);

    // STRUCT UPDATE SYNTAX
    println!("\n--- Struct Update Syntax ---");

    let user4 = User {
        email: String::from("david@example.com"),
        ..user3  // Use remaining fields from user3
        // Note: This moves data from user3!
    };

    println!("User4: {}", user4.username);
    // println!("{}", user3.username);  // ERROR! username was moved

    // TUPLE STRUCTS
    println!("\n--- Tuple Structs ---");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);

    // UNIT-LIKE STRUCTS
    println!("\n--- Unit-Like Structs ---");

    let marker = AlwaysEqual;
    println!("Unit-like struct created (no fields)");

    // METHODS
    println!("\n--- Methods ---");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {}x{}", rect.width, rect.height);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Can hold 10x10? {}", rect.can_hold(&Rectangle { width: 10, height: 10 }));

    // MUTABLE METHODS
    println!("\n--- Mutable Methods ---");

    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };

    println!("Before: {}x{}", rect.width, rect.height);
    rect.double();
    println!("After double: {}x{}", rect.width, rect.height);

    // ASSOCIATED FUNCTIONS (like static methods)
    println!("\n--- Associated Functions ---");

    let square = Rectangle::square(25);
    println!("Square: {}x{}, Area: {}", square.width, square.height, square.area());

    // MULTIPLE IMPL BLOCKS
    println!("\n--- Multiple Implementation Blocks ---");

    let circle = Circle { radius: 5.0 };
    println!("Circle radius: {}", circle.radius);
    println!("Area: {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());
    println!("Description: {}", circle.describe());

    // STRUCT WITH DIFFERENT TYPES
    println!("\n--- Generic-like Struct (different example) ---");

    let product = Product {
        id: 101,
        name: String::from("Laptop"),
        price: 999.99,
        in_stock: true,
    };

    println!("Product: {}", product.name);
    println!("Price: ${:.2}, In stock: {}", product.price, product.in_stock);

    // NESTED STRUCTS
    println!("\n--- Nested Structs ---");

    let address = Address {
        street: String::from("123 Main St"),
        city: String::from("Boston"),
        zipcode: String::from("02101"),
    };

    let person = Person {
        name: String::from("John Doe"),
        age: 30,
        address,
    };

    println!("Person: {}, Age: {}", person.name, person.age);
    println!("Address: {}, {}", person.address.street, person.address.city);

    // DEBUG TRAIT
    println!("\n--- Debug Trait ---");

    let debug_rect = DebugRectangle {
        width: 30,
        height: 50,
    };

    println!("Debug: {:?}", debug_rect);
    println!("Pretty Debug:\n{:#?}", debug_rect);

    // BUILDER PATTERN
    println!("\n--- Builder Pattern ---");

    let config = Config::new()
        .host("localhost")
        .port(8080)
        .debug(true)
        .build();

    println!("Config - Host: {}, Port: {}, Debug: {}",
             config.host, config.port, config.debug);
}

// Basic struct
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

// Struct with methods
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block
impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Method with additional parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Mutable method
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    // Associated function (no self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks for the same struct
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Circle {
    fn describe(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

// Struct with different types
struct Product {
    id: u32,
    name: String,
    price: f64,
    in_stock: bool,
}

// Nested structs
struct Address {
    street: String,
    city: String,
    zipcode: String,
}

struct Person {
    name: String,
    age: u32,
    address: Address,
}

// Struct with Debug trait
#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}

// Builder pattern example
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

struct ConfigBuilder {
    host: String,
    port: u16,
    debug: bool,
}

impl Config {
    fn new() -> ConfigBuilder {
        ConfigBuilder {
            host: String::from("0.0.0.0"),
            port: 3000,
            debug: false,
        }
    }
}

impl ConfigBuilder {
    fn host(mut self, host: &str) -> Self {
        self.host = String::from(host);
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    fn build(self) -> Config {
        Config {
            host: self.host,
            port: self.port,
            debug: self.debug,
        }
    }
}
