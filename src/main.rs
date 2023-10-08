mod cpu;
mod memory;

use cpu::CPU;

/// Reads the opcode (eventually, from memory)
/// Decodes instruction
/// Dispatches execution of the operation to a specific function
/// Matches decoded instruction to known opcodes

#[allow(unused_doc_comments, unused_variables)]
fn main() {
    let mut cpu = CPU::new();

    /// 8XY4 -> Add Opcode
    /// Vx += Vy
    cpu.set_opcode(0x8014);
    cpu.set_opcode(0x8024);
    cpu.set_opcode(0x8034);

    cpu.registers[0 as usize] = 1;
    cpu.registers[1 as usize] = 2;
    cpu.registers[2 as usize] = 3;

    cpu.run();
}
