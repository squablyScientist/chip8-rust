extern crate rand;
use std::fs::File;
//use self::rand::random;
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
    pub  pc: u16,       

    // Stack pointer
    pub  sp: u8,        
    
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
            pc: 0x200,
            sp: 0,
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
        let pc = self.pc;

        let mut opcode: u16  = ((self.mem[pc as usize]) as u16) << 8;
        opcode |= self.mem[(pc as usize) + 1] as u16;

        // Decode & Execute 
        match opcode & 0xF000 {

            
            // 0 instructions
            0x0000 => {
                match opcode & 0x00FF {

                    // CLS: Clears the screen
                    0x00E0 => {
                        //TODO implement for clearing
                    },

                    // RET: returns from a subroutine
                    0x00EE => {
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }

                    _ => panic!(),
                }
            },

            // JP addr: Sets the pc to the lower 3 nibbles of the instruction.
            0x1000 => self.pc = opcode & 0xFF,

            // CALL addr: puts return addr on the stack and jumps.
            0x2000 => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = opcode & 0xFFF;
            },

            // SE Vx, byte: Skips next instruction if Vx == byte
            0x3000 => {
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let byte : u8 = (opcode & 0x00FF) as u8;
                if self.V[x] == byte {
                    self.pc += 2;
                }
                self.pc += 2;
            },

            // SNE Vx, byte: Skips next instruction if Vx != byte
            0x4000 => {
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let byte : u8 = (opcode & 0x00FF) as u8;
                if self.V[x] != byte {
                    
                    // Skips the next instruction
                    self.pc += 2;
                }
                self.pc += 2;
            },
            
            // SE Vx Vy : Skips next instruction if Vx == Vy
            0x5000 => {
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let y : usize = ((opcode & 0x00F0) >> 4) as usize;

                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            },

            // LD Vx, byte: sets the value in Vx to byte
            0x6000 => {
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let byte : u8 = (opcode & 0x00FF) as u8;
                self.V[x] = byte;
                self.pc += 2;
            },

            // ADD Vx, byte : Vx += byte
            0x7000 => {
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let byte : u8 = (opcode & 0x00FF) as u8;
                self.V[x] += byte;
                self.pc += 2;
            },

            // Various Arithmetic and Bitwise Operations
            0x8000 => {

                // These arguments are constant for all 0x8xy? instructions
                let x : usize = ((opcode & 0x0F00) >> 8) as usize;
                let y : usize = ((opcode & 0x00F0) >> 4) as usize;

                match opcode & 0x000F {

                    // LD Vx, Vy : Sets Vx = Vy
                    0x0000 => self.V[x] = self.V[y],

                    // OR Vx, Vy : Vx |= Vy
                    0x0001 => self.V[x] |= self.V[y],

                    // AND Vx, Vy : Vx &= Vy
                    0x0002 => self.V[x] &= self.V[y],

                    // XOR Vx, Vy : Vx ^= Vy
                    0x0003 => self.V[x] ^= self.V[y],

                    // ADD Vx, Vy : Vx += Vy. Sets VF to 1 if overflow occurs, to 0 o/wise
                    0x0004 => {
                        let sum : u16 = self.V[x] as u16 + self.V[y] as u16;
                        if sum > 255 {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[x] = (sum & 0x00FF) as u8;
                    }

                    // SUB Vx, Vy : Vx -= Vy. If Vx > Vy then VF is set to 1, 0 o/wise
                    0x0005 => {
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        }
                        else{
                            self.V[0xF] = 0;
                        }

                        self.V[x] -= self.V[y];
                    },

                    // SHR Vx (, Vy) : Shifts Vx right by 1, setting VF to 1 if the lsb was 1
                    // before shift
                    0x0006 => {

                        // If the least significant bit is 1
                        if self.V[x] & 0x0001 == 1 {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }

                        self.V[x] = self.V[x] >> 1;
                    },

                    // SUBN Vx, Vy : Vx = Vy - Vx, sets VF = NOT BORROW
                    0x0007 => {
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }

                        self.V[x] = self.V[y] - self.V[x];
                    },

                    // SHR Vx (,Vy) : Shifts Vx right and sets VF 1 if msb was 1 before shift
                    0x000E => {

                        // If the most significant bit is 1 
                        if self.V[x] & 0x80 != 0 {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[x] = self.V[x] << 1;
                    },

                    _ => panic!(),
                };

                self.pc += 2;
            },
            
            // SNE Vx, Vy: Skips the next instruction if Vx != Vy
            0x9000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let y: usize = ((opcode & 0x00F0) >> 4) as usize;
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                }
                self.pc += 2;
            },

            // LD I, addr: Loads a 12-bit address into I
            0xA000 => {
                self.I = opcode & 0x0FFF;
                self.pc += 2;
            }

            // JP V0, addr: Sets the pc to V0 + addr
            0xB000 => self.pc = (opcode & 0x0FFF) + (self.V[0] as u16),

            // RND Vx, byte: Vx = random byte AND byre
            0xC000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;
                let byte: u8 = (opcode & 0x00FF) as u8;
                let r = rand::random::<u8>();
                self.V[x] = byte & r;
                self.pc += 2;
            },

            // TODO: implement displaying sprite to a screen 
            0xD000 => {},

            // Keypad skip instructions
            0xE000 => {
                let val_of_x: usize = self.V[((opcode & 0x0F00) >> 8) as usize] as usize; 

                match opcode & 0x00FF {

                    // SKP Vx : Skips the next instruction iff they key with the value in Vx is
                    // pressed.
                    0x009E => {
                        if self.keypad[val_of_x] == true {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }

                    // SKNP Vx : Skips the next instruction iff the keypad with the value in Vx is
                    // not pressed.
                    0x00A1 => {
                        if self.keypad[val_of_x] == false {
                            self.pc += 2;
                        }
                        self.pc += 2;
                    }

                    _ => panic!(),
                }

            },


            0xF000 => {
                let x: usize = ((opcode & 0x0F00) >> 8) as usize;

                match opcode & 0x00FF {
                    
                    // LD Vx, DT: Puts the value of the delay timer into Vx
                    0x0007 => self.V[x] = self.delay,

                    // LD Vx, K: Waits for a key press and puts the value of key in Vx
                    // TODO: implement
                    0x000A => {},

                    // LD DT, Vx: Loads the delay timer with the value in Vx
                    0x0015 => self.delay = self.V[x],

                    // LD ST, Vx: Loads the sound timer with the value in Vx
                    0x0018 => self.sound = self.V[x],

                    // ADD I, Vx: I += Vx
                    0x001E => self.I += self.V[x] as u16,

                    // LD I, Vx: Sets I to the location of the sprite for digit Vx
                    0x0029 => {
                        self.I = (0x50 + self.V[x]*5) as u16;
                    },

                    // LD B, Vx: Stores the decimal representation of Vx at: 100s:I, 10s:I+1,
                    // 1s:I+2.
                    0x0033 => {
                        self.mem[(self.I) as usize] = self.V[x] / 100;
                        self.mem[(self.I + 1) as usize] = (self.V[x] % 100) / 10;
                        self.mem[(self.I + 2) as usize] = self.V[x] % 10;
                    },

                    // LD [I], Vx : Stores registers V0 thru Vx in memory starting at addr I
                    0x0055 => {
                        for i in 0..(x + 1) {
                            self.mem[(self.I + (i as u16)) as usize] = self.V[i];
                        }
                    },

                    // LD Vx, [I] : Reads registers V0 thru Vx from memory starting at addr I
                    0x0065 => {
                        for i in 0..(x + 1) {
                            self.V[i] = self.mem[(self.I + (i as u16)) as usize];
                        }
                    }

                    _ => panic!(),
                }
                self.pc += 2;
            },

            _ => panic!(),
        }
        
        // TODO update timers
    }
}
