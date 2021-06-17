---
id: index
slug: /
title: Introduction
---

**RustDuino** is a one of the 4 projects offered by Electronics Club in summer 2021 for Y20 batch students which introduces the concepts of Embedded programming using **Rust Language** and also covers **Arduino Architecture** from the very basics. The project is oriented towards making our own IoT Hemisphere for Arduino.

We will be making optimizations along the way in this project, especially because what currently happens is that C/C++ program is converted into an arduino language and then into binaries. We'll be directly interfacing them so that these multiple conversions are not needed.

**HAL (Hardware Abstraction Layer)** is a layer of programming that allows a computer Operating System to interact with a hardware device at a general level. AVR-GCC is a cross platform compiler which will allow us to compile code meant for arduino devices on our computers. We will be using these along the course of the project.  

![arduino](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/Arduino.gif?raw=true)

Understanding the Arduino Architecture at its core and developing a Rust Lang Crate ("library") for standard Arduino sensors and microcontrollers is the prime goal of this project. This will be accomplished using a **Hardware Abstraction Library** and the **AVR-GCC compiler**. RUST language offers memory safety, efficiency, among other benefits, making it essential in the IoT Hemisphere.

We would try to cover as many libraries as possible but the very fact of availability of a large number of features in Arduino makes this a vast never ending task. Our final aim is developing libraries for 2 chips specifically **Atmega4809p** (Arduino Every,Edge)  and **Atmega328p** (Arduino nano, uno).