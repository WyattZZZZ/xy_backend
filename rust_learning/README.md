# Rust Learning Journey

A comprehensive, hands-on guide to learning Rust from basic data types to advanced concepts like traits and lifetimes.

## What's Included

This learning project contains 11 modules covering essential Rust concepts:

1. **Basic Data Types** - Integers, floats, booleans, characters, arrays, tuples
2. **Strings** - String vs &str, string operations and manipulations
3. **Control Flow** - if expressions, loops, while, for, match
4. **Functions** - Function syntax, parameters, closures, higher-order functions
5. **Ownership** - Rust's unique ownership system for memory safety
6. **Borrowing** - References and borrowing rules
7. **Structs** - Custom data types, methods, and implementation blocks
8. **Enums** - Enumerations, pattern matching, Option, and Result types
9. **Traits** - Interfaces, polymorphism, and code reuse
10. **Lifetimes** - Lifetime annotations and memory safety guarantees
11. **Error Handling** - Result, Option, panic, and error propagation

## How to Run

### Option 1: Using rustc (Simple)

Navigate to the `rust_learning` directory and compile with rustc:

```bash
cd rust_learning
rustc main.rs
./main
```

### Option 2: Using Cargo (Recommended)

If you want to use Cargo (Rust's package manager):

```bash
# Create a new cargo project
cargo new rust_learning_project
cd rust_learning_project

# Copy all .rs files to src/ directory
cp ../rust_learning/*.rs src/

# Rename main.rs if needed
mv src/main.rs src/main_backup.rs  # backup the generated one
mv ../rust_learning/main.rs src/main.rs

# Run the project
cargo run
```

Or convert the current directory:

```bash
cd rust_learning
cargo init
cargo run
```

## Interactive Menu

When you run the program, you'll see an interactive menu:

```
╔════════════════════════════════════════════════════════════╗
║           WELCOME TO RUST LEARNING JOURNEY!               ║
╚════════════════════════════════════════════════════════════╝

  1.  Basic Data Types
  2.  Strings
  3.  Control Flow
  4.  Functions
  5.  Ownership
  6.  Borrowing
  7.  Structs
  8.  Enums
  9.  Traits
  10. Lifetimes
  11. Error Handling

  A.  Run ALL modules sequentially
  Q.  Quit
```

Select a module by entering its number, or press 'A' to run all modules.

## Learning Path

### Beginner (Start Here)
1. Basic Data Types
2. Strings
3. Control Flow
4. Functions

### Intermediate (Core Concepts)
5. **Ownership** ⭐ (Most Important!)
6. **Borrowing** ⭐ (Most Important!)
7. Structs
8. Enums

### Advanced
9. Traits
10. Lifetimes
11. Error Handling

## File Structure

```
rust_learning/
├── README.md                    # This file
├── main.rs                      # Entry point with interactive menu
├── _01_basic_types.rs          # Module 1
├── _02_strings.rs              # Module 2
├── _03_control_flow.rs         # Module 3
├── _04_functions.rs            # Module 4
├── _05_ownership.rs            # Module 5
├── _06_borrowing.rs            # Module 6
├── _07_structs.rs              # Module 7
├── _08_enums.rs                # Module 8
├── _09_traits.rs               # Module 9
├── _10_lifetimes.rs            # Module 10
└── _11_error_handling.rs       # Module 11
```

## Key Concepts

### Ownership Rules
1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped

### Borrowing Rules
1. At any given time, you can have EITHER:
   - One mutable reference, OR
   - Any number of immutable references
2. References must always be valid

## Tips for Learning

1. **Take Your Time** - Especially with Ownership and Borrowing
2. **Experiment** - Modify the examples and see what happens
3. **Read Error Messages** - Rust's compiler errors are very helpful
4. **Practice** - Try solving problems on [Exercism](https://exercism.org/tracks/rust)
5. **Read the Book** - [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Additional Resources

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings - Small exercises](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)
- [Rust Playground](https://play.rust-lang.org/)
- [Rust Community](https://www.rust-lang.org/community)

## What Makes Rust Special?

- **Memory Safety** without garbage collection
- **Zero-cost abstractions** - high-level code runs as fast as low-level
- **Ownership System** - prevents memory leaks and data races at compile time
- **Fearless Concurrency** - write concurrent code without fear
- **Great tooling** - Cargo, rustfmt, clippy, and more

## Next Steps

After completing these modules:
1. Build a CLI application
2. Create a web server with Actix or Rocket
3. Learn async programming with Tokio
4. Explore system programming
5. Contribute to open source Rust projects

Happy Learning! 🦀
