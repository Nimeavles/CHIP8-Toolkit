# CHIP 8 Interpreter

This is an experimental **CHIP8 Interpreter** written on rust.

## Supported Opcodes and Instructions

| Instruction           | Opcode     | Description                                  | Supported
| --------------------- | ---------- | -------------------------------------------- | --------------------
| `RET`                 | `0x00EE`   | Return from a function                       | :white_check_mark:
| `JP NNN`              | `0x1NNN`   | Jump to a given address                      | :white_check_mark:
| `CALL NNN`            | `0x2NNN`   | Call a function on a given address           | :white_check_mark:
| `SE Vx, NN`           | `0x3XNN`   | Skips the next instruction if Vx == NN       | :white_check_mark:
| `SNE Vx, NN`          | `0x4XNN`   | Skips the next instruction if Vx != NN       | :white_check_mark:
| `SE Vx, Vy`           | `0x5XY0`   | Skips the next instruction if Vx == Vy       | :white_check_mark:
| `LD Vx, NN`           | `0x6XNN`   | Move a value into a register                 | :white_check_mark:
| `ADD Vx, NN`          | `0x7XNN`   | Add the NN value to the Vx register          | :white_check_mark:
| `LD Vx, Vy`           | `0x8XY0`   | Move a register value into a register        | :white_check_mark: 
| `OR Vx, Vy`           | `0x8XY1`   | Bitwise OR Vx with Vy                        | :white_check_mark:
| `AND Vx, Vy`          | `0x8XY2`   | Bitwise AND Vx with Vy                       | :white_check_mark:
| `XOR Vx, Vy`          | `0x8XY3`   | Bitwise XOR Vx with Vy                       | :white_check_mark:
| `ADD Vx, Vy`          | `0x8XY4`   | Add the Vy register to the Vx value          | :white_check_mark:
| `SUB Vx, Vy`          | `0x8XY5`   | Sub the Vy register to the Vx value          | :white_check_mark:

## Usage

This proyect still being on an early development stage.

Soon will be posted how to use it!

###### Made by Nimeavles :heart: