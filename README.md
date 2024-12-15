# Dangling Pointer in Rust

This repository demonstrates a common mistake in Rust: creating a dangling pointer by accessing memory after it has been freed.  The code showcases how to create a dangling pointer and its consequences.  A solution showing safe memory handling is provided. 

## Bug

The `bug.rs` file contains code that creates a vector, gets a raw pointer to its data, and then drops the vector.  Subsequently, it attempts to dereference the raw pointer, leading to undefined behavior (often a crash).

## Solution

The `bugSolution.rs` file provides a corrected version of the code. It uses techniques like borrowing or cloning to ensure that the memory is accessed safely within its lifetime.

## Running the Code

To run the example, clone this repo and execute the following commands:

```bash
cargo run --example bug
cargo run --example bugSolution
```