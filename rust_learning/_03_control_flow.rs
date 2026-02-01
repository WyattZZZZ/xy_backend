// CONTROL FLOW IN RUST
// if expressions, loops, while, for, match

pub fn run() {
    println!("\n=== CONTROL FLOW ===\n");

    // IF EXPRESSIONS
    println!("--- If Expressions ---");
    let number = 7;

    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is 5");
    } else {
        println!("Number is greater than 5");
    }

    // if is an expression, so it returns a value
    let condition = true;
    let value = if condition { 5 } else { 10 };
    println!("Value from if expression: {}", value);

    // Both arms must return the same type
    // let value = if condition { 5 } else { "text" };  // Compile error!

    // LOOP - infinite loop
    println!("\n--- Loop (infinite) ---");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 2;  // break with a value
        }
    };
    println!("Result from loop: {}", result);

    // LOOP LABELS - for nested loops
    println!("\n--- Loop Labels ---");
    let mut count = 0;
    'outer: loop {
        println!("Outer loop count: {}", count);
        let mut inner_count = 0;

        loop {
            println!("  Inner loop count: {}", inner_count);
            inner_count += 1;

            if inner_count == 3 {
                break;  // Breaks the inner loop
            }

            if count == 2 {
                break 'outer;  // Breaks the outer loop
            }
        }
        count += 1;
    }

    // WHILE LOOP
    println!("\n--- While Loop ---");
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // FOR LOOP - iterating over collections
    println!("\n--- For Loop ---");
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("Value: {}", element);
    }

    // Range
    println!("\nCounting with range:");
    for number in 1..4 {  // 1, 2, 3 (excludes 4)
        println!("  {}", number);
    }

    println!("\nInclusive range:");
    for number in 1..=4 {  // 1, 2, 3, 4 (includes 4)
        println!("  {}", number);
    }

    // Reverse range
    println!("\nCountdown:");
    for number in (1..4).rev() {
        println!("  {}", number);
    }

    // Enumerate for index and value
    println!("\nWith index:");
    let names = ["Alice", "Bob", "Charlie"];
    for (index, name) in names.iter().enumerate() {
        println!("  {}: {}", index, name);
    }

    // MATCH - pattern matching (more powerful than switch)
    println!("\n--- Match ---");
    let number = 3;

    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 | 5 => println!("Four or Five!"),  // Multiple patterns
        _ => println!("Something else"),      // Default case (catch-all)
    }

    // Match is an expression
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    };
    println!("Number in words: {}", result);

    // Match with ranges
    println!("\n--- Match with Ranges ---");
    let age = 25;

    match age {
        0..=12 => println!("Child"),
        13..=19 => println!("Teenager"),
        20..=59 => println!("Adult"),
        60.. => println!("Senior"),  // Open-ended range
        // _ => println!("Invalid age"),  // Not needed with comprehensive ranges
    }

    // Match with guards (additional conditions)
    println!("\n--- Match with Guards ---");
    let number = Some(4);

    match number {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("Got: {}", x),
        None => println!("Nothing"),
    }

    // Destructuring with match
    println!("\n--- Destructuring with Match ---");
    let point = (0, 5);

    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On Y-axis at {}", y),
        (x, 0) => println!("On X-axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // IF LET - concise pattern matching
    println!("\n--- If Let ---");
    let favorite_color: Option<&str> = Some("blue");

    // Instead of:
    match favorite_color {
        Some(color) => println!("Favorite color is {}", color),
        None => {},
    }

    // You can use if let:
    if let Some(color) = favorite_color {
        println!("Favorite color is {} (using if let)", color);
    }

    // WHILE LET - loop with pattern matching
    println!("\n--- While Let ---");
    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // CONTINUE and BREAK
    println!("\n--- Continue and Break ---");
    for number in 1..=10 {
        if number % 2 == 0 {
            continue;  // Skip even numbers
        }
        if number > 7 {
            break;  // Stop at 7
        }
        println!("Odd number: {}", number);
    }

    // NESTED CONTROL FLOW
    println!("\n--- Nested Control Flow ---");
    for x in 1..=3 {
        for y in 1..=3 {
            let product = x * y;
            if product % 2 == 0 {
                println!("{} * {} = {} (even)", x, y, product);
            } else {
                println!("{} * {} = {} (odd)", x, y, product);
            }
        }
    }
}
