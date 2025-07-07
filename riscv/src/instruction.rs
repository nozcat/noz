use std::fmt;

/// RISC-V instruction representation for 32-bit IM (Integer + Multiplication) extension.
///
/// This implementation specifically targets RV32IM, which includes:
/// - **RV32I**: Base integer instruction set (arithmetic, load/store, branch, jump)
/// - **RV32M**: Standard extension for integer multiplication and division
///
/// ## Architecture
/// - **32-bit RISC-V**: All operations are 32-bit width
/// - **Register set**: X0-X31 (32 general-purpose registers)
/// - **Memory**: 32-bit addressing space
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RiscVInstruction {
    /// Jump and Link Register instruction (RV32I base instruction set)
    ///
    /// Jumps to address `rs1 + imm` and saves return address in `rd`.
    /// If `rd = x0`, the return address is discarded (simple jump).
    Jalr { rd: u8, rs1: u8, imm: i16 },

    /// Unsupported instruction
    ///
    /// Contains the raw 32-bit instruction word for debugging purposes.
    Unsupported(u32),
}

impl fmt::Display for RiscVInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscVInstruction::Jalr { rd, rs1, imm } => {
                write!(f, "jalr x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Unsupported(opcode) => {
                write!(f, "unsupported(0x{:08x})", opcode)
            }
        }
    }
}

pub const JALR_OPCODE: u32 = 0x67;
pub const JALR_FUNCT3: u32 = 0x0;

pub const OPCODE_MASK: u32 = 0x7f;
pub const FUNCT3_MASK: u32 = 0x7000;
pub const RD_MASK: u32 = 0xf80;
pub const RS1_MASK: u32 = 0xf8000;
pub const IMM_I_MASK: u32 = 0xfff00000;

pub const FUNCT3_SHIFT: u32 = 12;
pub const RD_SHIFT: u32 = 7;
pub const RS1_SHIFT: u32 = 15;
pub const IMM_I_SHIFT: u32 = 20;

