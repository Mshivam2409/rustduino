![arduino](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/assets/logo.png?raw=true)

**RustDuino** is a one of the 4 projects offered by Electronics Club in summer 2021 for Y20 batch students which introduces the concepts of Embedded programming using **Rust Language** and also covers **Arduino Architecture** from the very basics. The project is oriented towards making our own Chips feature implementation library for Arduino.

We will be making optimizations along the way in this project, especially because what currently happens is that C/C++ program is converted into an arduino language and then into binaries. We'll be directly interfacing them so that these multiple conversions are not needed.

**HAL (Hardware Abstraction Library)** is a layer of programming that allows a computer Operating System to interact directly with the hardware device at a general level. **AVR-GCC linker** is a cross platform compiler which will allow us to compile code meant for arduino devices on our computers. We will be using this along the course of the project.

**Communication Control Library** will be a part of the project where we will be implementing various communication protocols of our Arduino board so that it could be further interfaced with different sensors and peripheral devices according to the power sources allowed by the source code. We will be implementing both **USART (Universal Synchronous and Asynchronous serial Receiver and Transmitter )** and **I2C** communication protocols.

![arduino](https://github.com/Mshivam2409/RustDuino-Docs/blob/master/docs/embedded/images/Arduino.gif?raw=true)

Understanding the Arduino Architecture at its core and developing a Rust Lang Crate ("library") for standard Arduino sensors and microcontrollers is the prime goal of this project. This will be accomplished using the **Hardware Abstraction Library**, **Communication Control Library** which we will make and the **AVR-GCC compiler**.

RUST language offers both memory safety and efficiency among other benefits, making it essential for low level embedded programming .Also with very robust memory allocation and dis-allocation rules with unique ownership and borrowing rules the compiler itself provides sufficient check on the code before trying it on the microcontrollers so that there is absolutely no wrongly functioning code preventing any potential harm to the chip.

We would try to cover as many library features as possible but the very fact of availability of a large number of features in Arduino makes this a vast never ending task. Our final aim is developing libraries for 2 chips specifically **Atmega2560p** (Arduino Mega) and **Atmega328p** (Arduino nano, uno).

#### What you want next ?

- [I don't know how to start?](install.md)
- [What is rust?](rust/rust.md)
- [What is arduino?](arduino/index.md)
- [I know programming but what is embedded?](embedded/index_embedded.md)
- [Yeah I know the basics start straight-away](core/index_core.md)
