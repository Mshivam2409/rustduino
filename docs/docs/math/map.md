---
id: map
slug: /map
title: Continuous Data Map
---

*data type change is important but how without significant data loss?*

---

## Description

In any library we need to implement some mathematical features which might be useful in various ways in the rest of the library functions.
One of those is a map function which maps a integer number from one data type to another ( a 32 bit un signed integer to a 8 bit un signed integer for instance).
We can easily typecast values from a smaller datatype to a larger datatype without any data loss but if we do the reverse than some data goes out of memeory and thus results in either a loss of data or a wrong functioning due to wrong parameters ( which we seriously want to avoid in case of library functions).

So we create a map() function which maps a larger data type into a smaller one in uniform ratio of the ranges of the two types. Though we still can have some sought of data loss but the effect is greatly minimized.
The function goes like this - 

```rust
pub fn map(val: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    /* Uniform mapping from initial data type to final */
}
```

### A usage example
While reading the data from analog pins and writing the same data to analog write we require to change the data from 32 bit un signed integer to 8 bit unsigned integer which is done using this function as shown below -

```rust
use rustduino::math::map;

let a:u32 = { /* analog read implementation */ }

let b:u8 = map(a as u64,0,255,0,1023);
```
