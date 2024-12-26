use self::chip::cpu::instructions::CpuInstructions;
use self::chip::cpu::{CPUResponse, Cpu};
use self::chip::memory::{MemoryError, Ram, Rom};

mod chip;
mod gate;

struct Computer {
    rom: Rom,
    ram: Ram,
    cpu: Cpu,
    prev_cpu_response: CPUResponse,
}

#[derive(Debug)]
pub enum Error {
    Memory(MemoryError),
}

impl From<MemoryError> for Error {
    fn from(value: MemoryError) -> Self {
        Error::Memory(value)
    }
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            prev_cpu_response: CPUResponse {
                out_m: 0,
                write_m: false,
                address_m: 0,
                pc: 0,
            },
            ram: Ram::new(),
            rom: Rom::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn reset(&mut self) {
        self.prev_cpu_response = CPUResponse {
            out_m: 0,
            write_m: false,
            address_m: 0,
            pc: 0,
        };
        self.ram.reset();
        self.cpu.reset();
    }

    pub fn execute(&mut self) -> Result<(), Error> {
        let instruction = self.rom.read(self.prev_cpu_response.pc)?;

        let input_m = self.ram.read(self.prev_cpu_response.address_m)?;

        let cpu_response = self
            .cpu
            .execute(CpuInstructions::from(instruction), input_m);

        if cpu_response.write_m {
            self.ram.load(cpu_response.out_m, cpu_response.address_m)?;
        }

        self.prev_cpu_response = cpu_response;

        Ok(())
    }

    pub fn load_program(&mut self, program: Vec<u16>) -> Result<(), Error> {
        for (idx, instuction) in program.iter().enumerate() {
            self.rom
                .load(*instuction, idx as u16 + self.prev_cpu_response.pc)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Computer, Error};

    #[test]
    fn test_add_program() -> Result<(), Error> {
        #[derive(Debug, Clone)]
        struct State {
            a_register: u16,
            d_register: u16,
            pc: u16,
            ram_0: u16,
        }
        let add_program: Vec<u16> = vec![
            0b0000000000000010,
            0b1110110000010000,
            0b0000000000000011,
            0b1110000010010000,
            0b0000000000000000,
            0b1110001100001000,
        ];

        let expected_state = vec![
            State {
                a_register: 2,
                d_register: 0,
                pc: 1,
                ram_0: 0,
            },
            State {
                a_register: 2,
                d_register: 2,
                pc: 2,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 2,
                pc: 3,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 5,
                pc: 4,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 5,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 6,
                ram_0: 5,
            },
        ];

        let mut computer = Computer::new();

        computer.load_program(add_program)?;

        for State {
            a_register,
            d_register,
            pc,
            ram_0,
        } in expected_state
        {
            computer.execute()?;
            assert_eq!(computer.cpu.a_register, a_register);
            assert_eq!(computer.cpu.d_register, d_register);
            assert_eq!(computer.prev_cpu_response.pc, pc);
            assert_eq!(computer.ram.read(0b0)?, ram_0);
        }

        computer.reset();

        let expected_state = vec![
            State {
                a_register: 2,
                d_register: 5,
                pc: 1,
                ram_0: 0,
            },
            State {
                a_register: 2,
                d_register: 2,
                pc: 2,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 2,
                pc: 3,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 5,
                pc: 4,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 5,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 6,
                ram_0: 5,
            },
        ];

        for State {
            a_register,
            d_register,
            pc,
            ram_0,
        } in expected_state
        {
            computer.execute()?;
            assert_eq!(computer.cpu.a_register, a_register);
            assert_eq!(computer.cpu.d_register, d_register);
            assert_eq!(computer.prev_cpu_response.pc, pc);
            assert_eq!(computer.ram.read(0b0)?, ram_0);
        }
        Ok(())
    }

    #[test]
    fn test_max_program() -> Result<(), Error> {
        #[derive(Debug, Clone)]
        struct State {
            a_register: u16,
            d_register: u16,
            pc: u16,
            ram_0: u16,
        }
        let max_program: Vec<u16> = vec![
            0b0000000000000000,
            0b1111110000010000,
            0b0000000000000001,
            0b1111010011010000,
            0b0000000000001010,
            0b1110001100000001,
            0b0000000000000001,
            0b1111110000010000,
            0b0000000000001100,
            0b1110101010000111,
            0b0000000000000000,
            0b1111110000010000,
            0b0000000000000010,
            0b1110001100001000,
            0b0000000000001110,
            0b1110101010000111,
        ];

        let expected_state = vec![];

        let mut computer = Computer::new();

        computer.load_program(max_program)?;

        for State {
            a_register,
            d_register,
            pc,
            ram_0,
        } in expected_state
        {
            computer.execute()?;
            assert_eq!(computer.cpu.a_register, a_register);
            assert_eq!(computer.cpu.d_register, d_register);
            assert_eq!(computer.prev_cpu_response.pc, pc);
            assert_eq!(computer.ram.read(0b0)?, ram_0);
        }

        computer.reset();

        let expected_state = vec![
            State {
                a_register: 2,
                d_register: 5,
                pc: 1,
                ram_0: 0,
            },
            State {
                a_register: 2,
                d_register: 2,
                pc: 2,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 2,
                pc: 3,
                ram_0: 0,
            },
            State {
                a_register: 3,
                d_register: 5,
                pc: 4,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 5,
                ram_0: 0,
            },
            State {
                a_register: 0,
                d_register: 5,
                pc: 6,
                ram_0: 5,
            },
        ];

        for State {
            a_register,
            d_register,
            pc,
            ram_0,
        } in expected_state
        {
            computer.execute()?;
            assert_eq!(computer.cpu.a_register, a_register);
            assert_eq!(computer.cpu.d_register, d_register);
            assert_eq!(computer.prev_cpu_response.pc, pc);
            assert_eq!(computer.ram.read(0b0)?, ram_0);
        }
        Ok(())
    }
}
