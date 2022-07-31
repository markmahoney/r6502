# A Rust-based 6502 Emulator
I want to learn Rust, and I'm interested in the 6502 processor, famously used in systems like the Apple II and the NES, so I decided to flail at both things at once. It's been a real mess so far.

Right now this codebase can:

- emulate access to RO and RW memory regions
- bundle those regions up into a contiguous 64k block
- load compiled 6502 programs into those memory regions
- implement enough 6502 opcodes to add two numbers together and store the results in memory

Eventually I'd love to build out full opcode support with robust tests and turn this into a library that can be used it to emulate more complicated systems. But let's be honest: I probably won't!

## Dependencies
It's Rust, so you'll need to install the Rust compiler: https://www.rust-lang.org/tools/install

To compile the 6502 assembler programs used for testing, you'll need `cc65` installed.

Mac:
```
brew install cc65
```

## Compiling and Running
`make` will compile the example 6502 assembly program into a file called add.bin, and then run `cargo run` to build and execute the emulator, which loads `add.bin` into 0x0000 in emulated RAM and attempts to run it. The program will crap out some aspects of the CPU and memory into stdout per clock cycle.

Eventually I would like to convert this to a library and have tests selectively load compiled assembly programs at launch.

### Resources
- http://archive.6502.org/datasheets/synertek_programming_manual.pdf
- https://www.masswerk.at/6502/6502_instruction_set.html
