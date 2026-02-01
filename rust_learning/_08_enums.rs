// ENUMS IN RUST
// Enumerations allow you to define a type by enumerating its possible variants
// Rust enums are more powerful than in many other languages

pub fn run() {
    println!("\n=== ENUMS ===\n");

    // BASIC ENUM
    println!("--- Basic Enum ---");

    let ip_v4 = IpAddrKind::V4;
    let ip_v6 = IpAddrKind::V6;

    println!("Created IPv4 and IPv6 enum variants");

    // ENUM WITH DATA
    println!("\n--- Enum with Data ---");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home IP: IPv4");
    println!("Loopback IP: IPv6");

    // ENUM WITH DIFFERENT TYPES
    println!("\n--- Enum with Different Types ---");

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    println!("Created different message variants");

    // METHODS ON ENUMS
    println!("\n--- Methods on Enums ---");

    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();

    // OPTION ENUM (built-in)
    println!("\n--- Option<T> Enum ---");

    let some_number = Some(5);
    let some_string = Some("text");
    let absent_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent: {:?}", absent_number);

    // MATCHING ENUMS
    println!("\n--- Pattern Matching with Enums ---");

    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);

    println!("Penny value: {} cents", value_in_cents(coin1));
    println!("Quarter value: {} cents", value_in_cents(coin2));

    // MATCHING WITH OPTION
    println!("\n--- Matching Option ---");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);

    // IF LET
    println!("\n--- If Let ---");

    let config_max = Some(3u8);

    // Instead of match with one arm:
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Use if let:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {} (using if let)", max);
    }

    // IF LET with ELSE
    println!("\n--- If Let with Else ---");

    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        println!("Not a state quarter");
    }

    // RESULT ENUM (built-in) - Preview
    println!("\n--- Result<T, E> Enum Preview ---");

    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("error"));

    match success {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match failure {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // DESTRUCTURING ENUMS
    println!("\n--- Destructuring Enums ---");

    let msg = Message::Move { x: 100, y: 200 };

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to x:{}, y:{}", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
    }

    // ENUM WITH MATCH GUARDS
    println!("\n--- Match Guards ---");

    let number = Some(4);

    match number {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Greater than or equal to five: {}", x),
        None => println!("No value"),
    }

    // COMPLEX ENUM EXAMPLE
    println!("\n--- Complex Enum ---");

    let event = WebEvent::Click { x: 100, y: 200 };
    inspect(event);

    let event = WebEvent::KeyPress('x');
    inspect(event);

    let event = WebEvent::PageLoad;
    inspect(event);

    // ENUM METHODS WITH SELF
    println!("\n--- Enum Methods ---");

    let status1 = Status::Active;
    let status2 = Status::Inactive;

    println!("Status 1 is active? {}", status1.is_active());
    println!("Status 2 is active? {}", status2.is_active());

    // USING ENUMS IN VECTORS
    println!("\n--- Enums in Collections ---");

    let messages = vec![
        Message::Write(String::from("First")),
        Message::Move { x: 10, y: 20 },
        Message::Quit,
    ];

    for msg in messages {
        msg.call();
    }
}

// Basic enum
enum IpAddrKind {
    V4,
    V6,
}

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Enum with different types
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Methods on enum
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("  Quit message"),
            Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
            Message::Write(text) => println!("  Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("  Change color to ({}, {}, {})", r, g, b),
        }
    }
}

// Enum for coins
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Function using match with enum
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("  Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("  State quarter from {:?}!", state);
            25
        }
    }
}

// Function with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Complex enum
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("  Page loaded"),
        WebEvent::PageUnload => println!("  Page unloaded"),
        WebEvent::KeyPress(c) => println!("  Pressed '{}'", c),
        WebEvent::Paste(s) => println!("  Pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("  Clicked at x={}, y={}", x, y),
    }
}

// Enum with methods
enum Status {
    Active,
    Inactive,
}

impl Status {
    fn is_active(&self) -> bool {
        match self {
            Status::Active => true,
            Status::Inactive => false,
        }
    }
}
