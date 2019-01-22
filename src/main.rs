use std::{thread, time};
mod cpu;
fn main() {
    let mut chip8 = cpu::CHIP8::init();
    chip8.load_program(String::from("roms/BC_test.ch8"));
    loop {
        println!("{}", chip8.dump_reg());
        chip8.cycle();
        println!("{}", chip8.dump_display());
        thread::sleep(time::Duration::from_millis(100));
    }
}

