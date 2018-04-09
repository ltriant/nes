## nes

A tiny NES emulator

## Summary

This is a NES emulator, aiming to be tiny in many ways, from its resource footprint, to its code and design. We'll see how that pans out...

## To Do

* [x] read 6502 docs
* [x] read how NES used the 6502
* [ ] come up with a better name
* [ ] implement 6502 CPU
 * [x] memory mapping (RAM)
 * [x] registers + PC
 * [x] basic iNES parser
 * [x] opcodes + addressing modes structure
 * [x] iNES parser in its own module
 * [ ] figure out how the mappers will fit into this design
 * [ ] complete the iNES parser
 * [x] calculate cycle counts
 * [x] implement the rest of the instructions
 * [ ] illegal opcodes
* [ ] something something PPU
* [ ] something something APU
* [ ] ???
* [ ] profit
