use rand::Rng;

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;
const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;
const START_ADDRESS: u16 = 0x200;
const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct Emu {
    ram: [u8; RAM_SIZE],
    v: [u8; NUM_REGISTERS],
    pc: u16,
    stack: [u16; STACK_SIZE],
    screen: [bool; SCREEN_HEIGHT * SCREEN_WIDTH],
    keys: [bool; NUM_KEYS],
    sp: u8,
    i: u16,
    dt: u8,
    st: u8,
    running: bool,
}

// Basic Stuff
impl Emu {
    fn new() -> Self {
        let mut new_emu = Self {
            ram: [0; RAM_SIZE],
            v: [0; NUM_REGISTERS],
            pc: START_ADDRESS,
            stack: [0; STACK_SIZE],
            screen: [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            keys: [false; NUM_KEYS],
            sp: 0,
            i: 0,
            dt: 0,
            st: 0,
            running: true,
        };
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
        new_emu
    }

    fn reset(&mut self) {
        self.ram = [0; RAM_SIZE];
        self.v = [0; NUM_REGISTERS];
        self.pc = START_ADDRESS;
        self.stack = [0; STACK_SIZE];
        self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH];
        self.keys = [false; NUM_KEYS];
        self.sp = 0;
        self.i = 0;
        self.dt = 0;
        self.st = 0;
        self.running = true;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        higher_byte << 8 | lower_byte
    }
}

