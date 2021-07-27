---
id: random
slug: /random
title: Random Number Generation
---

*even we don't know the output*

---

## Introduction
*Randomness is present everywhere, from the flow of water out of a sink to a dice roll. To succesfully model physical phenomenon or secure data using cryptography, a good source of true random numbers is necessary.*

Due to the deterministic nature of processors, it is almost impossible to generate true random numbers using only processor instructions.As more people are interested in the field of stochastic simulation and cryptography, and seek cheap platforms , the Arduino platform should be able to at least be able to close the gap between affordability and quality while creating Random Numbers.

![Pictorial Randomness](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/1f216bb8b1c4c457c215bf2d70213213dd667a87/random_comparison.png)
![you can check here for some more analysis data](https://www.random.org/analysis/)


## Current Methods
One can try extracting randomness directly from the analog pins by taking the least significant bit of analogRead().
It would be something like this - 

```c++
while (have less than 8 bits stored)  
    store the least significant bit of analogRead()  
    wait for an arbitrary amount of time
```    

![at 4 microseconds](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/06dabbf93c6d0bf2920718b5934a6b2175187e88/4us.png)
![at 16 microseconds](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/06dabbf93c6d0bf2920718b5934a6b2175187e88/16us.png)
![at 128 microseconds](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/06dabbf93c6d0bf2920718b5934a6b2175187e88/128us.png)
![at 256 microseconds](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/06dabbf93c6d0bf2920718b5934a6b2175187e88/256us.png)
![at 1024 microseconds](https://gist.githubusercontent.com/bloc97/b5831977ccfeae3aa71976686c9c8afa/raw/06dabbf93c6d0bf2920718b5934a6b2175187e88/1024us.png)

It is common knowledge that the level of entropy from using the analogRead() method on floating pins is very low, as they are only 210 possible values and their overall amplitude changes very slowly. So this naive method is flawed and a quick visual check reveals the periodicity of its output.


## Our Method of Extraction ->

We have created analog implementation in our code which will help us in generating random numbers.
First we will define some processes ->
1) - Bitwise XOR (abbreviated as XOR from here on) - XOR of all the bits of u8 numbers.
```rust
pub fn xor(a: u8, b: u8) -> u8 { /*code goes on */ }
```
2) - Shifting - Will randomly shift bits of the u8 and take the XOR of all.
```rust
pub fn xor_shift(a: u8) -> u8 { /* code goes on */ }
```
3) - Rotation - It is a seeding algorithm which read numbers 8 times from analofRead() which are XORed regularly. It starts our random number generation algorithm.
```rust
pub unsafe fn xor_rotate() -> u8 { /* code goes on */ }
```
4) - Push - These would be used to push the bits into a u8 according to direction bias.
```rust
pub fn push_right(val: u8, change: u8) -> u8 { /* code goes on */ }
pub fn push_left(val: u8, change: u8) -> u8 { /* code goes on */ }
```

For the implementation of both the methods for creating Random Number we create a special structure as - 

```rust
pub struct RandomNumberGenerator {
    pins: Pins,
    mpu: &'static mut MPU6050<'static>,
}

impl RandomNumberGenerator { /* Various functions provided */ }
```

#### METHOD 1 (USING ANALOG READ) ->
We define a **Von Neumann extractor** as a transformation that maps a non-uniform bitstream into a uniform bitstream by presuming that rising edges (01) are as probable as falling edges (10) and that there is no correlation between successive bits. Such a transformation can help us get rid of the low frequency periodic components of the previous method however it is slow.
We define a **Direct Sampling** as a transformation that maps a bitstream to another bitstream according to a defined XORing pattern where we create 2 rotate numbers and one of them is shifted then XORed with the other to give a result.Though it is quite fast but the results of this could be predicted for a particular environment after observing the results carefully.

So we will make a careful mix of **direct sampling** for *local randomness* and **Von Neumann extractor** for *global randomness* so that we have both the abilities in our generator.
So we start with creating some *directly sampled numbers* in u8 and then go for further manipulation.
We will improve the statistical properties of the XOR-Rotation algorithm by combining it with a quick 8-bit XORShift PRNG. In this case the small period cycle of a 8-bit PRNG does not matter since we will be re-seeding it at each step.
At the end Von Neumann Extractor algorithm is used on the already created and shifted number to add *globally random bits*.

```rust
pub fn generate_by_analog(&mut self) -> u8 { /* code goes on */ }
```

*Note: An interesting property of this method is the ability for the user to choose the sample rate. A slower sample rate will yield a better random result (due to internal interference having less impact on the randomness of analogRead()) while a faster sample rate guarantees a quick response. This will generate 8 random bits within a reasonable amount of time.*


#### METHOD 2 (USING MPU6050 GYROSCOPIC SENSOR) ->
The MPU6050 is a multipurpose Accelerometer and Gyroscope sensor module for the Arduino, it can read raw acceleration from 3 axis and raw turn rate from 3 orientations. Its acceleration sensor's noise level far surpasses its resolution, with at least 4 bits of recorded entropy.
We can achieve 8 bits of randomness using 4 bits that are not necessarily the same magnitude from each axis. We are supposing that the upper 2 bits are not always reliable, so we will XOR each axis' higher 2 bits with another axis' lower 2 bits, and vice-versa.

We will be using this ability of the sensor to generate a random number along with directly seeding the float numbers generated so that we can get a randomly generated u8 number.

```rust
pub fn generate_by_mpu(&mut self) -> u8 { /* code goes on */ }
pub fn generate_mpu() -> (u8, u8, u8, u8, u8, u8) { /* code goes on */ }
```
