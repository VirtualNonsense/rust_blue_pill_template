Bluepill Template 🔵💊
=========

This project aims to be an easy-to-use template to get started with rust on the stm32f103c8.

# Rust reference
Here are a couple of nice references in case you are just starting out
- [getting started with rust](https://www.rust-lang.org/learn/get-started)
- [rust book](https://doc.rust-lang.org/book/)
- [rust embedded book](https://docs.rust-embedded.org/book/)
- [RTIC - Reference](https://rtic.rs/1/book/en/)

# Setup
## Toolchain
 - [rustup](rustup.rs/) - Follow the instructions on this side to get rust up and running 
([further reading](https://www.rust-lang.org/learn/get-started))
 - Install target: `rustup target install thumbv7m-none-eabi`
 - In some optional tools
   - install buildutils used to dissect resulting binaries: `cargo install cargo-binutils`
     - [dependency] install llvm tools: `rustup component add llvm-tools-preview`
 - The project currently is configured to use a stlink v2. If you install [open ocd](https://openocd.org) it should be 
configured correctly. (assuming you are using clion)


# Project strucutre
A quick overview over the project structure that I came up with

## `./{{project-name}}`
This directory contains the main programm that will be build for the bluepill board. 

Here is a quick reference over the important locations and a quick description:
 - `.cargo/config`: contains the target configuration
 - `src/main.rs`: the program that will be run on the target
 - `cargo.toml`: project meta information and dependencies of the main program. **Use this file to change the project name**
 - `memory.x`: device specific information such as memory capacity
 - `strm32f103c8_blue_pill.cfg`: OCD Config for blue pill board
 - binary locations
   - `./target/thumbv7m-none-eabi/release/{{project-name}}`
   - `./target/thumbv7m-none-eabi/debug/{{project-name}}`

Unfortunately everything that is contained within this folder cannot be tested via the build in rust test kit, due
to the fact that everything will be build for the target architecture.
The testkit is not supported on embedded chips (it would not be reasonable to run on such devices anyway).

The easiest way to maneuver around this problem is to split the code up in different crates.
App should only contain the code that is necessary to hook everything up and therefor is not easily testable or in case 
of the used libraries already tested in the external crates.

## `./lib/`
This folder should contain all custom crates that should be tested. It might be reasonable to split up the code into 
crates as shown in `./lib/{{sublibrary}}/`.

### `./lib/{{sublibrary}}/src/lib.rs`
This file is used to expose the library to other crates.
Use it for your unittests as well. 
