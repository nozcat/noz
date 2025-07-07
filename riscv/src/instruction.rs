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
    /// Add Immediate instruction (RV32I base instruction set)
    ///
    /// Adds the immediate value to register `rs1` and stores the result in `rd`.
    /// If `rs1 = x0`, this effectively loads the immediate value into `rd`.
    Addi { rd: u8, rs1: u8, imm: i16 },

    /// XOR Immediate instruction (RV32I base instruction set)
    ///
    /// Performs bitwise XOR between register `rs1` and the immediate value,
    /// storing the result in `rd`.
    Xori { rd: u8, rs1: u8, imm: i16 },

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
            RiscVInstruction::Addi { rd, rs1, imm } => {
                write!(f, "addi x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Xori { rd, rs1, imm } => {
                write!(f, "xori x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Jalr { rd, rs1, imm } => {
                write!(f, "jalr x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Unsupported(opcode) => {
                write!(f, "unsupported(0x{:08x})", opcode)
            }
        }
    }
}

const IMM_OPCODE: u32 = 0x13;
const ADDI_FUNCT3: u32 = 0x0;
const XORI_FUNCT3: u32 = 0x4;

const JALR_OPCODE: u32 = 0x67;
const JALR_FUNCT3: u32 = 0x0;

const OPCODE_MASK: u32 = 0x7f;
const FUNCT3_MASK: u32 = 0x7000;
const RD_MASK: u32 = 0xf80;
const RS1_MASK: u32 = 0xf8000;
const IMM_I_MASK: u32 = 0xfff00000;

const FUNCT3_SHIFT: u32 = 12;
const RD_SHIFT: u32 = 7;
const RS1_SHIFT: u32 = 15;
const IMM_I_SHIFT: u32 = 20;

impl RiscVInstruction {
    /// Decode a 32-bit instruction word into a RiscVInstruction
    ///
    /// # Arguments
    ///
    /// * `word` - The 32-bit instruction word to decode
    pub fn decode(word: u32) -> RiscVInstruction {
        let opcode = word & OPCODE_MASK;

        match opcode {
            IMM_OPCODE => {
                let funct3 = (word & FUNCT3_MASK) >> FUNCT3_SHIFT;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let imm = ((word & IMM_I_MASK) as i32 >> IMM_I_SHIFT) as i16;

                match funct3 {
                    ADDI_FUNCT3 => RiscVInstruction::Addi { rd, rs1, imm },
                    XORI_FUNCT3 => RiscVInstruction::Xori { rd, rs1, imm },
                    _ => RiscVInstruction::Unsupported(word),
                }
            }
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

        mod immediate_instructions {
            use super::*;

            mod addi {
                use super::*;

                #[test]
                fn basic() {
                    let addi_x1_x2_100 = 0x06410093;
                    let decoded = RiscVInstruction::decode(addi_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let addi_x0_x1_0 = 0x00008013;
                    let decoded = RiscVInstruction::decode(addi_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let addi_x31_x1_0 = 0x00008013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(addi_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let addi_x1_x0_0 = 0x00000093;
                    let decoded = RiscVInstruction::decode(addi_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let addi_x1_x31_0 = 0x000f8093;
                    let decoded = RiscVInstruction::decode(addi_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let addi_x0_x1_neg4 = 0xffc08013;
                    let decoded = RiscVInstruction::decode(addi_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let addi_x1_x2_0 = 0x00010093;
                    let decoded = RiscVInstruction::decode(addi_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let addi_x1_x0_2047 = 0x7ff00093;
                    let decoded = RiscVInstruction::decode(addi_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let addi_x1_x0_neg2048 = 0x80000093;
                    let decoded = RiscVInstruction::decode(addi_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Addi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected ADDI instruction"),
                    }
                }
            }

            mod xori {
                use super::*;

                #[test]
                fn basic() {
                    let xori_x1_x2_100 = 0x06414093;
                    let decoded = RiscVInstruction::decode(xori_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let xori_x0_x1_0 = 0x0000c013;
                    let decoded = RiscVInstruction::decode(xori_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let xori_x31_x1_0 = 0x0000c013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(xori_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let xori_x1_x0_0 = 0x00004093;
                    let decoded = RiscVInstruction::decode(xori_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let xori_x1_x31_0 = 0x000fc093;
                    let decoded = RiscVInstruction::decode(xori_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let xori_x0_x1_neg4 = 0xffc0c013;
                    let decoded = RiscVInstruction::decode(xori_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let xori_x1_x2_0 = 0x00014093;
                    let decoded = RiscVInstruction::decode(xori_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let xori_x1_x0_2047 = 0x7ff04093;
                    let decoded = RiscVInstruction::decode(xori_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let xori_x1_x0_neg2048 = 0x80004093;
                    let decoded = RiscVInstruction::decode(xori_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Xori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected XORI instruction"),
                    }
                }
            }
        }

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
            fn min_rd() {
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
            }

            #[test]
            fn max_rd() {
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
            fn min_rs1() {
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
            }

            #[test]
            fn max_rs1() {
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
            fn zero_imm() {
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
            }

            #[test]
            fn max_positive_imm() {
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
            }

            #[test]
            fn min_negative_imm() {
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
            }

            #[test]
            fn neg_one_imm() {
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

        mod unsupported {
            use super::*;

            #[test]
            fn unsupported_opcode() {
                let unsupported = 0x12345678;
                let decoded = RiscVInstruction::decode(unsupported);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x12345678);
                    }
                    _ => panic!("Expected unsupported instruction"),
                }
            }

            #[test]
            fn unsupported_immediate_funct3() {
                let imm_with_invalid_funct3 = 0x00411093;
                let decoded = RiscVInstruction::decode(imm_with_invalid_funct3);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x00411093);
                    }
                    _ => panic!(
                        "Expected unsupported instruction for immediate instruction with invalid funct3"
                    ),
                }
            }

            #[test]
            fn unsupported_jalr_funct3() {
                let jalr_with_invalid_funct3 = 0x004110e7;
                let decoded = RiscVInstruction::decode(jalr_with_invalid_funct3);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x004110e7);
                    }
                    _ => panic!("Expected unsupported instruction for JALR with invalid funct3"),
                }
            }
        }
    }

    mod display {
        use super::*;

        mod addi {
            use super::*;

            #[test]
            fn positive_immediate() {
                let addi = RiscVInstruction::Addi {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", addi), "addi x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let addi = RiscVInstruction::Addi {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", addi), "addi x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let addi = RiscVInstruction::Addi {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", addi), "addi x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let addi_min = RiscVInstruction::Addi {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", addi_min), "addi x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let addi_max = RiscVInstruction::Addi {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", addi_max), "addi x31, x31, 2047");
            }
        }

        mod xori {
            use super::*;

            #[test]
            fn positive_immediate() {
                let xori = RiscVInstruction::Xori {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", xori), "xori x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let xori = RiscVInstruction::Xori {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", xori), "xori x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let xori = RiscVInstruction::Xori {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", xori), "xori x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let xori_min = RiscVInstruction::Xori {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", xori_min), "xori x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let xori_max = RiscVInstruction::Xori {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", xori_max), "xori x31, x31, 2047");
            }
        }

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
            fn min_values() {
                let jalr_min = RiscVInstruction::Jalr {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", jalr_min), "jalr x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let jalr_max = RiscVInstruction::Jalr {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", jalr_max), "jalr x31, x31, 2047");
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
