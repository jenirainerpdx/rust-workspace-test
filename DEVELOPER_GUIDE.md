# Developer Documentation

## Overview

This repository provides a complete Rust development environment setup for Week 1 of the Rust Bootcamp. It includes everything needed to get started with Rust development using modern tools and best practices.

## What's Included

### Core Components
- **Rust Project Structure**: A properly configured Rust project with `Cargo.toml` and source code
- **Sample Application**: A weather forecast demo that demonstrates basic Rust concepts
- **Development Environment**: Pre-configured setup for Visual Studio Code and GitHub Codespaces
- **Learning Materials**: Comprehensive lessons and hands-on assignments

### Development Tools Setup
- **Rust Toolchain**: rustc, cargo, and related tools
- **Visual Studio Code Extensions**:
  - Rust Analyzer (language support, IntelliSense, error checking)
  - GitHub Copilot (AI pair programming)
  - LLDB (debugging support)
- **GitHub Codespaces**: Browser-based development environment

## Quick Start Guide

### Option 1: Using GitHub Codespaces (Recommended for Beginners)
1. Click the "Code" button on the GitHub repository page
2. Select "Codespaces" tab
3. Click "Create codespace on main"
4. Wait for the environment to load (2-3 minutes)
5. Open a terminal and run:
   ```bash
   cargo run
   ```

### Option 2: Local Development
1. **Install Prerequisites**:
   - Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Install Visual Studio Code
   - Install recommended extensions

2. **Clone and Run**:
   ```bash
   git clone <repository-url>
   cd rust-workspace-test
   cargo run
   ```

## Understanding the Code

The sample application (`src/main.rs`) demonstrates:

```rust
use rand::thread_rng;           // Importing external crate
use rand::seq::SliceRandom;

fn main() {                     // Entry point
    let forecasts = [...];      // Array declaration
    let mut rng = thread_rng(); // Mutable variable
    let forecast = forecasts    // Method chaining
        .choose(&mut rng)       // Random selection
        .unwrap();              // Error handling
    
    println!("Today's weather forecast: {}", forecast); // String formatting
}
```

### Key Concepts Covered
- **External Dependencies**: Using the `rand` crate for random number generation
- **Variables**: `let` bindings, mutability with `mut`
- **Arrays**: Fixed-size collections
- **Method Calls**: Calling methods on types
- **Error Handling**: Basic use of `unwrap()`
- **String Formatting**: Using `println!` macro

## Development Workflow

### Building the Project
```bash
cargo build        # Compile the project
cargo build --release  # Optimized build for production
```

### Running the Project
```bash
cargo run          # Build and run in one command
cargo run --release    # Run optimized version
```

### Development Tools
```bash
cargo check        # Fast compilation check without executable
cargo clippy       # Linting and suggestions
cargo fmt          # Code formatting
cargo test         # Run tests (when tests exist)
```

## Assignment Completion

This repository supports three main assignments:

1. **Assignment 1**: Basic Rust and VS Code setup
   - Verify by running `cargo run` successfully
   
2. **Assignment 2**: GitHub Copilot integration
   - Use Copilot to modify or extend the sample code
   - Document your experience in a README

3. **Assignment 3**: Codespaces configuration
   - This repository is already Codespaces-ready
   - Test by creating a new Codespace and running the demo

## Next Steps

After completing this week:
- Explore the [Rust Book](https://doc.rust-lang.org/book/) for deeper learning
- Try modifying the weather app to add new features
- Continue to Week 2 of the Rust Bootcamp
- Practice with more complex Rust projects

## Resources for Further Learning

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Rust Programming Language (Video Course)](https://ds500.paiml.com/learn/course/g6u1k/)