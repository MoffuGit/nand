use super::computation::Computation;
use super::PC;

#[derive(PartialEq, Debug)]
pub enum CpuInstructions {
    Ainstruction(u16),
    CInstruction {
        comp: Computation,
        dest: Destination,
        jump: Jump,
    },
}

impl From<u16> for CpuInstructions {
    fn from(value: u16) -> Self {
        if value >> 13 & 0b111 == 0b111 {
            return CpuInstructions::CInstruction {
                comp: Computation::from(value >> 6 & 0b1111111),
                dest: Destination::from(value >> 3 & 0b111),
                jump: Jump::from(value & 0b111),
            };
        }
        CpuInstructions::Ainstruction(value & 0x7FFF)
    }
}

impl From<CpuInstructions> for u16 {
    fn from(value: CpuInstructions) -> Self {
        match value {
            CpuInstructions::Ainstruction(value) => value & 0x7FFF,
            CpuInstructions::CInstruction { comp, dest, jump } => {
                let mut binary: u16 = 0b1110_0000_0000_0000;
                binary |= (comp as u16) << 6;
                binary |= u16::from(dest) << 3;
                binary |= jump as u16;
                binary
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Jump {
    Null = 0b000,
    Jgt = 0b001,
    Jeq = 0b010,
    Jge = 0b011,
    Jlt = 0b100,
    Jne = 0b101,
    Jle = 0b110,
    Jmp = 0b111,
}

impl From<u16> for Jump {
    fn from(value: u16) -> Self {
        match value & 0b111 {
            0b000 => Jump::Null,
            0b001 => Jump::Jgt,
            0b010 => Jump::Jeq,
            0b011 => Jump::Jge,
            0b100 => Jump::Jlt,
            0b101 => Jump::Jne,
            0b110 => Jump::Jle,
            0b111 => Jump::Jmp,
            _ => unreachable!(),
        }
    }
}

impl Jump {
    pub fn execute(&self, a_value: u16, pc: &mut PC, zr: bool, ng: bool) {
        match self {
            Jump::Null => {
                *pc += 1;
            }
            Jump::Jgt => {
                if !zr && !ng {
                    *pc = a_value
                } else {
                    *pc += 1;
                }
            }
            Jump::Jeq => {
                if zr {
                    *pc = a_value
                } else {
                    *pc += 1;
                }
            }
            Jump::Jge => {
                if !ng {
                    *pc = a_value
                } else {
                    *pc += 1;
                }
            }
            Jump::Jlt => {
                if ng {
                    *pc = a_value
                } else {
                    println!("incorrect");
                    *pc += 1;
                }
            }
            Jump::Jne => {
                if !zr {
                    *pc = a_value
                } else {
                    *pc += 1;
                }
            }
            Jump::Jle => {
                if zr || ng {
                    *pc = a_value
                } else {
                    *pc += 1;
                }
            }
            Jump::Jmp => *pc = a_value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Destination {
    pub a: bool,
    pub m: bool,
    pub d: bool,
}

impl From<Destination> for u16 {
    fn from(value: Destination) -> Self {
        let mut binary = 0b000;
        binary |= value.m as u16;
        binary |= (value.d as u16) << 1;
        binary |= (value.a as u16) << 2;
        binary
    }
}

impl From<u16> for Destination {
    fn from(value: u16) -> Self {
        Destination {
            a: value >> 2 & 0b1 == 1,
            d: value >> 1 & 0b1 == 1,
            m: value & 0b1 == 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cpu_instruction_to_u16() {
        struct Test {
            instruction: CpuInstructions,
            expected: u16,
        }
        let tests = vec![
            Test {
                instruction: CpuInstructions::Ainstruction(0),
                expected: 0,
            },
            Test {
                instruction: CpuInstructions::Ainstruction(32767),
                expected: 32767,
            },
            Test {
                instruction: CpuInstructions::Ainstruction(32768),
                expected: 0,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::Zero,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                expected: 0b1110101010000000,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                expected: 0b1110111111000000,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::MMinusOne,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                expected: 0b1111110010000000,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::AMinusOne,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                expected: 0b1110110010000000,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b100),
                    jump: Jump::Null,
                },
                expected: 0b1110111111100000,
            },
            Test {
                instruction: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b100),
                    jump: Jump::Jmp,
                },
                expected: 0b1110111111100111,
            },
        ];

        for Test {
            instruction,
            expected,
        } in tests
        {
            assert_eq!(u16::from(instruction), expected)
        }
    }

    #[test]
    fn test_from_u16_cpu_instruction() {
        struct Test {
            instruction: u16,
            expected: CpuInstructions,
        }
        let tests = vec![
            Test {
                instruction: 0,
                expected: CpuInstructions::Ainstruction(0),
            },
            Test {
                instruction: 32767,
                expected: CpuInstructions::Ainstruction(32767),
            },
            Test {
                expected: CpuInstructions::Ainstruction(0),
                instruction: 32768,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::Zero,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                instruction: 0b1110101010000000,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                instruction: 0b1110111111000000,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::MMinusOne,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                instruction: 0b1111110010000000,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::AMinusOne,
                    dest: Destination::from(0b000),
                    jump: Jump::Null,
                },
                instruction: 0b1110110010000000,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b100),
                    jump: Jump::Null,
                },
                instruction: 0b1110111111100000,
            },
            Test {
                expected: CpuInstructions::CInstruction {
                    comp: Computation::One,
                    dest: Destination::from(0b100),
                    jump: Jump::Jmp,
                },
                instruction: 0b1110111111100111,
            },
        ];

        for Test {
            instruction,
            expected,
        } in tests
        {
            assert_eq!(CpuInstructions::from(instruction), expected)
        }
    }
}
