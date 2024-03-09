# opgen
## Overview
This module is used to generate the opcode rust code for the specified device

## Inspiration
* The code generation is inspired by https://github.com/floooh/chips which generates Z80 opcode tables for C in Python. 
* Decoding rules for the Z80 are here: http://www.z80.info/decoding.htm
* Evalexpr: https://github.com/ISibboI/evalexpr

## TODO
- [ ] Investigate the possibility of moving the yaml definition to rust procedural macros