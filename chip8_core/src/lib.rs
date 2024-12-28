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
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
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
    running: bool
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
            running: true
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


}