impl RiscVInstruction {
    /// Decode a 32-bit instruction word into a RiscVInstruction
    ///
    /// # Arguments
    ///
    /// * `word` - The 32-bit instruction word to decode
    pub fn decode(word: u32) -> RiscVInstruction {
        let opcode = word & OPCODE_MASK;

        match opcode {
            JALR_OPCODE => {
                let funct3 = (word & FUNCT3_MASK) >> FUNCT3_SHIFT;
                if funct3 == JALR_FUNCT3 {
                    let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                    let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                    let imm = ((word & IMM_I_MASK) as i32 >> IMM_I_SHIFT) as i16;

                    RiscVInstruction::Jalr { rd, rs1, imm }
                } else {
                    RiscVInstruction::Unsupported(word)
                }
            }
            _ => RiscVInstruction::Unsupported(word),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod decode {
        use super::*;

        mod jalr {
            use super::*;

            #[test]
            fn basic() {
                let jalr_x1_x2_4 = 0x004100e7;
                let decoded = RiscVInstruction::decode(jalr_x1_x2_4);

                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 1);
                        assert_eq!(rs1, 2);
                        assert_eq!(imm, 4);
                    }
                    _ => panic!("Expected JALR instruction"),
                }
            }

            #[test]
            fn negative_imm() {
                let jalr_x0_x1_neg4 = 0xffc08067;
                let decoded = RiscVInstruction::decode(jalr_x0_x1_neg4);

                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, -4);
                    }
                    _ => panic!("Expected JALR instruction"),
                }
            }

            #[test]
            fn unsupported_func3() {
                let jalr_with_invalid_funct3 = 0x004110e7;
                let decoded = RiscVInstruction::decode(jalr_with_invalid_funct3);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x004110e7);
                    }
                    _ => panic!("Expected unsupported instruction for JALR with invalid funct3"),
                }
            }

            #[test]
            fn rd_range() {
                // Test rd = 0 (x0)
                let jalr_x0 = 0x00008067;
                let decoded = RiscVInstruction::decode(jalr_x0);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, 0);
                    }
                    _ => panic!("Expected JALR instruction"),
                }

                // Test rd = 31 (x31)
                let jalr_x31 = 0x000080e7 | (31 << 7);
                let decoded = RiscVInstruction::decode(jalr_x31);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 31);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, 0);
                    }
                    _ => panic!("Expected JALR instruction"),
                }
            }

            #[test]
            fn rs1_range() {
                // Test rs1 = 0 (x0)
                let jalr_rs1_0 = 0x00000067;
                let decoded = RiscVInstruction::decode(jalr_rs1_0);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 0);
                        assert_eq!(imm, 0);
                    }
                    _ => panic!("Expected JALR instruction"),
                }

                // Test rs1 = 31 (x31)
                let jalr_rs1_31 = 0x000f8067;
                let decoded = RiscVInstruction::decode(jalr_rs1_31);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 31);
                        assert_eq!(imm, 0);
                    }
                    _ => panic!("Expected JALR instruction"),
                }
            }

            #[test]
            fn imm_range() {
                // Test imm = 0
                let jalr_imm_0 = 0x00008067;
                let decoded = RiscVInstruction::decode(jalr_imm_0);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, 0);
                    }
                    _ => panic!("Expected JALR instruction"),
                }

                // Test imm = 2047 (max positive)
                let jalr_imm_2047 = 0x7ff08067;
                let decoded = RiscVInstruction::decode(jalr_imm_2047);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, 2047);
                    }
                    _ => panic!("Expected JALR instruction"),
                }

                // Test imm = -2048 (min negative)
                let jalr_imm_neg2048 = 0x80008067;
                let decoded = RiscVInstruction::decode(jalr_imm_neg2048);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, -2048);
                    }
                    _ => panic!("Expected JALR instruction"),
                }

                // Test imm = -1
                let jalr_imm_neg1 = 0xfff08067;
                let decoded = RiscVInstruction::decode(jalr_imm_neg1);
                match decoded {
                    RiscVInstruction::Jalr { rd, rs1, imm } => {
                        assert_eq!(rd, 0);
                        assert_eq!(rs1, 1);
                        assert_eq!(imm, -1);
                    }
                    _ => panic!("Expected JALR instruction"),
                }
            }
        }

        #[test]
        fn unsupported() {
            let unsupported = 0x12345678;
            let decoded = RiscVInstruction::decode(unsupported);

            match decoded {
                RiscVInstruction::Unsupported(word) => {
                    assert_eq!(word, 0x12345678);
                }
                _ => panic!("Expected unsupported instruction"),
            }
        }
    }

    mod display {
        use super::*;

        mod jalr {
            use super::*;

            #[test]
            fn positive_immediate() {
                let jalr = RiscVInstruction::Jalr {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", jalr), "jalr x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let jalr = RiscVInstruction::Jalr {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", jalr), "jalr x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let jalr = RiscVInstruction::Jalr {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", jalr), "jalr x31, x0, 0");
            }

            #[test]
            fn boundary_values() {
                let jalr_max = RiscVInstruction::Jalr {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", jalr_max), "jalr x31, x31, 2047");

                let jalr_min = RiscVInstruction::Jalr {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", jalr_min), "jalr x0, x0, -2048");
            }
        }

        #[test]
        fn unsupported() {
            let unsupported = RiscVInstruction::Unsupported(0x12345678);
            assert_eq!(format!("{}", unsupported), "unsupported(0x12345678)");

            let unsupported_zero = RiscVInstruction::Unsupported(0x00000000);
            assert_eq!(format!("{}", unsupported_zero), "unsupported(0x00000000)");

            let unsupported_max = RiscVInstruction::Unsupported(0xffffffff);
            assert_eq!(format!("{}", unsupported_max), "unsupported(0xffffffff)");
        }
    }
}
