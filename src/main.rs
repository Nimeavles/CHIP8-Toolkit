/// Opcodes -> https://en.wikipedia.org/wiki/CHIP-8

const N_CPU_REGISTERS: u8 = 16;

#[allow(dead_code)]
struct CPU {
    registers: Vec<u16>,
    current_operation: u16,
}

#[allow(dead_code, unused_variables)]
impl CPU {
    fn new() -> Self {
        Self {
            registers: vec![0x0; N_CPU_REGISTERS as usize],
            current_operation: 0x0,
        }
    }

    fn set_opcode(&mut self, opcode: u16) {
        self.current_operation = opcode;
    }

    fn parse_opcode(&self) -> (u8, u8, u8, u8) {
        let opcode = self.current_operation;

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        (c, x, y, d)
    }

    fn run(&mut self) {
        let opcodes = self.parse_opcode();

        match opcodes.0 {
            0x8 => match opcodes.3 {
                4 => {
                    println!("Add: Vx += Vy!");
                    // register x = 1
                    self.registers[opcodes.1 as usize] = 1;
                    // register y = 2
                    self.registers[opcodes.2 as usize] = 2;
                    // register x += register y
                    self.registers[opcodes.1 as usize] += self.registers[opcodes.2 as usize];

                    println!(
                        "Result of adding 1 + 2: {}",
                        self.registers[opcodes.1 as usize]
                    );
                }
                _ => println!("Non reconigsed N"),
            },
            _ => panic!("Opcode not identified!"),
        }
    }
}

/// [-] Initialize a CPU.
/// [-] Load u8 values into registers.
/// [-] Load the addition opcode into current_operation.
/// [-] Perform the operation

/// CPU OPCODE DECODING
/// 0X80F12
/// 8 -> First nibble (4bits)
/// 0 -> Second nibble  (x) -> Register to look
/// F -> Third nibble   (y) -> Register to look
/// 1 -> A 4 bit number (n) -> value to use ?
///

#[allow(unused_doc_comments, unused_variables)]
fn main() {
    let mut cpu = CPU::new();

    /// 8XY4 -> Add Opcode
    /// Vx += Vy
    cpu.set_opcode(0x8014);

    cpu.run();
}

#[cfg(test)]
mod test {
    use super::CPU;

    #[test]
    fn test_cpu_add() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);
        cpu.run();

        assert_eq!(cpu.registers[3], 3);
        assert_eq!(cpu.registers[2], 2);
    }
}
