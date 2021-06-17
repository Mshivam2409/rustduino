---
id: watchdog
slug: /watchdog
title: Watchdog
---
* A watchdog timer (WDT) is a hardware timer that automatically generates a system reset if the main program neglects to periodically service it. It is often used to automatically reset an embedded device that hangs because of a software or hardware fault.
* A watchdog timer sometimes called a computer operating properly or COP timer, or simply a watchdog is an electronic or software timer that is used to detect and recover from computer malfunctions. 
* During normal operation, the computer regularly restarts the watchdog timer to prevent it from elapsing, or "timing out". If, due to a hardware fault or program error, the computer fails to restart the watchdog, the timer will elapse and generate a timeout signal. The timeout signal is used to initiate corrective actions. The corrective actions typically include placing the computer and associated hardware in a safe state and invoking a computer reboot.

![watchdog](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/assets/Watchdog_timer_IC.jpg?raw=true)

```use core;
impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
         //return address of the first register
    }

    pub fn disable(&mut self) {
        unsafe{
            core::ptr::write_volatile(&mut self.unlock, 0xC520);
            __nop();
            core::ptr::write_volatile(&mut self.unlock, 0xD928);
            __nop();
            let mut ctrl_disable = core::ptr::read_volatile(&self.stctrlh);
            core::ptr::write_volatile(&mut self.stctrlh, ctrl_disable & 0 );
        }
    }
}
```

fn disable is the function used :

1. To unlock the watchdog for modification.
2. Then to disable the watchdog i.e the function changes the zeroth bit in the 16-bit value ,keeping others same. Here it's done using AND gate.  