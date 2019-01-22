mod cpu;
fn main() {
    let mut chip8 = cpu::CHIP8::init();
    chip8.load_program(String::from("roms/all_instructions.rom"));
    loop {
        println!("{}", chip8.dump_reg());
        chip8.cycle();
    }
}

