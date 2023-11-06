mod cpu;
mod memory;

use std::{
    env,
    fs::File,
    io::{BufReader, Read, Result},
};

use cpu::CPU;

fn main() -> Result<()> {
    let path_to_rom = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("\n\u{001b}[31mError (Missing argument) => path\n\u{001b}[32mUsage: cargo run <my_file.ch8>\u{001b}[0m");
            std::process::exit(1);
        }
    };

    let mut file = BufReader::new(match File::open(path_to_rom) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("\u{001b}[31mError: {err}\u{001b}[0m");
            std::process::exit(1);
        }
    });

    let mut buf = CPU::create_buffer();
    let mut cpu = CPU::new();

    match file.read(&mut buf) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("\u{001b}[31mError: {err}\u{001b}[0m");
            std::process::exit(1);
        }
    };

    cpu.load_rom(&buf);

    cpu.run();

    Ok(())
}
