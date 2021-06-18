---
id: unsafe
slug: /unsafe
title: Unsafe Rust
---

*The dark arts of Unsafe Rust*

----

*Unsafe rust is perhaps best thought of as an entirely new programming language, hidden in the dark depth of the Rust.
Unsafe Rust is like a superset of Rust; it allows you to do all things Rust can do, as well as a few other things that are too wild for the compiler to verify.
Unsafe Rust unleashes indescribable horrors that shatter your psyche and set your mind adrift in the unknowingly infinite cosmos.*

With that said Unsafe Rust does not disable the ownership system or any of Rust's safety checks.
It only gives you five additional superpowers that Rust doesn't have:

* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of `union`s

----

## Using Unsafe Rust

To use Unsafe Rust, enclose the code inside an `unsafe` block

```rust
unsafe {
    // unsafe code
}
```

An unsafe function

```rust
unsafe fn dangerous() {
    // function body
}
```

----

## Unsafe Rust in Embedded

In embedded programming you interact with hardware by writing to specific memory addresses.
Unfortunately Rust doesn't allow writing to arbitrary memory addresses; so, we have to use Unsafe Rust.
One of our goals in this project is to provide safe APIs (Application Programming Interfaces) for these unsafe memory accesses; hence isolating unsafe code as much as possible within safe abstractions.

In addition, `unsafe` code isn't necessarily dangerous or definitely have memory safety problems.
The intent is that the programmer should ensure that all the `unsafe` code accesses memory in a valid way.
It is trickier to get `unsafe` code correct because the compiler cannot uphold memory safety.
Separating `unsafe` code from safe code makes it easier to track down the source of bugs faster when discovered.
