---
id: why-rust
slug: /why-rust
title: Why Rust?
---

* **High performance**: Rust is blazing fast and memory-efficient, with no runtime or garbage collector. Embedded devices are low resource devices, they cannot handle heavy runtime and garbage collectors. Rust helps us write fast and efficient programs making it a good choice for embedded programming.

* **Safety**: Embedded devices are mission-critical and must perform correctly and must not perform incorrectly. Well you don't want your toaster to behave like a vacuum cleaner. Rust’s rich type-system and ownership model guarantees memory-safety and thread-safety, both of which are essential for a program to function correctly.

* **Powerful static analysis**: Rust’s type system prevents data races at compile time. The type system can also be used to check other properties at compile time; reducing the need for checks at runtime in some cases. When applied to embedded programs these static checks can be used, for example, to enforce that configuration of I/O is done properly. The ownership model can be used to ensure that only certain parts of a program can modify a peripheral.

* **Fearless concurrency**: Simply put concurrency is the ability of different parts of a program to be executed at the same time or out-of-order. In an embedded context, this includes:
  * interrupt handlers,
  * multi-threading,
  * multiple-core microprocessors.

  Since, many embedded systems have to deal with interrupts, concurrency will come up sooner or later, and it’s where many subtle and difficult bugs can occur. Rust provides a number of abstractions and safety guarantees that help us write correct code.

* **Portability**: In embedded environments portability is an important topic. Every vendor and even each family from a single manufacturer offers different capabilities and similarly the ways to interact with peripherals will vary. A common way to deal with this is building Hardware Abstraction Layers (HALs).
