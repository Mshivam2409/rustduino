---
id: micromath
slug: /micromath
title: MicroMath
---

Mircomath is Embedded-friendly (i.e. `no_std`) Rust math library featuring fast,
safe floating point approximations for common arithmetic operations,
trigonometry, 2D/3D vector types, statistical analysis, and quaternions.

It optimizes for performance and small code size at the cost of precision.

This documentation is available here [here](https://docs.rs/micromath/)

## Features

- [`f32` extension]:

  - Fast approximations:
    - [asin]
    - [acos]
    - [atan]
    - [atan2]
    - [cos]
    - [hypot]
    - [inv]
    - [invsqrt]
    - [ln]
    - [log]
    - [log2]
    - [log10]
    - [powf]
    - [exp]
    - [sin]
    - [sqrt]
    - [tan]
  - `std` polyfills:
    - [abs]
    - [ceil]
    - [floor]
    - [round]
    - [trunc]
    - [fract]
    - [copysign]

- [Algebraic vector types]:
  - 2D:
    - [I8x2]
    - [I16x2]
    - [I32x2]
    - [U8x2]
    - [U16x2]
    - [U32x2]
    - [F32x2]
  - 3D:
    - [I8x3]
    - [I16x3]
    - [I32x3]
    - [U8x3]
    - [U16x3]
    - [U32x3]
    - [F32x3]
- [Statistical analysis]:
  - [mean]
  - [stddev]
  - [trim]
  - [variance]
- [Quaternions]
