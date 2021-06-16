---
id: registers
slug: /registers
title: Registers
---
A Register is just a location in memory that you can write data to or read data from. They are the building blocks of RAM

The data RAM on a microcontroller is organized into several “registers”, each with its own unique “address”. A RAM register on an 8 bit microcontroller can hold a total of 8 bits, or one byte of data. A typical RAM space specification may specify that it is 256 x 8. This means that there are a total of 256 registers in the RAM, and those registers can hold 8 bits each.

There are also **Special Function Registers** (or simply SFR's) on a microcontroller are just like the registers in data RAM. You can write data to them as well as read data from them. Where they differ is that some SFR’s directly control the on-chip hardware on the microcontroller while others are controlled by the on-chip hardware on the microcontroller.

Each bit in an SFR is assigned to a function. In the SFR’s you have control bits and flag bits. Control bits are like “switches” that turn a function on or off depending on if you write a 1 or a 0 to that bit location in the SFR. Flag bits are like “indicator lights” that indicate whether or not a given condition exists depending on whether the flag bit is a 1 or a 0. Control bits directly control the hardware. Flag bits are controlled by the hardware. In any given program, we typically write to control bits while we read flag bits (some flag bits must be manually cleared by writing to them depending on the microcontroller).

Each piece of hardware on the microcontroller will have at least 1 SFR assigned to it. Some hardware may have several SFR’s assigned to it.More details can be found on the microcontrollers datasheet.