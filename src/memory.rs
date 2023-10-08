use std::vec;

#[allow(dead_code)]
const MAX_MEMORY_SIZE: usize = 10;
const MAX_STACK_SIZE: usize = 16;

/// Max size is 4096 bytes (4kb)
/// That means that usize size in chip8
/// is 12 bits, so 212 * 12 = 4096

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Stack {
    stack: Vec<u16>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: vec![0x0; MAX_STACK_SIZE],
        }
    }

    pub fn push(&mut self, address: u16, sp: u16) {
        self.stack[sp as usize] = address;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Memory {
    memory: Vec<u8>,
    pc: u16,
    pub read_pc: u16,
}

#[allow(dead_code)]
impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0x0; MAX_MEMORY_SIZE],
            pc: 0x000,
            read_pc: 0x000,
        }
    }

    pub fn write(&mut self, data: u16) {
        for i in data.to_le_bytes() {
            self.memory[self.pc as usize] = i;
            self.pc += 1;
        }
    }

    pub fn read(&mut self, size: u8) -> u16 {
        let mut bytes_readed: [u8; 2] = [0, 0];

        for i in 0..size {
            bytes_readed[i as usize] = self.memory[self.read_pc as usize];
            self.read_pc += 1;
        }

        let byte_1 = bytes_readed[0] as u16;
        let byte_2 = bytes_readed[1] as u16;

        byte_2 << 8 | byte_1
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn test_write_into_memory() {
        let mut mem: Memory = Memory::new();
        // 0x0ff7 = 4087
        mem.write(0x0ff7);

        assert_eq!(mem.read(2), 0x0ff7);
    }
}
