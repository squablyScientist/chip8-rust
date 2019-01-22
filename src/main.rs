use std::{thread, time, env};
mod cpu;
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut chip8 = cpu::CHIP8::init();
    chip8.load_program(filename);
    loop {
        //println!("{}", chip8.dump_reg());
        chip8.cycle();
        println!("{}", chip8.dump_display());
        thread::sleep(time::Duration::from_millis(2));
    }
}

