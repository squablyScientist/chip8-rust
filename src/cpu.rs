use std::fs::File;
use std::io::Read;

pub const FONT: [u8; 80] = [
    
    0xF0,   // 0
    0x90,   // 0
    0x90,   // 0
    0x90,   // 0
    0xF0,   // 0

    0x20,	// 1
    0x60,	// 1
    0x20,	// 1
    0x20,	// 1
    0x70,	// 1

    0xF0,	// 2
    0x10,	// 2
    0xF0,	// 2
    0x80,	// 2
    0xF0,	// 2

    0xF0,	// 3
    0x10,	// 3
    0xF0,	// 3
    0x10,	// 3
    0xF0,	// 3

    0x90,	// 4
    0x90,	// 4
    0xF0,	// 4
    0x10,	// 4
    0x10,	// 4

    0xF0,	// 5
    0x80,	// 5
    0xF0,	// 5
    0x10,	// 5
    0xF0,	// 5

    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,

	0xF0,
	0x10,
	0x20,
	0x40,
	0x40,

	0xF0,
	0x90,
	0xF0,
	0x90,
	0xF0,

	0xF0,
	0x90,
	0xF0,
	0x10,
	0xF0,

	0xF0,
	0x90,
	0xF0,
	0x90,
	0x90,

	0xE0,
	0x90,
	0xE0,
	0x90,
	0xE0,

	0xF0,
	0x80,
	0x80,
	0x80,
	0xF0,

	0xE0,
	0x90,
	0x90,
	0x90,
	0xE0,

	0xF0,
	0x80,
	0xF0,
	0x80,
	0xF0,

	0xF0,
	0x80,
	0xF0,
	0x80,
	0x80,
    ];
pub struct CHIP8 {
    // General purpose registers except for V[0xF]
    pub  V: [u8; 16],   

    // 16 bit register to store addresses.
    pub  I: u16,        

    // This register counts towards 0 at a rate of 60Hz
    pub  delay: u8,     

    // This register counts towards 0 at a rate of 60Hz. A noise is made at 0
    pub  sound: u8,     

    // Program counter
    pub  PC: u16,       

    // Stack pointer
    pub  SP: u8,        
    
    // The memory accessible by the CHIP-8 CPU
    pub mem: [u8; 4096],

    // The stack of return addrs
    pub stack: [u16; 16],

    // Monochrome 64 x 32 display 
    pub display: [[bool; 64]; 32],

    // 16 key keypad
    pub keypad: [bool; 16],
}

impl CHIP8 {
    pub fn init() -> CHIP8 {
        let mut mem = [0; 4096];

        // Loads the fontset into the interpreter region of memory
        for i in 0..80 {
            mem[i  + 0x50] = FONT[i];
        }

        CHIP8 {
            V: [0; 16],
            I: 0,
            delay: 0,
            sound: 0,
            PC: 0x200,
            SP: 0,
            mem: mem,
            stack: [0; 16],
            display: [[false; 64]; 32],
            keypad: [false; 16],
        }
    }

    pub fn load_program(&mut self, filename: String) {

        let mut file = File::open(filename).unwrap();
        let mut buf = Vec::new();

        // Reads the entire ROM file 
        let bytes_read: usize = file.read_to_end(&mut buf).unwrap();

        for i in 0..bytes_read{
            self.mem[i + 0x200] = buf[i];
        }
    }

    // Function to dump the entire memory of the chip8 in xxd format. 
    // TODO: allow for memory ranges and ASCII printing.
    pub fn dump_mem(&self) -> String {
        let mut mem_dump = String::new();
        for i in (0..4096).step_by(16) {
            mem_dump.push_str(&format!("{:03x}:   ", i));
            for j in 0..8 {
                mem_dump.push_str(&format!("{:02x}{:02x} ", 
                                          self.mem[i+j], self.mem[i+j+1]));
            }
            mem_dump.push_str("\n");
        }
        mem_dump
    }
    
    pub fn cycle(&mut self) {
        // Fetch
        let pc = self.PC;
        let mem = self.mem;

        let mut opcode: u16 = 0;

        opcode = ((self.mem[pc as usize]) as u16) << 8;
        opcode |= self.mem[(pc as usize) + 1] as u16;

        // Decode
        match opcode & 0xF000 {

            // JP addr: Sets the pc to the lower 3 nibbles of the instruction.
            0x1000 => self.PC = ((opcode << 4) >> 4),

            // CALL addr: puts return addr on the stack and jumps.
            0x2000 => {
                self.stack[self.SP as usize] = self.PC;
                self.SP = self.SP + 1;
                self.PC = ((opcode << 4) >> 4)
            }

            // TODO: implement rest of instructions


            _ => panic!(),
        }
        // Execute
        
        // update timers
    }
}
