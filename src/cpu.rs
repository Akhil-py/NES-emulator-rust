// Placeholder for the Central Processing Unit (CPU) implementation

// Define the CPU struct and its methods here
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xFFFF]
        }
    }

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }
 
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
  
    pub fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
   
    /// Update the zero and negative flags based on the result of an operation.
    /// The zero flag is set if the result is zero, and the negative flag is set if the result is negative.
    pub fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
             self.status = self.status | 0b0000_0010;
        } else {
             self.status = self.status & 0b1111_1101;
        }
 
         if result & 0b1000_0000 != 0 {
             self.status = self.status | 0b1000_0000;
        } else {
             self.status = self.status & 0b0111_1111;
        }
    }

    // Fetch-Decode-Execute Cycle
    pub fn run(&mut self) {
        //self.program_counter = 0;
    
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;
    
            match opscode {
                0xA9 => { // LDA Immediate
                    let param = self.mem_read(self.program_counter);
                    self.program_counter += 1;
                    
                    self.lda(param);
                }
    
                0xAA => self.tax(), // TAX (Transfer A to X)
    
                0x00 => return, // BRK (Force Interrupt)
                
                0xE8 => { // INX (Increment X)
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_zero_and_negative_flags(self.register_x);
                }
                _ => todo!(),
            }
        }
    }
    
}