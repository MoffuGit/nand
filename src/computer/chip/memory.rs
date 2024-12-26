pub type Ram16K = [u16; 8192];
pub type Ram8K = [u16; 4096];
pub type Register = u16;
pub type ROM32K = [u16; 16384];

#[derive(Debug)]
pub enum MemoryError {
    OutOfBound,
}

pub struct Ram {
    ram16k: Ram16K,
    screen: Ram8K,
    keyboard: Register,
}

pub struct Rom {
    rom: ROM32K,
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

        Err(MemoryError::OutOfBound)
    }

    pub fn read(&self, address: u16) -> Result<u16, MemoryError> {
        if address <= 16384 {
            return Ok(self.rom[address as usize]);
        }

        Err(MemoryError::OutOfBound)
    }
}

impl Ram {
    pub fn new() -> Self {
        Ram {
            ram16k: [0; 8192],
            screen: [0; 4096],
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
        if address <= 8192 {
            self.ram16k[address] = input;
            return Ok(self.ram16k[address]);
        }

        if address > 8192 && address <= 8192 + 4096 {
            self.screen[address - 8192] = input;
            return Ok(self.screen[address - 8192]);
        }

        if address == 8192 + 4096 + 1 {
            self.keyboard = input;
            return Ok(self.keyboard);
        }

        Err(MemoryError::OutOfBound)
    }

    pub fn read(&self, address: u16) -> Result<u16, MemoryError> {
        let address = address as usize;
        if address <= 8192 {
            return Ok(self.ram16k[address]);
        }

        if address > 8192 && address <= 8192 + 4096 {
            return Ok(self.screen[address - 8192]);
        }

        if address == 8192 + 4096 + 1 {
            return Ok(self.keyboard);
        }

        Err(MemoryError::OutOfBound)
    }
}
