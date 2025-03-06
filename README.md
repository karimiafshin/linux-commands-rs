# Linux Commands in Rust (`linux-commands-rs`)

A collection of Linux command-line utilities reimplemented in Rust, inspired by and based on the teachings from Ken Youens-Clark's book, Command-Line Rust. This project aims to provide Rust-based implementations of common Linux commands like ls, echo, cat, and more, while focusing on performance, simplicity, and idiomatic Rust practices.

The implementations and design patterns in this project are heavily influenced by the examples and exercises from Command-Line Rust, which provides an excellent foundation for building robust and efficient command-line tools in Rust.
## Features
- **Rust-based implementations**: Learn how to build CLI tools in Rust.
- **Modular design**: Each command is a separate binary, making it easy to extend and maintain.
- **Cross-platform**: Designed to work on Linux, macOS, and Windows (where applicable).
- **Educational**: Perfect for Rust beginners looking to practice systems programming.

## Commands Implemented
- `ls`: List directory contents.
- `echo`: Print arguments to the terminal.
- `cat`: Concatenate and print files.
- (More commands to come!)

## Getting Started
1. Clone the repository:
   ```bash
   git clone https://github.com/afshinnkarimi/linux-commands-rs.git
   cd linux-commands-rs
2. Build the project:
   ```bash
   cargo build --release
3. Run a command:
   ```bash
   ./target/release/ls
   ./target/release/echo Hello, world!
   
Inspired by Command-Line Rust

This project is heavily inspired by Ken Youens-Clark's book, Command-Line Rust. The book provides a hands-on approach to learning Rust by building command-line tools, and many of the design patterns, testing strategies, and Rust idioms used in this project are derived from the book.

If you're interested in learning Rust through practical, real-world examples, I highly recommend checking out Command-Line Rust:
- [Official Website](https://www.oreilly.com/library/view/command-line-rust/9781098109424)
- [GitHub Repository](https://github.com/kyclark/command-line-rust)

Contributing

Contributions are welcome! Whether you want to add a new command, improve an existing one, or fix a bug, feel free to open an issue or submit a pull request. Please follow the contribution guidelines.
License

This project is licensed under the MIT License. See LICENSE for details.

Why Rust?
Rust's focus on performance, memory safety, and modern tooling makes it an excellent choice for building reliable and efficient command-line tools. This project is a great way to learn Rust while recreating familiar Linux utilities.

Tags: rust, linux, cli, command-line, systems-programming, learning-rust
