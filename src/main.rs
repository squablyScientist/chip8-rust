mod cpu;
fn main() {
    let mut chip8 = cpu::CHIP8::init();
    chip8.load_program(String::from("roms/Chip8 Picture.ch8"));
    loop {
        println!("{}", chip8.dump_reg());
        chip8.cycle();
        println!("{}", chip8.dump_display());
    }
}

