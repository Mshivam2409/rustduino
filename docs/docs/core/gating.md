---
id: power
slug: /clock-gating
title: Power
---
The SIM(System Integration Module) is another piece of hardware we need for our microcontroller setup. Here, we have used a SIM to enable the appropriate clock gate to enable our I/O port.

## Function definitions
- Structure Sim represents a block of memory using structures representing registers in SIM.

```rust
pub struct Sim {/* fields omitted */}
```

## Implementation
### - Impl `new` for `Sim`
```rust
pub unsafe fn new() -> &'static mut Sim
```
Return a struct containing register definition of the Sim.

### - Impl `enable_clock` to enable `clock gate`.

```rust
pub fn enable_clock(&mut self, clock: Clock) 
```        
Enable clock gate by changing the corresponding memory location.
