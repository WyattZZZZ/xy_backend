// LIFETIMES IN RUST
// Lifetimes ensure references are valid for as long as they need to be
// They prevent dangling references and ensure memory safety
//
// LIFETIME RULES:
// 1. Each reference has a lifetime
// 2. The lifetime of a reference must be at least as long as the scope it's used in
// 3. References must not outlive the data they refer to

pub fn run() {
    println!("\n=== LIFETIMES ===\n");

    // WHY LIFETIMES?
    println!("--- Why Lifetimes? ---");
    println!("Lifetimes prevent dangling references");
    println!("They ensure references are always valid");

    // LIFETIME IN FUNCTION SIGNATURES
    println!("\n--- Function Lifetime Annotations ---");

    let string1 = String::from("long string");
    let string2 = String::from("short");

    let result = longest(&string1, &string2);
    println!("Longest string: {}", result);

    // LIFETIME ANNOTATIONS SYNTAX
    println!("\n--- Lifetime Syntax ---");
    println!("'a is a lifetime parameter (like a generic)");
    println!("&'a str means a reference with lifetime 'a");

    // LIFETIME WITH DIFFERENT SCOPES
    println!("\n--- Different Scopes ---");

    let string1 = String::from("outer");
    let result;
    {
        let string2 = String::from("inner");
        result = longest(&string1, &string2);
        println!("Result inside scope: {}", result);
    }
    // result still valid because it refers to string1, not string2
    // If we tried to use result here and it referred to string2, it would be invalid

    // MULTIPLE LIFETIME PARAMETERS
    println!("\n--- Multiple Lifetimes ---");

    let text = String::from("announcement");
    let ann = "important";
    let result = announce_and_return_first(&text, ann);
    println!("Result: {}", result);

    // LIFETIME IN STRUCT DEFINITIONS
    println!("\n--- Lifetimes in Structs ---");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);

    // LIFETIME ELISION RULES
    println!("\n--- Lifetime Elision ---");
    println!("Compiler can often infer lifetimes (lifetime elision)");

    let s = String::from("hello");
    let result = first_word_simple(&s);
    println!("First word: {}", result);

    // LIFETIME IN METHODS
    println!("\n--- Lifetimes in Methods ---");

    let novel = String::from("Once upon a time. There was a story.");
    let first = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first };

    println!("Level: {}", excerpt.level());
    println!("Announce: {}", excerpt.announce_and_return_part("Warning"));

    // STATIC LIFETIME
    println!("\n--- Static Lifetime ---");

    let s: &'static str = "I have a static lifetime";
    println!("Static string: {}", s);
    println!("'static means the reference lives for entire program");

    // GENERIC TYPE PARAMETERS, TRAIT BOUNDS, AND LIFETIMES TOGETHER
    println!("\n--- Combined: Generics + Traits + Lifetimes ---");

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_announcement(&string1, string2, "Comparing");
    println!("Result: {}", result);

    // LIFETIME SUBTYPING
    println!("\n--- Lifetime Bounds ---");

    let string1 = String::from("test");
    let string2 = String::from("data");
    let parser = Parser {
        data: &string1,
        metadata: &string2,
    };
    println!("Parser created with data: {}", parser.data);

    // COMMON LIFETIME PATTERNS
    println!("\n--- Common Patterns ---");

    // Pattern 1: Return references from functions
    let s = String::from("hello");
    let first = get_first_char(&s);
    println!("First char: {}", first);

    // Pattern 2: Structs holding references
    let text = String::from("Some text");
    let wrapper = Wrapper { text: &text };
    println!("Wrapped: {}", wrapper.text);

    // LIFETIME ANNOTATIONS IN PRACTICE
    println!("\n--- Practical Example ---");

    let text = String::from("The quick brown fox jumps over the lazy dog");
    let analyzer = TextAnalyzer::new(&text);
    println!("Word count: {}", analyzer.word_count());
    println!("First word: {}", analyzer.first_word());
}

// Function with lifetime annotation
// 'a indicates the lifetime of the returned reference
// It means: "the returned reference will be valid for as long as both input references are valid"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This would NOT compile - can't return a reference to local data
// fn invalid_return<'a>() -> &'a str {
//     let s = String::from("hello");
//     &s  // ERROR: s doesn't live long enough
// }

// Multiple lifetime parameters
// 'a and 'b are independent lifetimes
fn announce_and_return_first<'a, 'b>(x: &'a str, announcement: &'b str) -> &'a str {
    println!("  Announcement: {}", announcement);
    x
}

// Struct with lifetime annotation
// The struct cannot outlive the reference it holds
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods with lifetimes
impl<'a> ImportantExcerpt<'a> {
    // Lifetime elision: compiler infers lifetimes
    fn level(&self) -> i32 {
        3
    }

    // Multiple lifetime parameters in method
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("  Attention: {}", announcement);
        self.part
    }
}

// Function that doesn't need explicit lifetime annotation (lifetime elision)
// Rule 1: Each parameter gets its own lifetime
// Rule 2: If there's one input lifetime, it's assigned to all output lifetimes
fn first_word_simple(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Combining generics, trait bounds, and lifetimes
use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("  Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime subtyping
struct Parser<'a, 'b> {
    data: &'a str,
    metadata: &'b str,
}

// Simple function with lifetime elision
fn get_first_char(s: &str) -> &str {
    &s[0..1]
}

// Struct holding reference
struct Wrapper<'a> {
    text: &'a str,
}

// Practical example: text analyzer
struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

// LIFETIME ELISION RULES (implicit lifetimes):
//
// Input lifetimes (on function parameters):
// 1. Each reference parameter gets its own lifetime parameter
//    fn foo(x: &i32, y: &i32) becomes fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
//
// Output lifetimes (on return values):
// 2. If there's exactly one input lifetime, that lifetime is assigned to all output lifetimes
//    fn foo<'a>(x: &'a i32) -> &'a i32
//
// 3. If there are multiple input lifetimes, but one is &self or &mut self,
//    the lifetime of self is assigned to all output lifetimes
//    fn foo<'a>(&'a self, x: &i32) -> &'a Type

// Examples of lifetime elision:

// Explicit:
// fn example<'a>(x: &'a str) -> &'a str { x }

// Elided (same thing):
// fn example(x: &str) -> &str { x }

// Explicit:
// fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str { x }

// This REQUIRES explicit annotation because:
// - Two input lifetimes
// - Return lifetime must be specified (which one to use?)
