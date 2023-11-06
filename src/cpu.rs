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

    /**
     * Writes the given opcode on memory
     */
    pub fn set_opcode(&mut self, opcode: u16) {
        self.memory.write(opcode);
    }

    /**
     * Load the Rom to the memory
     */
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.memcpy(rom);
    }

    // CPU OPCODE DECODING
    // 0X80F12
    // 8 -> First nibble (4bits)
    // 0 -> Second nibble  (x) -> Register to look
    // F -> Third nibble   (y) -> Register to look
    // 1 -> A 4 bit number (n) -> value to use ?

    /**
      Parses the given opcode and returns an
      **u8 tuple** with each nibble
    */
    fn parse_opcode(&mut self) -> (u8, u8, u8, u8) {
        let opcode = self.memory.read(2);

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        (c, x, y, d)
    }

    /**
     * Assign a value to a register. Vx = NN
     */
    fn set_value_to_register_operation(&mut self, register: u8, value: u8) {
        if register > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {register}");
        }

        self.registers[register as usize] = value;
    }

    /**
     * Move to Vx, Xy. (**LD Vx, Vy**)
     */
    fn move_y_register_value_to_x_instruction(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        self.registers[x as usize] = self.registers[y as usize];
    }

    /**
     * Performs the add operation
     */
    fn add_operation(&mut self, x: u8, y: u8) -> bool {
        println!("Add: V{x} += V{y}!");

        let x_register = self.registers[x as usize];
        let y_register = self.registers[y as usize];

        // Cast to u16 to avoid panicking when attempting an overflow
        if (x_register as u16 + y_register as u16) > 255 {
            return true;
        }

        self.registers[x as usize] += y_register;

        false
    }

    /**
     * Add a value to the Vx register
     */
    fn add_value_to_register_operation(&mut self, register: u8, value: u8) {
        if register > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {register}");
        }

        self.registers[register as usize] += value;
    }

    /**
     * Substract Vx - Vy
     */
    fn sub_operation(&mut self, x: u8, y: u8) -> bool {
        let x_register = self.registers[x as usize];
        let y_register = self.registers[y as usize];

        // Cast to i16 to avoid panicking when attempting an overflow
        if (x_register as i16 - y_register as i16) > 255
            || (x_register as i16 - y_register as i16) < 0
        {
            return true;
        }

        self.registers[x as usize] -= y_register;

        false
    }

    /**
     * Substract Vx - Vy and save it on Vx
     */
    fn sub_vx_minus_vy_operation(&mut self, x: u8, y: u8) {
        if self.registers[y as usize] < self.registers[x as usize] {
            self.registers[15] = 1;
            return;
        }

        self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
        self.registers[15] = 0;
    }

    /**
     * Bitwise Or Operation among registers
     */
    fn bitwise_or_operation(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        self.registers[x as usize] |= self.registers[y as usize];
    }

    /**
     * Bitwise And Operation among registers
     */
    fn bitwise_and_operation(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        self.registers[x as usize] &= self.registers[y as usize];
    }

    /**
     * Bitwise Xor Operation among registers
     */
    fn bitwise_xor_operation(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        self.registers[x as usize] ^= self.registers[y as usize];
    }

    /**
     * If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
     */
    fn bitwise_shr_operation(&mut self, x: u8) {
        self.registers[15] = self.registers[x as usize] >> 1;

        self.registers[x as usize] >>= 1;
    }

    /**
     * If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
     */
    fn bitwise_shl_operation(&mut self, x: u8) {
        self.registers[15] = (self.registers[x as usize] >> 7) & 1;

        self.registers[x as usize] <<= 1;
    }

    /**
     * Exec jp instruction
     */
    fn jp_operation(&mut self, address: u16) {
        self.memory.read_pc = address;
    }

    /**
     * Performs the call operation
     */
    fn call_operation(&mut self, address: u16) {
        // Store the current memory location on the stack.
        self.stack.push(self.memory.read_pc, self.stack_pointer);

        // Increment the stack pointer.
        self.stack_pointer += 1;

        // Jump to the called address
        self.memory.read_pc = address;
    }

    /**
     * Performs the ret operation
     */
    fn ret_operation(&mut self) {
        // Decrement the stack pointer.
        self.stack_pointer -= 1;

        // Retrieve and jump to the calling memory address from the stack.
        self.memory.read_pc = self.stack.pop(self.stack_pointer);
    }

    /**
     * Skips the next instruction if Vx = NN
     */
    fn skip_next_instruction_if_equals(&mut self, register: u8, value: u8) {
        if register > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {register}");
        }
        if self.registers[register as usize] == value {
            self.memory.read_pc += 2;
        }
    }

    /**
     * Skips the next instruction if Vx != NN
     */
    fn skip_next_instruction_if_not_equals(&mut self, register: u8, value: u8) {
        if register > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {register}");
        }

        if self.registers[register as usize] != value {
            self.memory.read_pc += 2;
        }
    }

    /**
     * Skips the next instruction if Vx == Vy
     */
    fn skip_next_instruction_if_registers_equals(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        if self.registers[x as usize] == self.registers[y as usize] {
            self.memory.read_pc += 2;
        }
    }

    /**
     * Skips the next instruction if Vx != Vy
     */
    fn skip_next_instruction_if_registers_not_equals(&mut self, x: u8, y: u8) {
        if x > N_CPU_REGISTERS || y > N_CPU_REGISTERS {
            panic!("Attempted to write on an undefined register: {x} or {y}");
        }

        if self.registers[x as usize] != self.registers[y as usize] {
            self.memory.read_pc += 2;
        }
    }

    fn parse_12bit_address(&self, opcode: Opcode) -> u16 {
        let op1 = opcode.1 as u16;
        let op2 = opcode.2 as u16;
        let op3 = opcode.3 as u16;

        op3 << 0 | op2 << 4 | op1 << 8
    }

    /**
     * Parse 2 nibbles into a 1 byte hex value
     */
    fn parse_8bit_address(&self, nibble1: u8, nibble2: u8) -> u8 {
        nibble1 << 4 | nibble2
    }

    /**
    * Enter into a loop with will fetch opcodes from memory,
    then it would be parsed and matched to be executed.
    * It is considered the entry point of the program
    */
    pub fn run(&mut self) {
        loop {
            let opcodes: Opcode = self.parse_opcode();

            let x_register = opcodes.1;
            let y_register = opcodes.2;

            match opcodes {
                // Ret instruction
                (0, 0, 0xE, 0xE) => {
                    self.ret_operation();
                }
                // Jp instruction. jp NNN
                (0x1, _, _, _) => {
                    let parsed_address_to_jump = self.parse_12bit_address(opcodes);
                    self.jp_operation(parsed_address_to_jump);
                }
                // Call instruction
                (0x2, _, _, _) => {
                    self.call_operation(self.parse_12bit_address(opcodes));
                }
                // if Vx == NN
                (0x3, _, _, _) => {
                    let parsed_value_to_compare = self.parse_8bit_address(opcodes.2, opcodes.3);
                    self.skip_next_instruction_if_equals(x_register, parsed_value_to_compare);
                }
                // if Vx != NN
                (0x4, _, _, _) => {
                    let parsed_value_to_compare = self.parse_8bit_address(opcodes.2, opcodes.3);
                    self.skip_next_instruction_if_not_equals(x_register, parsed_value_to_compare);
                }
                // if Vx == Vy
                (0x5, _, _, 0x0) => {
                    self.skip_next_instruction_if_registers_equals(x_register, y_register);
                }
                // Assign a value to a register. Vx = NN
                (0x6, _, _, _) => {
                    let value_to_set = self.parse_8bit_address(opcodes.2, opcodes.3);

                    self.set_value_to_register_operation(opcodes.1, value_to_set);
                }
                // Add a value to a register
                (0x7, _, _, _) => {
                    let value_to_set = self.parse_8bit_address(opcodes.2, opcodes.3);

                    self.add_value_to_register_operation(x_register, value_to_set);
                }
                // Move the Vy value to Vx
                (0x8, _, _, 0x0) => {
                    self.move_y_register_value_to_x_instruction(x_register, y_register);
                }
                // Bitwise OR operation
                (0x8, _, _, 0x1) => {
                    self.bitwise_or_operation(x_register, y_register);
                }
                // Bitwise AND operation
                (0x8, _, _, 0x2) => {
                    self.bitwise_and_operation(x_register, y_register);
                }
                // Bitwise XOR operation
                (0x8, _, _, 0x3) => {
                    self.bitwise_xor_operation(x_register, y_register);
                }
                // Add operation. Vx += Vy
                (0x8, _, _, 0x4) => {
                    let overflow = self.add_operation(x_register, y_register);

                    if overflow {
                        self.registers[15] = 1;
                    }
                }
                // SUB Vx, Vy
                (0x8, _, _, 0x5) => {
                    let overflow = self.sub_operation(x_register, y_register);

                    if overflow {
                        self.registers[15] = 1;
                    }
                }
                // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                (0x8, _, _, 0x6) => {
                    self.bitwise_shr_operation(x_register);
                }
                // Vx = Vy - Vx
                (0x8, _, _, 0x7) => {
                    self.sub_vx_minus_vy_operation(x_register, y_register);
                }
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                (0x8, _, _, 0xE) => {
                    self.bitwise_shl_operation(x_register);
                }
                // Skip next instruction if Vx != Vy
                (0x9, _, _, 0x0) => {
                    self.skip_next_instruction_if_registers_not_equals(x_register, y_register);
                }
                // Halt instruction
                (0, 0, 0, 0) => {
                    return;
                }
                _ => panic!("Opcode <{:?}> not identified!", opcodes),
            }
        }
    }
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
    fn test_cpu_add_overflow() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);

        cpu.registers[3] = 255;
        cpu.registers[2] = 1;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }

    #[should_panic]
    #[test]
    fn test_cpu_call_instruction() {
        std::panic::set_hook(Box::new(|_info| {
            // In order to not to show any panic info
        }));

        let mut cpu = CPU::new();

        cpu.set_opcode(0x8324);

        cpu.registers[3] = 2;
        cpu.registers[2] = 1;

        cpu.set_opcode(0x2200);

        cpu.run();
    }

    #[test]
    fn test_cpu_ret_instruction() {
        let mut cpu = CPU::new();

        // set the instructions to exec on 0x50 address
        cpu.memory.write_into(0x8324, 0x300);

        cpu.set_opcode(0x8014);

        cpu.registers[0 as usize] = 1;
        cpu.registers[1 as usize] = 2;

        cpu.registers[2 as usize] = 2;

        cpu.set_opcode(0x2300);
        cpu.set_opcode(0x8424);

        // Writes the ret opcode after the executed function on 0x100
        cpu.memory.write_into(0xEE, 0x302);

        cpu.run();

        assert_eq!(cpu.registers[3], 2);
        assert_eq!(cpu.registers[4], 2);
    }

    #[test]
    fn test_cpu_set_value_to_register_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x6012);

        cpu.run();

        // 0x12 -> 18
        assert_eq!(cpu.registers[0], 18);
    }

    #[test]
    fn test_cpu_jp_instruction() {
        let mut cpu = CPU::new();
        // Write into 0x200 -> 0x8014 which is an add
        cpu.memory.write_into(0x8014, 0x200);

        // ld registers[0], 1
        cpu.set_opcode(0x6001);
        // JP 0x200
        cpu.set_opcode(0x1200);

        cpu.run();

        // If not fails means that the add has been carried on,
        // so thats means that the code has jumped
        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_skip_instruction_if_equals() {
        let mut cpu = CPU::new();

        // LD V0, 0x01
        cpu.set_opcode(0x6001);

        // If V0 == 0x01 -> Skip 1 instruction
        cpu.set_opcode(0x3001);

        cpu.run();

        // 8 = 2 bytes + 2 bytes + 2 bytes (skipped instruction) + 2 bytes (Halt)
        assert_eq!(cpu.memory.read_pc, 0x200 + 8);
    }

    #[test]
    fn test_cpu_skip_instruction_if_not_equals() {
        let mut cpu = CPU::new();

        // If V0 == 0x01 -> Skip 1 instruction
        cpu.set_opcode(0x4001);

        cpu.run();

        // 6 = 2 bytes + 2 bytes (skipped instruction) + 2 bytes (Halt)
        assert_eq!(cpu.memory.read_pc, 0x200 + 6);
    }

    #[test]
    fn test_cpu_skip_instruction_if_registers_equals() {
        let mut cpu = CPU::new();

        // LD V0, 0x01
        cpu.set_opcode(0x6001);
        // LD V1, 0x01
        cpu.set_opcode(0x6101);

        // If V0 == V1 -> Skip 1 instruction
        cpu.set_opcode(0x5010);

        cpu.run();

        // 10 = 2 bytes + 2 bytes + 2 bytes + 2 bytes (skipped instruction) + 2 bytes (Halt)
        assert_eq!(cpu.memory.read_pc, 0x200 + 10);
    }

    #[test]
    fn test_cpu_move_y_register_value_to_x() {
        let mut cpu = CPU::new();

        // LD V1, 0x01
        cpu.set_opcode(0x6101);

        // LD V0, V1
        cpu.set_opcode(0x8010);

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_bitwise_or_operation() {
        let mut cpu = CPU::new();

        // LD V1, 0x01
        cpu.set_opcode(0x6101);

        // OR V0, V1
        cpu.set_opcode(0x8011);

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_bitwise_and_operation() {
        let mut cpu = CPU::new();

        // LD V1, 0x01
        cpu.set_opcode(0x6101);

        // AND V0, V1
        cpu.set_opcode(0x8012);

        cpu.run();

        assert_eq!(cpu.registers[0], 0);
    }

    #[test]
    fn test_cpu_bitwise_xor_operation() {
        let mut cpu = CPU::new();

        // LD V1, 0x01
        cpu.set_opcode(0x6101);

        // XOR V0, V1
        cpu.set_opcode(0x8013);

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_sub_operation() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8015);

        cpu.registers[0] = 2;
        cpu.registers[1] = 1;

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_sub_overflow_operation() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8015);

        cpu.registers[1] = 1;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_cpu_add_value_to_register() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x7001);

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_shr_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8016);

        cpu.registers[0] = 4;

        cpu.run();

        assert_eq!(cpu.registers[0], 2);
    }

    #[test]
    fn test_cpu_shr_vf_set_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8016);

        cpu.registers[0] = 2;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_cpu_sub_y_minus_x_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8017);

        cpu.registers[0] = 2;
        cpu.registers[1] = 3;

        cpu.run();

        assert_eq!(cpu.registers[0], 1);
    }

    #[test]
    fn test_cpu_sub_y_minus_x_vf_set_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x8017);

        cpu.registers[0] = 3;
        cpu.registers[1] = 2;

        cpu.run();

        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_cpu_shl_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x801E);

        cpu.registers[0] = 72;

        cpu.run();

        assert_eq!(cpu.registers[0], 144);
    }

    #[test]
    fn test_cpu_shl_vf_instruction() {
        let mut cpu = CPU::new();

        cpu.set_opcode(0x801E);

        cpu.registers[0] = 218;

        cpu.run();

        // Due to registers size, the result is 180.
        assert_eq!(cpu.registers[0], 180);
        assert_eq!(cpu.registers[15], 1);
    }

    #[test]
    fn test_cpu_skip_instruction_if_registers_not_equals() {
        let mut cpu = CPU::new();

        // LD V0, 0x01
        cpu.set_opcode(0x6001);
        // If V0 == V1 -> Skip 1 instruction
        cpu.set_opcode(0x9010);

        cpu.run();

        // 8 = 2 bytes + 2 bytes + 2 bytes (skipped instruction) + 2 bytes (Halt)
        assert_eq!(cpu.memory.read_pc, 0x200 + 8);
    }
}
