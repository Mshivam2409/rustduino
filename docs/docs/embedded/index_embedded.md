---
id: index_embedded
slug: /embedded
title: Introduction
---

Unlike with typical desktop or server applications, embedded programs do not have an operating system to provide them with hardware control. Instead, they must access the hardware directly. The exact process for hardware control varies depending on the type of processor in use. 

We directly write data to the registers in order to access the hardware and for the AVR microcontroller that we’re using, we access the hardware through memory mapped registers. Memory mapping is assigning a special memory address which, when read from or written to, interacts with a hardware device instead of RAM.

Writing to arbitrary memory addresses requires unsafe Rust. So, one of our goals will be to use Rust’s language features to create safe interfaces for these unsafe memory accesses.
