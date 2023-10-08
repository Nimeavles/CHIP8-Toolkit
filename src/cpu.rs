/// Opcodes -> https://en.wikipedia.org/wiki/CHIP-8
use crate::memory::Memory;

const N_CPU_REGISTERS: u8 = 16;

#[allow(dead_code)]
pub struct CPU {
    pub registers: Vec<u8>,
    memory: Memory,
}

#[allow(dead_code, unused_variables)]
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: vec![0x0; N_CPU_REGISTERS as usize],
            memory: Memory::new(),
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

    fn add_operation(&mut self, x: u8, y: u8, val1: u16, val2: u16) -> (bool, u16) {
        println!("Add: Vx += Vy!");

        let x_register = self.registers[x as usize];
        let y_register = self.registers[y as usize];

        // Cast to u16 to avoid panicking when attempting an overflow
        if (x_register as u16 + y_register as u16) > 255 {
            return (true, 0);
        }

        self.registers[x as usize] = x_register + y_register;

        (false, val1 + val2)
    }

    pub fn run(&mut self) {
        loop {
            let opcodes = self.parse_opcode();

            match opcodes {
                (0x8, _, _, 0x4) => {
                    let (overflow, val) = self.add_operation(opcodes.1, opcodes.2, 252, 3);

                    if overflow {
                        self.registers[15] = 1;
                    }
                }
                (0, 0, 0, 0) => {
                    return;
                }
                _ => panic!("Opcode not identified!"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CPU;

    #[test]
    fn test_cpu_add() {
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
}
