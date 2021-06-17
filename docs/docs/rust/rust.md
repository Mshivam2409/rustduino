---
id: rust
slug: /rust
title: Intro to Rust
---

Rust is a programming language designed for performance, safety, and especially safe concurrency.
Rust is statically and strongly typed systems programming language.
The biggest difference between Rust and other programming languages like C/C++ is that Rust offers safety by default.
High-level ergonomics and low-level control (such as memory usage) are often at odds in programming language design; Rust challenges that conflict.

Rust's ownership and type-system enforces safety at compile time.
In other words Rust's compiler doesn't allow us to make subtle mistakes when writing code.
Initially it might feel daunting but in the long run the strict checking at compile time helps us write code more confidently without worrying about memory errors which are notorious in languages like C.

Rust has been voted the most loved programming language in the [Stackoverflow Developer Survey](https://insights.stackoverflow.com/survey/2020) every year since 2016.

Here is a simple “Hello, world” program in Rust.

```rust
fn main() {
    println!("Hello, world");
}
```
