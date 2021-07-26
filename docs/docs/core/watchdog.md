---
id: watchdog
slug: /watchdog
title: Watchdog
---

*Let's disable the house-keeper*

----

- A watchdog timer (WDT) is a hardware timer that automatically generates a
  system reset if the main program neglects to periodically service it. It is
  often used to automatically reset an embedded device that hangs because of a
  software or hardware fault.
- A watchdog timer sometimes called a computer operating properly or COP timer,
  or simply a watchdog is an electronic or software timer that is used to detect
  and recover from computer malfunctions.
- During normal operation, the application regularly restarts the watchdog timer
  to prevent it from elapsing, or "timing out". If, due to a hardware fault or
  program error, the application fails to restart the watchdog, the timer will
  elapse and generate a timeout signal. The timeout signal is used to initiate
  corrective actions. The corrective actions typically include placing the
  computer and associated hardware in a safe state and invoking a computer
  reboot.
- It is recommended to disable the watchdog timer, in case of non continous use.

## Structure Definitions

```rust
pub struct WatchDog { /* fields omitted */ }
```

WatchDog Represents a struct containing the register definition for watchdog
timer. These include:

* `MCUSR `*(MCU Status Register)*:  The MCU status register provides information on which reset source caused an MCU reset.

* `WDTCSR` *(Watchdog Timer Control Register)* : Used to control the action of timer on timeout.

  |           Mode Action           |               on Time-out               |
  | :-----------------------------: | :-------------------------------------: |
  |             Stopped             |                  None                   |
  |         Interrupt mode          |                Interrupt                |
  |        System reset mode        |                  Reset                  |
  | Interrupt and system reset mode | Interrupt, then go to system reset mode |

More about these registers and Watchdog timer can be found at [Section 10.9 of ATmega328P datasheet.](t.ly/dBh5)



## Trait Implementations

### Impl `new` for `Watchdog`

```rust
pub unsafe fn new() -> &'static mut Watchdog
```

#### Usage:

```rust
use rustduino::hal::watchdog;
let mut wdt = watchdog::new();

// Here wdt is the pointer to the struct Watchdog.
```

Return a struct pointer containing register definition of the watchdog timer.



### Impl `disable` for `Watchdog`

```rust
pub fn disable(&mut self)
```

#### Usage:

```rust
use rustduino::hal::watchdog;

let mut wdt = watchdog::new(); 	// creating pointer to the struct Watchdog.
wdt.disable();					// disabling the watchdog timer.
OR
watchdog::disable(watchdog::new());
```

Disables the watchdog timer by performing the following sequence of operations:

* *Disabling interrupts globally.*
* *Resetting Watchdog timer*
* *Disabling watchdog timer*
* *Restoing the previous interrupts state*
