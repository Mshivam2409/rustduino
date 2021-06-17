---
id: install
slug: /install
title: Install
---

## Application Setup

We’ll start by creating a new application with cargo, and setting it to use
nightly Rust.

```
$ cargo new --bin teensy
$ cd teensy
$ rustup override set nightly
```rust

We will need the `rust-src` crate for several functions. Now move over to
[Compiling and Linking](arduino/index.md)

```
$ rustup +nightly component add rust-src
```rust