// Emulation
impl Emu {
    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn execute(&mut self, opcode: u16) {
        let d1 = (opcode & 0xF000) >> 12;
        let d2 = (opcode & 0x0F00) >> 8;
        let d3 = (opcode & 0x00F0) >> 4;
        let d4 = opcode & 0x000F;
        match (d1, d2, d3, d4) {
            // NOP
            (0x0, 0x0, 0x0, 0x0) => return,
            // Clear Screen
            (0x0, 0x0, 0xE, 0x0) => self.screen = [false; SCREEN_HEIGHT * SCREEN_WIDTH],
            // Return
            (0x0, 0x0, 0xE, 0xE) => self.pc = self.pop(),
            // Jmp NNN
            (0x1, _, _, _) => self.pc = opcode & 0x0FFF,
            // Call NNN
            (0x2, _, _, _) => {
                self.push(self.pc);
                self.pc = opcode & 0x0FFF;
            }
            // Skip if Vx == NN
            (0x3, _, _, _) => {
                let val = (opcode & 0x00FF) as u8;
                let ind = d2;
                if self.v[ind as usize] == val as u8 {
                    self.pc += 2;
                }
            }
            // Skip if Vx != NN
            (0x4, _, _, _) => {
                let val = (opcode & 0x00FF) as u8;
                let ind = d2;
                if self.v[ind as usize] != val as u8 {
                    self.pc += 2;
                }
            }
            // Skip if Vx == Vy
            (0x5, _, _, 0x0) => {
                let ind1 = d2;
                let ind2 = d3;
                if self.v[ind1 as usize] == self.v[ind2 as usize] {
                    self.pc += 2;
                }
            }
            // Set Vx = NN
            (0x6, _, _, _) => {
                let ind = d2;
                let val = (opcode & 0x00FF) as u8;
                self.v[ind as usize] = val;
            }
            // Set Vx = Vx + NN
            (0x7, _, _, _) => {
                let ind = d2;
                let val = (opcode & 0x00FF) as u8;
                self.v[ind as usize] = self.v[ind as usize].wrapping_add(val);
            }
            (0x8, _, _, _) => {
                let x = d2 as u8;
                let y = d3 as u8;
                let ld = d4 as u8;
                match ld {
                    // Set Vx = Vy
                    0x0 => self.v[x as usize] = self.v[y as usize],
                    // Set Vx = Vx | Vy
                    0x1 => self.v[x as usize] |= self.v[y as usize],
                    // Set Vx = Vx & Vy
                    0x2 => self.v[x as usize] &= self.v[y as usize],
                    // Set Vx = Vx ^ Vy
                    0x3 => self.v[x as usize] ^= self.v[y as usize],
                    // Set Vx = Vx + Vy
                    0x4 => {
                        let (new_vx, carry) =
                            self.v[x as usize].overflowing_add(self.v[y as usize]);
                        self.v[x as usize] = new_vx;
                        let new_vf = if carry { 1 } else { 0 };
                        self.v[0xF] = new_vf;
                    }
                    // Set Vx = Vx - Vy
                    0x5 => {
                        let (new_vx, carry) =
                            self.v[x as usize].overflowing_sub(self.v[y as usize]);
                        self.v[x as usize] = new_vx;
                        let new_vf = if carry { 1 } else { 0 };
                        self.v[0xF] = new_vf;
                    }
                    // Set Vx = Vx >> 1
                    0x6 => {
                        let new_vf = self.v[x as usize] & 0x1;
                        self.v[x as usize] >>= 1;
                        self.v[0xF] = new_vf;
                    }
                    // Set Vx = Vy - Vx
                    0x7 => {
                        let (new_vx, carry) =
                            self.v[y as usize].overflowing_sub(self.v[x as usize]);
                        self.v[x as usize] = new_vx;
                        let new_vf = if carry { 1 } else { 0 };
                        self.v[0xF] = new_vf;
                    }
                    // Set Vx = Vx << 1
                    0xE => {
                        let new_vf = self.v[x as usize] >> 7;
                        self.v[x as usize] <<= 1;
                        self.v[0xF] = new_vf;
                    }
                    _ => println!("Unknown opcode: {:#04x}", opcode),
                }
            }
            // Skip if Vx != Vy
            (0x9, _, _, 0x0) => {
                let x = d2 as u8;
                let y = d3 as u8;
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            // Set I = NNN
            (0xA, _, _, _) => self.i = opcode & 0x0FFF,
            // Jmp V0 + NNN
            (0xB, _, _, _) => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            // Set Vx = random() & NN
            (0xC, _, _, _) => {
                let x = d2 as u8;
                let nn = (opcode & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.v[x as usize] = rng & nn;
            }
            (0xD, _, _, _) => {
                // Get the (x, y) coords for our sprite
                let x_coord = self.v[d2 as usize] as u16;
                let y_coord = self.v[d3 as usize] as u16;
                // The last digit determines how many rows high our sprite is
                let num_rows = d4;

                // Keep track if any pixels were flipped
                let mut flipped = false;
                // Iterate over each row of our sprite
                for y_line in 0..num_rows {
                    // Determine which memory address our row's data is stored
                    let addr = self.i + y_line as u16;
                    let pixels = self.ram[addr as usize];
                    // Iterate over each column in our row
                    for x_line in 0..8 {
                        // Use a mask to fetch current pixel's bit. Only flip if a 1
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            // Sprites should wrap around screen, so apply modulo
                            let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                            let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;

                            // Get our pixel's index in the 1D screen array
                            let idx = x + SCREEN_WIDTH * y;
                            // Check if we're about to flip the pixel and set
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }
                // Populate VF register
                if flipped {
                    self.v[0xF] = 1;
                } else {
                    self.v[0xF] = 0;
                }
            }
            (0xE, _, 0x9, 0xE) => {
                let x = d2 as u8;
                if self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            (0xE, _, 0xA, 0x1) => {
                let x = d2 as u8;
                if !self.keys[self.v[x as usize] as usize] {
                    self.pc += 2;
                }
            }
            (0xF, _, 0x0, 0x7) => {
                let x = d2 as u8;
                self.v[x as usize] = self.dt;
            }
            (0xF, _, 0x0, 0xA) => {
                let x = d2 as u8;
                let mut key_pressed = false;
                for (i, key) in self.keys.iter().enumerate() {
                    if *key {
                        self.v[x as usize] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed {
                    self.pc -= 2;
                }
            }
            (0xF, _, 0x1, 0x5) => {
                let x = d2 as u8;
                self.dt = self.v[x as usize];
            }
            (0xF, _, 0x1, 0x8) => {
                let x = d2 as u8;
                self.st = self.v[x as usize];
            }
            (0xF, _, 0x1, 0xE) => {
                let x = d2 as u8;
                self.i += self.v[x as usize] as u16;
            }
            _ => println!("Unknown opcode: {:#04x}", opcode),
        }
    }
}
