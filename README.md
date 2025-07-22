# User Data Structure Collection

A small Rust project for experimenting with and learning about user-defined data structures using Rust's standard library collections.

## Project Overview

This project provides a simple command-line menu for users to interactively explore and build several common data structures in Rust:

- **HashMap**: An unordered map for fast key-value lookups.
- **BTreeMap**: An ordered map, sorted by key.
- **BTreeSet**: A set of unique, sorted values.
- **BinaryHeap**: A max-heap priority queue.

Each option demonstrates basic usage and prints the resulting structure to the console.

## Features

- Interactive menu for selecting a data structure to build.
- Example insertions and display for each structure.
- Input validation and user-friendly prompts.

## Usage

1. **Build the project:**

   ```sh
   cargo build
   ```

2. **Run the project:**

   ```sh
   cargo run
   ```

3. **Follow the on-screen menu:**
   - Choose a number to build and view a data structure.
   - Example output is shown for each structure.
   - Enter `5` to exit the program.

## Example Menu

```
===== Data Structure Menu =====
1. Build a HashMap     (unordered, fast key lookup)
2. Build a BTreeMap    (ordered map by key)
3. Build a BTreeSet    (unique sorted values)
4. Build a BinaryHeap  (priority queue / max-heap)
5. Exit
===============================
Enter your choice:
```

## Requirements

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## License

This project is for educational purposes.
