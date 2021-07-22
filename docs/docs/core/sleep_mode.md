---
id: sleep_mode
slug: /sleep_mode
title: Sleep
---

Sleep modes enable the application to shut down unused modules in the MCU, thereby saving power. The AVR
provides various sleep modes allowing the user to tailor the power consumption to the application’s requirements.

- Various modes are
     1. IDLE  : Idle sleep mode       
     2. ADC   : ADC Noise Reduction
     3. PD    : Power-down    
     4. PS    : Power-save
     5. SBY   : Standby
     6. ESBY  : Extended Standby
- The Sleep Mode Control Register(SMCR) contains control bits for power management.
## Structure Definitions
---

  It contains registers to control the sleep modes. These bits select between the six available sleep modes in ATMEGA2560P.

```rust
  pub struct Sleep {
    smcr: u8,
}
```
## Implementations
---
### Impl `new` for `Sleep`

```rust
pub unsafe fn new() -> &'static mut Sleep {
        &mut *(0x53 as *mut Sleep)
}
```
Creating a new reference to the Sleep structure according to appropriate location.

### Impl `disable` for `Sleep`

```rust
 pub fn disable(&mut self) {/* fields omitted */}
```
Write appropriate value to register for disabling the sleep mode.
### Impl `enable` for `Sleep`

```rust
 pub fn enable(&mut self) {/* fields omitted */}
```
Write appropriate value to register for enabling the sleep mode.
### Impl `select_mode` for `Sleep`

```rust
 pub fn select_mode(&mut self, mode: Options) {/* fields omitted */}
```
 Setting the bits of SMCR register according to the sleep mode required.
