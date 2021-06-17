---
id: power
slug: /clock-gating
title: Power
---
The SIM(System Integration Module) is another piece of hardware we need for our microcontroller setup. Here, we have used a SIM to enable the appropriate clock gate to enable our I/O port.

In this part we have enabled clock gate to Port C.

![mio1](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/core/images/clock gate.jpg?raw=true)

                  Fig.  Memory representation of System Clock Gating Control Register 5 
Function definitions
Structure Sim represents a block of memory using structures representing registers in SIM.

pub struct Sim {/* fields omitted */}
Function to enable clock gate in the memory location corre .

pub fn enable_clock(&mut self, clock: Clock) {
        unsafe {/*feilds omitted*/}
        }
Implementation
Impl new for sim
pub unsafe fn new() -> &'static mut Sim
Return a struct containing register definition of the Sim.
