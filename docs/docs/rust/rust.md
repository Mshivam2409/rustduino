---
id: rust
slug: /rust
title: Intro to Rust
---

_The most loved programming language_

----

Rust is a programming language designed for performance, safety, and especially safe concurrency.
Rust is a statically and strongly typed systems programming language.
The biggest difference between Rust and other programming languages like C/C++ is that Rust offers safety by default.
High-level ergonomics and low-level control (such as memory usage) are often at odds in programming language design; Rust challenges that conflict.

Rust's ownership and type-system enforce safety at compile time.
In other words, Rust's compiler doesn't allow us to make subtle mistakes when writing code, especially related to memory management and data races.
Initially, it might feel daunting but in the long run, the strict checking at compile time helps us write code more confidently and faster without worrying about bugs related to memory access; which are notorious in languages like C/C++ and cause of many security vulnerabilities which we don't wan't especially for a vulnerable microcontroller chip.

Rust has been voted the most loved programming language in the [Stackoverflow Developer Survey](https://insights.stackoverflow.com/survey/2020) every year since 2016.

Here is a simple “Hello, world” program in Rust.

```rust
fn main() {
    println!("Hello, world");
}
```

----

## Rust's Ownership: A glance

Rust's central feature is ownership.
All programs have to manage the way they use the computer's memory.
Some languages have garbage collection that constantly looks for unused memory as the program runs; in others, we must explicitly allocate and free memory.
Rust uses a different approach.
Memory is managed through an ownership system with a set of rules that the compiler checks at compile time.
The best part about it is that none of the ownership features slows down your program while it is running.

The ownership system consists of three simple rules:

- Each value in Rust has a variable that's called the owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped (no longer accessible).

Although the above rules are simple, they have deep implications rest of the language.
