# mimic
## Overview
This is another experimental emulator that will attempt to emulate a few different retro systems. The first system will be a Gameboy emulator that will use learnings from my OGBE project.

## Todo
### Disassembly
-[ ] Handle negative relative jump opcode parameters (eg; JR)

## Inspirations
* Z80 - https://floooh.github.io/2017/12/10/z80-emu-evolution.html

## References
### Z80
* Opcode decoding: http://www.z80.info/decoding.htm#upfx
* Opcode decoding: https://github.com/floooh/chips/blob/master/codegen/z80_gen.py
* Opcode table: https://gbdev.io/gb-opcodes//optables/octal
* Datasheet: https://gekkio.fi/files/gb-docs/gbctr.pdf or http://www.z80.info/zip/z80cpu_um.pdf

### Gameboy
* Pan Docs: https://gbdev.io/pandocs/#the-cartridge-header
* Boot rom disassm: https://gist.github.com/knightsc/ab5ebda52045b87fa1772f5824717ddf

### Gameboy Advance
* Decoding ARM7: https://www.gregorygaines.com/blog/decoding-the-arm7tdmi-instruction-set-game-boy-advance/
* GBA Docs: https://mgba-emu.github.io/gbatek/#armopcodespsrtransfermrsmsr
* GBA Emulator: https://github.com/michelhe/rustboyadvance-ng/tree/master