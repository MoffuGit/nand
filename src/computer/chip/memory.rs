#[derive(Debug)]
pub enum MemoryError {
    OutOfBound(String),
}

pub struct Ram {
    ram16k: [u16; 16384],
    screen: [u16; 8192],
    keyboard: u16,
}

pub struct Rom {
    rom: [u16; 16384],
}

impl Rom {
    pub fn new() -> Self {
        Rom { rom: [0; 16384] }
    }
    pub fn load(&mut self, input: u16, address: u16) -> Result<u16, MemoryError> {
        let address = address as usize;
        if address <= 16384 {
            self.rom[address] = input;
            return Ok(self.rom[address]);
        }

        Err(MemoryError::OutOfBound("ROM Error".to_string()))
    }

    pub fn read(&self, address: u16) -> Result<u16, MemoryError> {
        if address <= 16384 {
            return Ok(self.rom[address as usize]);
        }

        Err(MemoryError::OutOfBound("ROM Error".to_string()))
    }
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            ram16k: [0; 16384],
            screen: [0; 8192],
            keyboard: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ram16k.fill(0);
        self.screen.fill(0);
        self.keyboard = 0;
    }

    pub fn load(&mut self, input: u16, address: u16) -> Result<u16, MemoryError> {
        let address = address as usize;
        if address < 16384 {
            self.ram16k[address] = input;
            return Ok(self.ram16k[address]);
        }

        if address >= 16384 && address < 8192 + 16384 {
            self.screen[address - 16384] = input;
            return Ok(self.screen[address - 16384]);
        }

        if address == 8192 + 16384 {
            self.keyboard = input;
            return Ok(self.keyboard);
        }

        Err(MemoryError::OutOfBound("RAM Error on load".to_string()))
    }

    pub fn read(&self, address: u16) -> Result<u16, MemoryError> {
        let address = address as usize;
        if address < 16384 {
            return Ok(self.ram16k[address]);
        }

        if address >= 16384 && address < 8192 + 16384 {
            return Ok(self.screen[address - 16384]);
        }

        if address == 8192 + 16384 {
            return Ok(self.keyboard);
        }

        Err(MemoryError::OutOfBound("RAM Error on read".to_string()))
    }
}
