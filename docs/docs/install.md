---
id: install
slug: /install
title: Install
---

## Application Setup

*Get your tools*

----

We’ll start by creating a new application with cargo, and setting it to use
nightly Rust.

```bash
rustup toolchain install nightly-2021-01-07
rustup override set nightly-2021-01-07
rustup +nightly-2021-01-07 component add rustfmt-preview
```

We will need the `rust-src` crate for several functions. Now move over to
[Compiling and Linking](arduino/index.md)

```bash
rustup +nightly component add rust-src
```

