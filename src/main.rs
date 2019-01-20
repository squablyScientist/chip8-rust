mod cpu;
fn main() {
    let mut chip8 = cpu::CHIP8::init();
    chip8.load_program(String::from("roms/maze.ch8"));
    println!("{}", chip8.dump_mem());
}

