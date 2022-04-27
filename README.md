Bluepill Template 🔵💊
=========

The knx-wizad is currently able to parse and transcribe knx messages 
(data requests and extended date requests).
In the future some onboard inference might be included as well.

# Rust reference
There are a couple great source to learn rust. I will link everything that is relevant for this project
- [getting started with rust](https://www.rust-lang.org/learn/get-started)
- [rust book](https://doc.rust-lang.org/book/)
- [rust embedded book](https://docs.rust-embedded.org/book/)
- [RTIC - Reference](https://rtic.rs/1/book/en/)

# Setup
## Toolchain
 - [rustup](rustup.rs/) - Follow the instructions on this side to get rust up and running ([further reading](https://www.rust-lang.org/learn/get-started))
 - Install target `rustup target install thumbv7m-none-eabi`
 - The project currently is configured to use a stlink v2. If you install [open ocd](https://openocd.org) it should be configured correctly. (assuming it's you are using clion)



# Project strucutre
Unfortunately it is currently not possible to configure the building process, and it's dependencies for
the binaries and the tests separate. This makes writing unit tests pretty annoying or by default impossible.
I solved this issue by having the Firmware in one cargo project and everything I would like to unittests in a different 
library. 

## `./app/`
This directory contains the main programm that will be build for the bluepill board. 
It will also contain the binaries in `./app/target/thumbv7m-none-eabi/release`.
It contains
 - `.cargo/config`: contains the target configuration
 - `src/main.rs`: the program that will be run on the target
 - `cargo.toml`: project meta information and dependencies of the main program
 - `memory.x`: device specific information such as memory
 - `strm32f103c8_blue_pill.cfg`: OCD Config for blue pill board

Unfortunately everything that is contained within this folder cannot be tested via the build in rust test kit, due
to the fact that everything will be build for the target architecture.
The testkit is not supported on embedded chips (it would not be reasonable to run on such devices anyway).

The easiest way to manover around this problem is to split the code up in different crates.
App should only contain the code that is necessary to hook everything up and therefor is not easily testable or in case 
of the used libraries already tested in the external crates.

## `./lib/`
This folder should contain all custom crates that should be tested. It might be reasonable to split up the code into 
crates as shown in `./lib/example_lib/`.

### `./lib/example_lib/src/lib.rs`
This is file is used to expose the library to other crates.
Use it for your unittests as well
