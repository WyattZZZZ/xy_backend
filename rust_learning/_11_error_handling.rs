// ERROR HANDLING IN RUST
// Rust groups errors into two categories:
// 1. Recoverable errors (Result<T, E>)
// 2. Unrecoverable errors (panic!)

pub fn run() {
    println!("\n=== ERROR HANDLING ===\n");

    // PANIC - UNRECOVERABLE ERRORS
    println!("--- Panic (Unrecoverable Errors) ---");
    println!("panic! macro stops execution");
    println!("Use for unrecoverable errors or bugs");
    // panic!("This will crash the program!");

    // RESULT<T, E> - RECOVERABLE ERRORS
    println!("\n--- Result<T, E> ---");

    let result: Result<i32, String> = Ok(42);
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    let error: Result<i32, String> = Err(String::from("something went wrong"));
    match error {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // HANDLING RESULT WITH MATCH
    println!("\n--- Handling Result with Match ---");

    let division1 = divide(10.0, 2.0);
    match division1 {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    let division2 = divide(10.0, 0.0);
    match division2 {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // UNWRAP AND EXPECT
    println!("\n--- unwrap() and expect() ---");

    let good_result: Result<i32, &str> = Ok(5);
    let value = good_result.unwrap();  // Gets value or panics
    println!("Unwrapped value: {}", value);

    // This would panic:
    // let bad_result: Result<i32, &str> = Err("error");
    // bad_result.unwrap();  // PANIC!

    // expect() is like unwrap() but with custom message
    let good_result: Result<i32, &str> = Ok(10);
    let value = good_result.expect("This should not fail");
    println!("Expected value: {}", value);

    // QUESTION MARK OPERATOR (?)
    println!("\n--- Question Mark Operator (?) ---");

    match read_username() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // OPTION<T> - HANDLING NULL
    println!("\n--- Option<T> ---");

    let some_value = Some(5);
    let no_value: Option<i32> = None;

    println!("Some value: {:?}", some_value);
    println!("No value: {:?}", no_value);

    // UNWRAPPING OPTION
    println!("\n--- Unwrapping Option ---");

    let x = Some(10);
    println!("Value: {}", x.unwrap());

    // This would panic:
    // let y: Option<i32> = None;
    // y.unwrap();  // PANIC!

    // SAFER OPTION HANDLING
    println!("\n--- Safe Option Handling ---");

    let value = Some(42);
    if let Some(v) = value {
        println!("Got value: {}", v);
    }

    // unwrap_or() provides default value
    let value1 = Some(100);
    let value2: Option<i32> = None;
    println!("Value 1: {}", value1.unwrap_or(0));
    println!("Value 2: {}", value2.unwrap_or(0));

    // unwrap_or_else() with closure
    let value: Option<i32> = None;
    let result = value.unwrap_or_else(|| {
        println!("  Computing default value...");
        50
    });
    println!("Result: {}", result);

    // MAP AND AND_THEN
    println!("\n--- map() and and_then() ---");

    let maybe_number = Some(5);
    let doubled = maybe_number.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);

    let no_number: Option<i32> = None;
    let doubled = no_number.map(|x| x * 2);
    println!("Doubled None: {:?}", doubled);

    // and_then() for chaining operations
    let result = Some(4).and_then(|x| Some(x * 2)).and_then(|x| Some(x + 1));
    println!("Chained result: {:?}", result);

    // CUSTOM ERROR TYPES
    println!("\n--- Custom Error Types ---");

    match parse_person("Alice,30") {
        Ok(person) => println!("Person: {}, Age: {}", person.name, person.age),
        Err(e) => println!("Parse error: {}", e),
    }

    match parse_person("Bob") {
        Ok(person) => println!("Person: {}, Age: {}", person.name, person.age),
        Err(e) => println!("Parse error: {}", e),
    }

    // RESULT METHODS
    println!("\n--- Result Methods ---");

    let result: Result<i32, &str> = Ok(42);
    println!("is_ok: {}", result.is_ok());
    println!("is_err: {}", result.is_err());

    // ok() converts Result to Option
    let option = result.ok();
    println!("As Option: {:?}", option);

    // OPTION METHODS
    println!("\n--- Option Methods ---");

    let x = Some(5);
    println!("is_some: {}", x.is_some());
    println!("is_none: {}", x.is_none());

    // or() provides alternative Option
    let x: Option<i32> = None;
    let y = Some(100);
    println!("x or y: {:?}", x.or(y));

    // COMBINING RESULTS
    println!("\n--- Combining Results ---");

    let result = combine_results();
    match result {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }

    // EARLY RETURN WITH ?
    println!("\n--- Early Return Pattern ---");

    match process_data() {
        Ok(result) => println!("Processed: {}", result),
        Err(e) => println!("Processing error: {}", e),
    }

    // CONVERTING BETWEEN RESULT AND OPTION
    println!("\n--- Converting Result <-> Option ---");

    let result: Result<i32, String> = Ok(42);
    let option = result.ok();  // Result to Option, discards error
    println!("Result as Option: {:?}", option);

    let option = Some(42);
    let result: Result<i32, &str> = option.ok_or("no value");  // Option to Result
    println!("Option as Result: {:?}", result);

    // ITERATOR RESULT COLLECTION
    println!("\n--- Collecting Results ---");

    let strings = vec!["1", "2", "3", "4"];
    let numbers: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match numbers {
        Ok(nums) => println!("Parsed numbers: {:?}", nums),
        Err(e) => println!("Parse error: {}", e),
    }

    // With an error
    let strings = vec!["1", "2", "oops", "4"];
    let numbers: Result<Vec<i32>, _> = strings
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();

    match numbers {
        Ok(nums) => println!("Parsed numbers: {:?}", nums),
        Err(e) => println!("Parse error: {}", e),
    }
}

// Function returning Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Function using ? operator
fn read_username() -> Result<String, String> {
    let username = get_username()?;  // ? propagates error if Err
    let trimmed = trim_username(username)?;
    Ok(trimmed)
}

fn get_username() -> Result<String, String> {
    Ok(String::from("  alice  "))
}

fn trim_username(username: String) -> Result<String, String> {
    Ok(username.trim().to_string())
}

// Custom error type
#[derive(Debug)]
struct ParseError {
    message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

struct Person {
    name: String,
    age: u32,
}

fn parse_person(input: &str) -> Result<Person, ParseError> {
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 2 {
        return Err(ParseError {
            message: String::from("Expected format: name,age"),
        });
    }

    let name = parts[0].to_string();
    let age = parts[1].parse::<u32>().map_err(|_| ParseError {
        message: String::from("Invalid age"),
    })?;

    Ok(Person { name, age })
}

// Combining multiple Results
fn combine_results() -> Result<i32, String> {
    let a = divide(10.0, 2.0)?;
    let b = divide(20.0, 4.0)?;
    Ok((a + b) as i32)
}

// Early return pattern
fn process_data() -> Result<String, String> {
    let step1 = step_one()?;
    let step2 = step_two(step1)?;
    let step3 = step_three(step2)?;
    Ok(step3)
}

fn step_one() -> Result<i32, String> {
    Ok(10)
}

fn step_two(x: i32) -> Result<i32, String> {
    if x > 5 {
        Ok(x * 2)
    } else {
        Err(String::from("Too small"))
    }
}

fn step_three(x: i32) -> Result<String, String> {
    Ok(format!("Result: {}", x))
}

// BEST PRACTICES:
//
// 1. Use Result<T, E> for recoverable errors
// 2. Use panic! for unrecoverable errors or bugs
// 3. Use ? operator for clean error propagation
// 4. Prefer expect() over unwrap() for better error messages
// 5. Use custom error types for complex applications
// 6. Return Option<T> when None is a valid state (not an error)
// 7. Use unwrap_or() or unwrap_or_else() for safe defaults
// 8. Use map() and and_then() for functional error handling
