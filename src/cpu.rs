/// Opcodes -> https://en.wikipedia.org/wiki/CHIP-8
use crate::memory::{Memory, Stack};

const N_CPU_REGISTERS: u8 = 16;

type Opcode = (u8, u8, u8, u8);

#[derive(Debug)]
#[allow(dead_code)]
pub struct CPU {
    pub registers: Vec<u8>,
    memory: Memory,
    stack_pointer: u16,
    stack: Stack,
}

#[allow(dead_code, unused_variables)]
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: vec![0x0; N_CPU_REGISTERS as usize],
            memory: Memory::new(),
            stack: Stack::new(),
            stack_pointer: 0x0,
        }
    }

    pub fn set_opcode(&mut self, opcode: u16) {
        self.memory.write(opcode);
    }

    /// CPU OPCODE DECODING
    /// 0X80F12
    /// 8 -> First nibble (4bits)
    /// 0 -> Second nibble  (x) -> Register to look
    /// F -> Third nibble   (y) -> Register to look
    /// 1 -> A 4 bit number (n) -> value to use ?

    fn parse_opcode(&mut self) -> (u8, u8, u8, u8) {
        let opcode = self.memory.read(2);

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        (c, x, y, d)
    }

    fn add_operation(&mut self, x: u8, y: u8) -> bool {
        println!("Add: Vx += Vy!");

        let x_register = self.registers[x as usize];
        let y_register = self.registers[y as usize];

        // Cast to u16 to avoid panicking when attempting an overflow
        if (x_register as u16 + y_register as u16) > 255 {
            return true;
        }

        self.registers[x as usize] = x_register + y_register;

        false
    }

    fn call_operation(&mut self, address: u16) {
        // Store the current memory location on the stack.
        self.stack.push(self.memory.read_pc, self.stack_pointer);

        // Increment the stack pointer.
        self.stack_pointer += 1;

        // Jump to the called address
        self.memory.read_pc = address;
    }

    fn parse_12bit_address(&self, opcode: Opcode) -> u16 {
        let op1 = opcode.1 as u16;
        let op2 = opcode.2 as u16;
        let op3 = opcode.3 as u16;

        op3 << 0 | op2 << 4 | op1 << 8
    }

    pub fn run(&mut self) {
        loop {
            let opcodes: Opcode = self.parse_opcode();

            match opcodes {
                (0x8, _, _, 0x4) => {
                    let overflow = self.add_operation(opcodes.1, opcodes.2);

                    if overflow {
                        self.registers[15] = 1;
                    }
                }
                (0x2, _, _, _) => {
                    self.call_operation(self.parse_12bit_address(opcodes));
                }
                (0, 0, 0, 0) => {
                    return;
                }
                _ => panic!("Opcode not identified!"),
            }
        }
    }

    // RETURN
    // Decrement the stack pointer.
    // Retrieve the calling memory address from the stack.
    // Set the current memory location to the intended memory address
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_cpu_add_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);

        cpu.registers[3] = 3;
        cpu.registers[2] = 2;

        cpu.run();

        assert_eq!(cpu.registers[3], 5);
        assert_eq!(cpu.registers[2], 2);
    }

    #[test]
    fn test_add_cpu_overflow() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);

        cpu.registers[3] = 255;
        cpu.registers[2] = 1;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_cpu_call_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);

        cpu.registers[3] = 2;
        cpu.registers[2] = 1;

        cpu.set_opcode(0x2000);

        cpu.run();
    }
}
