---
id: interrupts
slug: /interrupts
title: Interrupts
---

*We have something important to fix !*

---

- Interrupts is a mechanism by which an I/O or instruction can suspend the normal execution of the 
  processor and gets itself serviced like it has higher priority. 
- Global interrupts configured in the chips is controlled here. The Status Register contains information about the result of
  the most recently executed arithmetic instruction. This information can be used for altering program flow in order to perform
  conditional operations.
- The Global Interrupt Enable bit must be set for the interrupts to be enabled. The individual interrupt enable control is then performed
  in separate control registers. If the Global Interrupt Enable Register is cleared, none of the interrupts are enabled independent of the
  individual interrupt enable settings.

## Structure Definitions
---

  This contains the registers to be manipulated for controlling global interrupts setup.
  This represents struct for Interrupt and is used to control sreg register.

```rust
  pub struct Interrupt {
    sreg: u8,
  }
```

## Trait Implementations
---- 
 
### Impl `new` for `Interrupt`

```rust
pub unsafe fn new() -> &'static mut Interrupt {
    &mut *(0x5F as *mut Interrupt)
}
```

Returns new struct of Global_Interrupts.

### Impl `disable` for `Interrupt`

```rust
 pub fn disable(&mut self){ /* fields omitted */}
```
This function Disable global interrupts also known as CLI

### Impl `enable` for `Interrupt`

```rust
pub fn enable(&mut self){ /* fields omitted */}
```
This function Enable global interrupts also known as SEI.
