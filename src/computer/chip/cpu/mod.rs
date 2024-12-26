use self::instructions::CpuInstructions;

use super::memory::Register;

mod computation;
pub mod instructions;

type PC = u16;

pub struct Cpu {
    pub d_register: Register,
    pub a_register: Register,
    pub pc: PC,
}

#[derive(PartialEq, Debug)]
pub struct CPUResponse {
    pub out_m: u16,
    pub write_m: bool,
    pub address_m: u16,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            d_register: 0,
            a_register: 0,
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
    }

    pub fn execute(&mut self, instruction: CpuInstructions, input_m: u16) -> CPUResponse {
        let mut res = CPUResponse {
            out_m: 0,
            write_m: false,
            address_m: 0,
            pc: 0,
        };
        match instruction {
            CpuInstructions::Ainstruction(register_a) => {
                self.a_register = register_a;
                self.pc += 1;
            }
            CpuInstructions::CInstruction { comp, dest, jump } => {
                let (alu_out, zr, ng) = comp.execute(self.d_register, self.a_register, input_m);
                res.out_m = alu_out;
                res.address_m = self.a_register;

                if dest.a {
                    self.a_register = alu_out;
                }
                if dest.d {
                    self.d_register = alu_out;
                }
                if dest.m {
                    res.write_m = true;
                }

                jump.execute(self.a_register, &mut self.pc, zr, ng);
            }
        };
        res.pc = self.pc;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu() {
        struct Test {
            instructions: (CpuInstructions, bool, u16),
            expected: (CPUResponse, u16),
        }
        let mut cpu = Cpu::new();
        let tests = vec![
            Test {
                instructions: (CpuInstructions::from(0b0011000000111001), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 1,
                    },
                    0,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110110000010000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 12345,
                        write_m: false,
                        address_m: 12345,
                        pc: 2,
                    },
                    12345,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0101101110100000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 3,
                    },
                    12345,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110000111010000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 11111,
                        write_m: false,
                        address_m: 23456,
                        pc: 4,
                    },
                    11111,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0000001111101000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 5,
                    },
                    11111,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110001100001000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 11111,
                        write_m: true,
                        address_m: 1000,
                        pc: 6,
                    },
                    11111,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0000001111101001), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 7,
                    },
                    11111,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110001110011000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 11110,
                        write_m: true,
                        address_m: 1001,
                        pc: 8,
                    },
                    11110,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110001110001000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 11109,
                        write_m: true,
                        address_m: 1001,
                        pc: 9,
                    },
                    11110,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0000001111101000), false, 0),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 10,
                    },
                    11110,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1111010011010000), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 65535,
                        write_m: false,
                        address_m: 1000,
                        pc: 11,
                    },
                    65535,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0000000000001110), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 12,
                    },
                    65535,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110001100000100), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 65535,
                        write_m: false,
                        address_m: 14,
                        pc: 14,
                    },
                    65535,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b0000001111100111), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 0,
                        write_m: false,
                        address_m: 0,
                        pc: 15,
                    },
                    65535,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110110111100000), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 1000,
                        write_m: false,
                        address_m: 999,
                        pc: 16,
                    },
                    65535,
                ),
            },
            Test {
                instructions: (CpuInstructions::from(0b1110001100001000), false, 11111),
                expected: (
                    CPUResponse {
                        out_m: 65535,
                        write_m: true,
                        address_m: 1000,
                        pc: 17,
                    },
                    65535,
                ),
            },
        ];

        for Test {
            instructions: (instructions, _reset, input_m),
            expected: (cpu_response, d_register),
        } in tests
        {
            println!("{instructions:?}");
            let test_res = cpu.execute(instructions, input_m);

            assert_eq!(cpu_response, test_res);
            assert_eq!(cpu.d_register, d_register);
        }
    }
}
