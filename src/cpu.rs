/// Opcodes -> https://en.wikipedia.org/wiki/CHIP-8
use crate::memory::Memory;

const N_CPU_REGISTERS: u8 = 16;

#[allow(dead_code)]
pub struct CPU {
    registers: Vec<u8>,
    program_counter: u16,
    memory: Memory,
}

#[allow(dead_code, unused_variables)]
impl CPU {
    pub fn new() -> Self {
        Self {
            registers: vec![0x0; N_CPU_REGISTERS as usize],
            program_counter: 0x200, // 0x200 = 512 that is the system reserved memory
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

    fn parse_opcode(&self) -> (u8, u8, u8, u8) {
        let opcode = self.memory.read(2);

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        (c, x, y, d)
    }

    fn add_operation(&mut self, x: u8, y: u8, val1: u16, val2: u16) -> (bool, u16) {
        println!("Add: Vx += Vy!");

        if (val1 + val2) as u16 > 255 {
            return (true, 0);
        }

        // register x = 1
        self.registers[x as usize] = val1 as u8;
        // register y = 2
        self.registers[y as usize] = val2 as u8;
        // register x += register y
        self.registers[x as usize] += self.registers[y as usize];

        (false, val1 + val2)
    }

    pub fn run(&mut self) {
        let opcodes = self.parse_opcode();

        match opcodes.0 {
            0x8 => match opcodes.3 {
                4 => {
                    let (overflow, val) = self.add_operation(opcodes.1, opcodes.2, 252, 3);

                    if overflow {
                        self.registers[15] = 1;
                    }
                }
                _ => println!("Non reconigsed N"),
            },
            _ => panic!("Opcode not identified!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn test_cpu_add() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);
        cpu.run();

        assert_eq!(cpu.registers[3], 4);
        assert_eq!(cpu.registers[2], 3);
    }

    #[test]
    fn test_add_cpu_overflow() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);
        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }
}
