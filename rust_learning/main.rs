// RUST LEARNING - MAIN ENTRY POINT
// This file runs all the learning modules sequentially
//
// HOW TO USE:
// 1. Compile: rustc main.rs
// 2. Run: ./main
//
// Or use cargo:
// 1. cargo init (if not already a cargo project)
// 2. Move all .rs files to src/
// 3. cargo run

// Declare all modules
mod _01_basic_types;
mod _02_strings;
mod _03_control_flow;
mod _04_functions;
mod _05_ownership;
mod _06_borrowing;
mod _07_structs;
mod _08_enums;
mod _09_traits;
mod _10_lifetimes;
mod _11_error_handling;

use std::io::{self, Write};

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║           WELCOME TO RUST LEARNING JOURNEY!               ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    println!("\nThis program will teach you Rust from basics to advanced concepts.");
    println!("Each module covers a specific topic with examples and explanations.");

    loop {
        println!("\n┌────────────────────────────────────────────────────────────┐");
        println!("│                      SELECT A MODULE                       │");
        println!("└────────────────────────────────────────────────────────────┘");
        println!("\n  1.  Basic Data Types (integers, floats, booleans, chars)");
        println!("  2.  Strings (String vs &str, operations)");
        println!("  3.  Control Flow (if, loops, match)");
        println!("  4.  Functions (syntax, parameters, closures)");
        println!("  5.  Ownership (Rust's unique feature)");
        println!("  6.  Borrowing (references and borrowing rules)");
        println!("  7.  Structs (custom data types, methods)");
        println!("  8.  Enums (pattern matching, Option, Result)");
        println!("  9.  Traits (interfaces, polymorphism)");
        println!("  10. Lifetimes (memory safety guarantees)");
        println!("  11. Error Handling (Result, Option, panic)");
        println!("\n  A.  Run ALL modules sequentially");
        println!("  Q.  Quit");

        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_uppercase();

        match choice.as_str() {
            "1" => {
                print_separator();
                _01_basic_types::run();
                press_enter_to_continue();
            }
            "2" => {
                print_separator();
                _02_strings::run();
                press_enter_to_continue();
            }
            "3" => {
                print_separator();
                _03_control_flow::run();
                press_enter_to_continue();
            }
            "4" => {
                print_separator();
                _04_functions::run();
                press_enter_to_continue();
            }
            "5" => {
                print_separator();
                _05_ownership::run();
                press_enter_to_continue();
            }
            "6" => {
                print_separator();
                _06_borrowing::run();
                press_enter_to_continue();
            }
            "7" => {
                print_separator();
                _07_structs::run();
                press_enter_to_continue();
            }
            "8" => {
                print_separator();
                _08_enums::run();
                press_enter_to_continue();
            }
            "9" => {
                print_separator();
                _09_traits::run();
                press_enter_to_continue();
            }
            "10" => {
                print_separator();
                _10_lifetimes::run();
                press_enter_to_continue();
            }
            "11" => {
                print_separator();
                _11_error_handling::run();
                press_enter_to_continue();
            }
            "A" => {
                run_all_modules();
            }
            "Q" => {
                println!("\n╔════════════════════════════════════════════════════════════╗");
                println!("║          Thank you for learning Rust! 🦀                   ║");
                println!("║          Keep practicing and happy coding!                 ║");
                println!("╚════════════════════════════════════════════════════════════╝\n");
                break;
            }
            _ => {
                println!("\n❌ Invalid choice. Please try again.");
            }
        }
    }
}

fn run_all_modules() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║              RUNNING ALL MODULES                           ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    let modules = vec![
        ("1. Basic Data Types", _01_basic_types::run as fn()),
        ("2. Strings", _02_strings::run as fn()),
        ("3. Control Flow", _03_control_flow::run as fn()),
        ("4. Functions", _04_functions::run as fn()),
        ("5. Ownership", _05_ownership::run as fn()),
        ("6. Borrowing", _06_borrowing::run as fn()),
        ("7. Structs", _07_structs::run as fn()),
        ("8. Enums", _08_enums::run as fn()),
        ("9. Traits", _09_traits::run as fn()),
        ("10. Lifetimes", _10_lifetimes::run as fn()),
        ("11. Error Handling", _11_error_handling::run as fn()),
    ];

    for (name, module) in modules {
        print_separator();
        println!("\n▶ Running: {}", name);
        module();
        println!("\n✓ Completed: {}", name);
    }

    print_separator();
    println!("\n🎉 All modules completed!");
    press_enter_to_continue();
}

fn print_separator() {
    println!("\n════════════════════════════════════════════════════════════");
}

fn press_enter_to_continue() {
    print!("\nPress ENTER to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// LEARNING PATH RECOMMENDATION:
//
// BEGINNER:
// 1. Start with Basic Data Types
// 2. Learn about Strings
// 3. Understand Control Flow
// 4. Master Functions
//
// INTERMEDIATE:
// 5. Understand Ownership (CRUCIAL!)
// 6. Learn Borrowing (CRUCIAL!)
// 7. Work with Structs
// 8. Explore Enums
//
// ADVANCED:
// 9. Master Traits
// 10. Understand Lifetimes
// 11. Handle Errors Properly
//
// TIPS:
// - Take your time with Ownership and Borrowing - they're the most important!
// - Try modifying the examples to see what happens
// - Read the Rust Book: https://doc.rust-lang.org/book/
// - Practice on https://exercism.org/tracks/rust
// - Join the Rust community: https://www.rust-lang.org/community
