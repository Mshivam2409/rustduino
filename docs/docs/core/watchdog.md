---
id: watchdog
slug: /watchdog
title: Watchdog
---

- A watchdog timer (WDT) is a hardware timer that automatically generates a
  system reset if the main program neglects to periodically service it. It is
  often used to automatically reset an embedded device that hangs because of a
  software or hardware fault.
- A watchdog timer sometimes called a computer operating properly or COP timer,
  or simply a watchdog is an electronic or software timer that is used to detect
  and recover from computer malfunctions.
- During normal operation, the computer regularly restarts the watchdog timer to
  prevent it from elapsing, or "timing out". If, due to a hardware fault or
  program error, the computer fails to restart the watchdog, the timer will
  elapse and generate a timeout signal. The timeout signal is used to initiate
  corrective actions. The corrective actions typically include placing the
  computer and associated hardware in a safe state and invoking a computer
  reboot.

## Function Definitions

WatchDog Represents a struct containing the register definition for watchdog
timer.

```rust
pub struct WatchDog { /* fields omitted */ }
```

## Implementations

### Impl `new` for `Watchdog`

```rust
pub unsafe fn new() -> &'static mut Watchdog
```

Return a struct containing register definition of the watchdog timer.

#### `disable` for `Watchdog`

```
pub fn disable(&mut self)
```

1. Unlock the watchdog for modification.
2. Disables the watchdog.

