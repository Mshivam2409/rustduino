---
id: memory-mapped-io
slug: /memory-mapped-io
title: Memory Mapped I/O
---

As a CPU needs to communicate with the various memory and input-output devices
(I/O), and as we know, data between the processor and these devices flow with
the help of the system bus. There are three ways in which system bus can be
allotted to them :

1. A separate set of address, control, and data bus to I/O and memory.
2. Have a common bus (data and address) for I/O and memory but different control
   lines.
3. Have a common bus (data, address, and control) for I/O and memory.

**The first case is simply because both have a different set of address space
and instruction but require more buses.** **Isolated I/O –** Then we have
Isolated I/O in which we have a common bus(data and address) for I/O and memory
but separate read and write control lines for I/O. So when CPU decodes the
instruction, then if data is for I/O, it places the address on the address line
and sets I/O to read or write control line on due to which data transfer occurs
between CPU and I/O. As the address space of memory and I/O is isolated, and the
name is so. The address for I/O here is called ports. Here we have different
read-write instructions for both I/O and memory. **Memory Mapped I/O –**
Memory-mapped I/O uses the same address space to address both memory and I/O
devices. The memory and registers of the I/O devices are mapped to (associated
with) address values. So when the CPU accesses an address, it may refer to a
portion of physical RAM, or it can instead refer to memory of the I/O device.

![mio1](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/mio1.png?raw=true)

![mio2](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/mio2.jpg?raw=true)

Memory-mapped I/O gives us a unified address space for both, memory and I/O

###### ADVANTAGES OF MEMORY-MAPPED I/O

Merits of memory-mapped I/O is that, by discarding the extra complexity that
port I/O brings, a CPU requires less internal logic and is thus cheaper, faster,
easier to build, consumes less power, and can be physically smaller; this
follows the basic tenets of reduced instruction set computing and is also
advantageous in embedded systems. The other advantage is that, because regular
memory instructions are used to address devices, all of the CPU's addressing
modes are available for the I/O as well as the memory, and instructions that
perform an ALU operation directly on a memory operand (loading an operand from a
memory location, storing the result to a memory location, or both) can be used
with I/O device registers as well.

###### Memory-mapped I/O in Processors

The most common place you see memory-mapped IO is inside a processor. A great
example is the **PORT** registers of an ATmega microcontroller (the ones used by
Arduino). When you write code for these processors, you can write something like
the following. _1 PORTB = 0xAA;_ This will set the 8 IO pins designated to
**PORTB** to the value 0xAA. However, in your code, **PORTB** is just a macro
and is a pointer to a special memory address. This address in memory doesn't
simply map to RAM but also maps to an IO peripheral that takes the value and
outputs it to the IO pins. Without memory-mapped IO, the microcontroller would
have no way to input or output any data!.

**DATA MEMORY MAP OF ATMEGA380P**

![mio3](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/mio3.png?raw=true)

**SYSTEM MEMORY MAP OF ATMEGA4809P**

![mio4](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/mio4.png?raw=true)
