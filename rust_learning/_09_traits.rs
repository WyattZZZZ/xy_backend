// TRAITS IN RUST
// Traits define shared behavior across types (similar to interfaces)
// They enable polymorphism and code reuse

pub fn run() {
    println!("\n=== TRAITS ===\n");

    // BASIC TRAIT IMPLEMENTATION
    println!("--- Basic Trait ---");

    let article = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("Boston"),
        author: String::from("John Doe"),
        content: String::from("This is the full article content..."),
    };

    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Rust 1.70 released!"),
        reply: false,
        retweet: false,
    };

    println!("Article: {}", article.summarize());
    println!("Tweet: {}", tweet.summarize());

    // DEFAULT IMPLEMENTATIONS
    println!("\n--- Default Implementations ---");

    let article2 = NewsArticle {
        headline: String::from("Default Summary Test"),
        location: String::from("NYC"),
        author: String::from("Jane Smith"),
        content: String::from("Content here..."),
    };

    println!("Author: {}", article2.author_summary());

    // TRAITS AS PARAMETERS
    println!("\n--- Traits as Parameters ---");

    notify(&article);
    notify(&tweet);

    // TRAIT BOUNDS
    println!("\n--- Trait Bounds ---");

    notify_with_bound(&article);

    // MULTIPLE TRAIT BOUNDS
    println!("\n--- Multiple Trait Bounds ---");

    let point = Point { x: 5, y: 10 };
    print_it(&point);

    // WHERE CLAUSE
    println!("\n--- Where Clause ---");

    let s = String::from("complex");
    complex_function(&s, &s);

    // RETURNING TYPES THAT IMPLEMENT TRAITS
    println!("\n--- Returning Trait Types ---");

    let summary = returns_summarizable();
    println!("Returned: {}", summary.summarize());

    // TRAIT INHERITANCE
    println!("\n--- Trait Inheritance ---");

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Display: {}", person.display());
    println!("Describe: {}", person.describe());

    // OPERATOR OVERLOADING WITH TRAITS
    println!("\n--- Operator Overloading ---");

    let p1 = PointAdd { x: 1, y: 2 };
    let p2 = PointAdd { x: 3, y: 4 };
    let p3 = p1 + p2;

    println!("Point addition: ({}, {})", p3.x, p3.y);

    // DERIVE TRAIT
    println!("\n--- Derive Trait ---");

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 30, height: 50 };

    println!("rect1: {:?}", rect1);
    println!("rect1 == rect2? {}", rect1 == rect2);

    let rect3 = rect1.clone();
    println!("Cloned rect3: {:?}", rect3);

    // ASSOCIATED TYPES
    println!("\n--- Associated Types ---");

    let container = Container { value: 42 };
    println!("Container has item: {}", container.has_item());

    // TRAIT OBJECTS (DYNAMIC DISPATCH)
    println!("\n--- Trait Objects ---");

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Square { side: 4.0 }),
    ];

    for shape in shapes {
        println!("Area: {:.2}", shape.area());
    }

    // CONDITIONAL TRAIT IMPLEMENTATION
    println!("\n--- Conditional Implementation ---");

    let wrapper = Wrapper(vec![1, 2, 3]);
    wrapper.print();

    // FROM AND INTO TRAITS
    println!("\n--- From/Into Traits ---");

    let my_str = "hello";
    let my_string: String = String::from(my_str);
    println!("Converted: {}", my_string);

    // Custom From implementation
    let num = Number::from(5);
    println!("Number: {}", num.value);

    // DISPLAY TRAIT
    println!("\n--- Display Trait ---");
    let user = User {
        name: String::from("Bob"),
        age: 25,
    };
    println!("{}", user);  // Uses Display trait
}

// Define a trait
trait Summary {
    // Required method (must be implemented)
    fn summarize(&self) -> String;

    // Default implementation (can be overridden)
    fn author_summary(&self) -> String {
        String::from("(Author not specified)")
    }
}

// Implement trait for a type
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn author_summary(&self) -> String {
        format!("By {}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as parameter (impl Trait syntax)
fn notify(item: &impl Summary) {
    println!("  Breaking news! {}", item.summarize());
}

// Trait bound syntax
fn notify_with_bound<T: Summary>(item: &T) {
    println!("  Notification: {}", item.summarize());
}

// Multiple trait bounds
use std::fmt::Display;
use std::fmt::Debug;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn print_it<T: Display + Debug>(item: &T) {
    println!("  Display: {}, Debug: {:?}", item, item);
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Where clause for complex bounds
fn complex_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("  Complex function called");
}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@example"),
        content: String::from("example tweet"),
        reply: false,
        retweet: false,
    }
}

// Trait inheritance (supertraits)
trait Displayable {
    fn display(&self) -> String;
}

trait Describable: Displayable {
    fn describe(&self) -> String {
        format!("Description: {}", self.display())
    }
}

struct Person {
    name: String,
    age: u32,
}

impl Displayable for Person {
    fn display(&self) -> String {
        format!("{}, age {}", self.name, self.age)
    }
}

impl Describable for Person {}

// Operator overloading with Add trait
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct PointAdd {
    x: i32,
    y: i32,
}

impl Add for PointAdd {
    type Output = PointAdd;

    fn add(self, other: PointAdd) -> PointAdd {
        PointAdd {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Derive common traits
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Associated types
trait Container {
    type Item;

    fn has_item(&self) -> bool;
}

struct Container {
    value: i32,
}

impl Container for Container {
    type Item = i32;

    fn has_item(&self) -> bool {
        true
    }
}

// Trait objects for dynamic dispatch
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// Conditional trait implementation
struct Wrapper<T>(T);

impl<T: Debug> Wrapper<T> {
    fn print(&self) {
        println!("  Wrapper: {:?}", self.0);
    }
}

// From trait implementation
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// Display trait
struct User {
    name: String,
    age: u32,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {} (age: {})", self.name, self.age)
    }
}
