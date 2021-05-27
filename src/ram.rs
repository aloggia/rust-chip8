pub struct Ram {
    // The chip 8 architecture has ram composed of 4096 8 but addresses
    mem: [u8; 4096]
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram {mem: [0; 4096]};

        // nested array of hex values that creates sprites for the hex following hex values
        // These sprites get loaded into memory starting at position 0
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], //sprite for 0
            [0x20, 0x60, 0x20, 0x20, 0x70], //sprite for 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], //sprite for 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], //sprite for 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], //sprite for 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], //sprite for 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], //sprite for 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], //sprite for 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], //sprite for 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], //sprite for 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], //sprite for A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], //sprite for B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], //sprite for C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], //sprite for D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], //sprite for E
            [0xF0, 0x80, 0xF0, 0x80, 0x80] //sprite for F
        ];

            // i is an interating position in memory used to load the sprites into mem starting at
            // position 0, that is what the nested for loop does
            let mut i = 0;
            for sprite in sprites.iter() {
                for ch in sprite {
                    ram.mem[i] = *ch;
                    i += 1;
                }
            }

        print!("RAM: {:?}", ram.mem);
        ram
    }
    // pretty much just getters & setters for memory
    // write_byte takes in an address and a value, and sets the given address to the given value
    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }
    // read_byte takes in an address and a value, and sets value to the value currently in address
    pub fn read_byte(&mut self, address: u16) -> u8 {
        let mut value = self.mem[address as usize];
        return value;
    }

}
