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

    /// OR Immediate instruction (RV32I base instruction set)
    ///
    /// Performs bitwise OR between register `rs1` and the immediate value,
    /// storing the result in `rd`.
    Ori { rd: u8, rs1: u8, imm: i16 },

    /// AND Immediate instruction (RV32I base instruction set)
    ///
    /// Performs bitwise AND between register `rs1` and the immediate value,
    /// storing the result in `rd`.
    Andi { rd: u8, rs1: u8, imm: i16 },

    /// Shift Left Logical Immediate instruction (RV32I base instruction set)
    ///
    /// Shifts register `rs1` left by the immediate value (0-31 bits) and
    /// stores the result in `rd`. Zero bits are shifted in from the right.
    Slli { rd: u8, rs1: u8, imm: i16 },

    /// Shift Right Logical Immediate instruction (RV32I base instruction set)
    ///
    /// Shifts register `rs1` right by the immediate value (0-31 bits) and
    /// stores the result in `rd`. Zero bits are shifted in from the left.
    Slri { rd: u8, rs1: u8, imm: i16 },

    /// Shift Right Arithmetic Immediate instruction (RV32I base instruction set)
    ///
    /// Shifts register `rs1` right by the immediate value (0-31 bits) and
    /// stores the result in `rd`. Sign bits are shifted in from the left.
    Srai { rd: u8, rs1: u8, imm: i16 },

    /// Set Less Than Immediate instruction (RV32I base instruction set)
    ///
    /// Compares register `rs1` with the immediate value using signed comparison.
    /// Sets `rd` to 1 if `rs1` < imm, otherwise sets `rd` to 0.
    Slti { rd: u8, rs1: u8, imm: i16 },

    /// Set Less Than Immediate Unsigned instruction (RV32I base instruction set)
    ///
    /// Compares register `rs1` with the immediate value using unsigned comparison.
    /// Sets `rd` to 1 if `rs1` < imm (unsigned), otherwise sets `rd` to 0.
    Sltiu { rd: u8, rs1: u8, imm: i16 },

    /// Load Byte instruction (RV32I base instruction set)
    ///
    /// Loads an 8-bit value from memory address `rs1 + imm` and sign-extends it to 32 bits,
    /// storing the result in `rd`.
    Lb { rd: u8, rs1: u8, imm: i16 },

    /// Load Halfword instruction (RV32I base instruction set)
    ///
    /// Loads a 16-bit value from memory address `rs1 + imm` and sign-extends it to 32 bits,
    /// storing the result in `rd`.
    Lh { rd: u8, rs1: u8, imm: i16 },

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
            RiscVInstruction::Ori { rd, rs1, imm } => {
                write!(f, "ori x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Andi { rd, rs1, imm } => {
                write!(f, "andi x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Slli { rd, rs1, imm } => {
                write!(f, "slli x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Slri { rd, rs1, imm } => {
                write!(f, "slri x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Srai { rd, rs1, imm } => {
                write!(f, "srai x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Slti { rd, rs1, imm } => {
                write!(f, "slti x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Sltiu { rd, rs1, imm } => {
                write!(f, "sltiu x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Lb { rd, rs1, imm } => {
                write!(f, "lb x{}, {}(x{})", rd, imm, rs1)
            }
            RiscVInstruction::Lh { rd, rs1, imm } => {
                write!(f, "lh x{}, {}(x{})", rd, imm, rs1)
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
const ADDI_FUNCT3: u8 = 0x0;
const SLTI_FUNCT3: u8 = 0x2;
const SLTIU_FUNCT3: u8 = 0x3;
const XORI_FUNCT3: u8 = 0x4;
const ORI_FUNCT3: u8 = 0x6;
const ANDI_FUNCT3: u8 = 0x7;
const SLLI_FUNCT3: u8 = 0x1;
const SLRI_FUNCT3: u8 = 0x5;

const SLLI_FUNCT7: u32 = 0x00;
const SLRI_FUNCT7: u32 = 0x00;
const SRAI_FUNCT7: u32 = 0x20;

const JALR_OPCODE: u32 = 0x67;
const JALR_FUNCT3: u32 = 0x0;

const LOAD_OPCODE: u32 = 0x03;
const LB_FUNCT3: u8 = 0x0;
const LH_FUNCT3: u8 = 0x1;

const OPCODE_MASK: u32 = 0x7f;
const FUNCT3_MASK: u32 = 0x7000;
const RD_MASK: u32 = 0xf80;
const RS1_MASK: u32 = 0xf8000;
const IMM_I_MASK: u32 = 0xfff00000;
const FUNCT7_MASK: u32 = 0xfe000000;

const FUNCT3_SHIFT: u32 = 12;
const RD_SHIFT: u32 = 7;
const RS1_SHIFT: u32 = 15;
const IMM_I_SHIFT: u32 = 20;
const FUNCT7_SHIFT: u32 = 25;

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
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let imm = ((word & IMM_I_MASK) as i32 >> IMM_I_SHIFT) as i16;
                let funct7 = (word & FUNCT7_MASK) >> FUNCT7_SHIFT;

                match funct3 {
                    ADDI_FUNCT3 => RiscVInstruction::Addi { rd, rs1, imm },
                    SLTI_FUNCT3 => RiscVInstruction::Slti { rd, rs1, imm },
                    SLTIU_FUNCT3 => RiscVInstruction::Sltiu { rd, rs1, imm },
                    SLLI_FUNCT3 => {
                        if funct7 == SLLI_FUNCT7 {
                            let shift_imm = imm & 0x1f;
                            RiscVInstruction::Slli {
                                rd,
                                rs1,
                                imm: shift_imm,
                            }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    SLRI_FUNCT3 => {
                        if funct7 == SRAI_FUNCT7 {
                            let shift_imm = imm & 0x1f;
                            RiscVInstruction::Srai {
                                rd,
                                rs1,
                                imm: shift_imm,
                            }
                        } else if funct7 == SLRI_FUNCT7 {
                            let shift_imm = imm & 0x1f;
                            RiscVInstruction::Slri {
                                rd,
                                rs1,
                                imm: shift_imm,
                            }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    XORI_FUNCT3 => RiscVInstruction::Xori { rd, rs1, imm },
                    ORI_FUNCT3 => RiscVInstruction::Ori { rd, rs1, imm },
                    ANDI_FUNCT3 => RiscVInstruction::Andi { rd, rs1, imm },
                    _ => unreachable!("All 3-bit funct3 values are handled above"),
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
            LOAD_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let imm = ((word & IMM_I_MASK) as i32 >> IMM_I_SHIFT) as i16;

                match funct3 {
                    LB_FUNCT3 => RiscVInstruction::Lb { rd, rs1, imm },
                    LH_FUNCT3 => RiscVInstruction::Lh { rd, rs1, imm },
                    _ => RiscVInstruction::Unsupported(word),
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

            mod ori {
                use super::*;

                #[test]
                fn basic() {
                    let ori_x1_x2_100 = 0x06416093;
                    let decoded = RiscVInstruction::decode(ori_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let ori_x0_x1_0 = 0x0000e013;
                    let decoded = RiscVInstruction::decode(ori_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let ori_x31_x1_0 = 0x0000e013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(ori_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let ori_x1_x0_0 = 0x00006093;
                    let decoded = RiscVInstruction::decode(ori_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let ori_x1_x31_0 = 0x000fe093;
                    let decoded = RiscVInstruction::decode(ori_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let ori_x0_x1_neg4 = 0xffc0e013;
                    let decoded = RiscVInstruction::decode(ori_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let ori_x1_x2_0 = 0x00016093;
                    let decoded = RiscVInstruction::decode(ori_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let ori_x1_x0_2047 = 0x7ff06093;
                    let decoded = RiscVInstruction::decode(ori_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let ori_x1_x0_neg2048 = 0x80006093;
                    let decoded = RiscVInstruction::decode(ori_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Ori { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected ORI instruction"),
                    }
                }
            }

            mod andi {
                use super::*;

                #[test]
                fn basic() {
                    let andi_x1_x2_100 = 0x06417093;
                    let decoded = RiscVInstruction::decode(andi_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let andi_x0_x1_0 = 0x0000f013;
                    let decoded = RiscVInstruction::decode(andi_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let andi_x31_x1_0 = 0x0000f013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(andi_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let andi_x1_x0_0 = 0x00007093;
                    let decoded = RiscVInstruction::decode(andi_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let andi_x1_x31_0 = 0x000ff093;
                    let decoded = RiscVInstruction::decode(andi_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let andi_x0_x1_neg4 = 0xffc0f013;
                    let decoded = RiscVInstruction::decode(andi_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let andi_x1_x2_0 = 0x00017093;
                    let decoded = RiscVInstruction::decode(andi_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let andi_x1_x0_2047 = 0x7ff07093;
                    let decoded = RiscVInstruction::decode(andi_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let andi_x1_x0_neg2048 = 0x80007093;
                    let decoded = RiscVInstruction::decode(andi_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Andi { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected ANDI instruction"),
                    }
                }
            }

            mod slli {
                use super::*;

                #[test]
                fn basic() {
                    let slli_x1_x2_5 = 0x00511093;
                    let decoded = RiscVInstruction::decode(slli_x1_x2_5);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 5);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let slli_x0_x1_1 = 0x00109013;
                    let decoded = RiscVInstruction::decode(slli_x0_x1_1);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let slli_x31_x1_1 = 0x00109013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(slli_x31_x1_1);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let slli_x1_x0_1 = 0x00101093;
                    let decoded = RiscVInstruction::decode(slli_x1_x0_1);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let slli_x1_x31_1 = 0x001f9093;
                    let decoded = RiscVInstruction::decode(slli_x1_x31_1);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let slli_x1_x2_0 = 0x00011093;
                    let decoded = RiscVInstruction::decode(slli_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }

                #[test]
                fn max_shift_amount() {
                    let slli_x1_x2_31 = 0x01f11093;
                    let decoded = RiscVInstruction::decode(slli_x1_x2_31);

                    match decoded {
                        RiscVInstruction::Slli { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 31);
                        }
                        _ => panic!("Expected SLLI instruction"),
                    }
                }
            }

            mod slri {
                use super::*;

                #[test]
                fn basic() {
                    let slri_x1_x2_5 = 0x00515093;
                    let decoded = RiscVInstruction::decode(slri_x1_x2_5);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 5);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let slri_x0_x1_1 = 0x0010d013;
                    let decoded = RiscVInstruction::decode(slri_x0_x1_1);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let slri_x31_x1_1 = 0x0010d013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(slri_x31_x1_1);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let slri_x1_x0_1 = 0x00105093;
                    let decoded = RiscVInstruction::decode(slri_x1_x0_1);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let slri_x1_x31_1 = 0x001fd093;
                    let decoded = RiscVInstruction::decode(slri_x1_x31_1);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let slri_x1_x2_0 = 0x00015093;
                    let decoded = RiscVInstruction::decode(slri_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }

                #[test]
                fn max_shift_amount() {
                    let slri_x1_x2_31 = 0x01f15093;
                    let decoded = RiscVInstruction::decode(slri_x1_x2_31);

                    match decoded {
                        RiscVInstruction::Slri { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 31);
                        }
                        _ => panic!("Expected SLRI instruction"),
                    }
                }
            }

            mod srai {
                use super::*;

                #[test]
                fn basic() {
                    let srai_x1_x2_5 = 0x00515093 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x1_x2_5);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 5);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let srai_x0_x1_1 = 0x0010d013 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x0_x1_1);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let srai_x31_x1_1 = (0x0010d013 | 0x40000000) | (31 << 7);
                    let decoded = RiscVInstruction::decode(srai_x31_x1_1);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let srai_x1_x0_1 = 0x00105093 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x1_x0_1);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let srai_x1_x31_1 = 0x001fd093 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x1_x31_1);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 1);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let srai_x1_x2_0 = 0x00015093 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }

                #[test]
                fn max_shift_amount() {
                    let srai_x1_x2_31 = 0x01f15093 | 0x40000000;
                    let decoded = RiscVInstruction::decode(srai_x1_x2_31);

                    match decoded {
                        RiscVInstruction::Srai { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 31);
                        }
                        _ => panic!("Expected SRAI instruction"),
                    }
                }
            }

            mod slti {
                use super::*;

                #[test]
                fn basic() {
                    let slti_x1_x2_100 = 0x06412093;
                    let decoded = RiscVInstruction::decode(slti_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let slti_x0_x1_0 = 0x0000a013;
                    let decoded = RiscVInstruction::decode(slti_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let slti_x31_x1_0 = 0x0000a013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(slti_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let slti_x1_x0_0 = 0x00002093;
                    let decoded = RiscVInstruction::decode(slti_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let slti_x1_x31_0 = 0x000fa093;
                    let decoded = RiscVInstruction::decode(slti_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let slti_x0_x1_neg4 = 0xffc0a013;
                    let decoded = RiscVInstruction::decode(slti_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let slti_x1_x2_0 = 0x00012093;
                    let decoded = RiscVInstruction::decode(slti_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let slti_x1_x0_2047 = 0x7ff02093;
                    let decoded = RiscVInstruction::decode(slti_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let slti_x1_x0_neg2048 = 0x80002093;
                    let decoded = RiscVInstruction::decode(slti_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Slti { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected SLTI instruction"),
                    }
                }
            }

            mod sltiu {
                use super::*;

                #[test]
                fn basic() {
                    let sltiu_x1_x2_100 = 0x06413093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let sltiu_x0_x1_0 = 0x0000b013;
                    let decoded = RiscVInstruction::decode(sltiu_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let sltiu_x31_x1_0 = 0x0000b013 | (31 << 7);
                    let decoded = RiscVInstruction::decode(sltiu_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let sltiu_x1_x0_0 = 0x00003093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let sltiu_x1_x31_0 = 0x000fb093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let sltiu_x0_x1_neg4 = 0xffc0b013;
                    let decoded = RiscVInstruction::decode(sltiu_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let sltiu_x1_x2_0 = 0x00013093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let sltiu_x1_x0_2047 = 0x7ff03093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let sltiu_x1_x0_neg2048 = 0x80003093;
                    let decoded = RiscVInstruction::decode(sltiu_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Sltiu { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected SLTIU instruction"),
                    }
                }
            }

            mod lb {
                use super::*;

                #[test]
                fn basic() {
                    let lb_x1_x2_100 = 0x06410083;
                    let decoded = RiscVInstruction::decode(lb_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let lb_x0_x1_0 = 0x00008003;
                    let decoded = RiscVInstruction::decode(lb_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let lb_x31_x1_0 = 0x00008003 | (31 << 7);
                    let decoded = RiscVInstruction::decode(lb_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let lb_x1_x0_0 = 0x00000083;
                    let decoded = RiscVInstruction::decode(lb_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let lb_x1_x31_0 = 0x000f8083;
                    let decoded = RiscVInstruction::decode(lb_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let lb_x0_x1_neg4 = 0xffc08003;
                    let decoded = RiscVInstruction::decode(lb_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let lb_x1_x2_0 = 0x00010083;
                    let decoded = RiscVInstruction::decode(lb_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let lb_x1_x0_2047 = 0x7ff00083;
                    let decoded = RiscVInstruction::decode(lb_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let lb_x1_x0_neg2048 = 0x80000083;
                    let decoded = RiscVInstruction::decode(lb_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Lb { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected LB instruction"),
                    }
                }
            }

            mod lh {
                use super::*;

                #[test]
                fn basic() {
                    let lh_x1_x2_100 = 0x06411083;
                    let decoded = RiscVInstruction::decode(lh_x1_x2_100);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 100);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn min_rd() {
                    let lh_x0_x1_0 = 0x00009003;
                    let decoded = RiscVInstruction::decode(lh_x0_x1_0);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn max_rd() {
                    let lh_x31_x1_0 = 0x00009003 | (31 << 7);
                    let decoded = RiscVInstruction::decode(lh_x31_x1_0);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 31);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn min_rs1() {
                    let lh_x1_x0_0 = 0x00001083;
                    let decoded = RiscVInstruction::decode(lh_x1_x0_0);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn max_rs1() {
                    let lh_x1_x31_0 = 0x000f9083;
                    let decoded = RiscVInstruction::decode(lh_x1_x31_0);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 31);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn negative_imm() {
                    let lh_x0_x1_neg4 = 0xffc09003;
                    let decoded = RiscVInstruction::decode(lh_x0_x1_neg4);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 0);
                            assert_eq!(rs1, 1);
                            assert_eq!(imm, -4);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn zero_imm() {
                    let lh_x1_x2_0 = 0x00011083;
                    let decoded = RiscVInstruction::decode(lh_x1_x2_0);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 2);
                            assert_eq!(imm, 0);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn max_positive_imm() {
                    let lh_x1_x0_2047 = 0x7ff01083;
                    let decoded = RiscVInstruction::decode(lh_x1_x0_2047);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, 2047);
                        }
                        _ => panic!("Expected LH instruction"),
                    }
                }

                #[test]
                fn min_negative_imm() {
                    let lh_x1_x0_neg2048 = 0x80001083;
                    let decoded = RiscVInstruction::decode(lh_x1_x0_neg2048);

                    match decoded {
                        RiscVInstruction::Lh { rd, rs1, imm } => {
                            assert_eq!(rd, 1);
                            assert_eq!(rs1, 0);
                            assert_eq!(imm, -2048);
                        }
                        _ => panic!("Expected LH instruction"),
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
            fn opcode() {
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
            fn slli_invalid_funct7() {
                let slli_with_invalid_funct7 = 0x02109093;
                let decoded = RiscVInstruction::decode(slli_with_invalid_funct7);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x02109093);
                    }
                    _ => panic!("Expected unsupported instruction for SLLI with invalid funct7"),
                }
            }

            #[test]
            fn slri_invalid_funct7() {
                let slri_with_invalid_funct7 = 0x02105093;
                let decoded = RiscVInstruction::decode(slri_with_invalid_funct7);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x02105093);
                    }
                    _ => panic!("Expected unsupported instruction for SLRI with invalid funct7"),
                }
            }

            #[test]
            fn srai_invalid_funct7() {
                let srai_with_invalid_funct7 = 0x60105093;
                let decoded = RiscVInstruction::decode(srai_with_invalid_funct7);

                match decoded {
                    RiscVInstruction::Unsupported(word) => {
                        assert_eq!(word, 0x60105093);
                    }
                    _ => panic!("Expected unsupported instruction for SRAI with invalid funct7"),
                }
            }

            #[test]
            fn jalr_funct3() {
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

        mod ori {
            use super::*;

            #[test]
            fn positive_immediate() {
                let ori = RiscVInstruction::Ori {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", ori), "ori x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let ori = RiscVInstruction::Ori {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", ori), "ori x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let ori = RiscVInstruction::Ori {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", ori), "ori x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let ori_min = RiscVInstruction::Ori {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", ori_min), "ori x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let ori_max = RiscVInstruction::Ori {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", ori_max), "ori x31, x31, 2047");
            }
        }

        mod andi {
            use super::*;

            #[test]
            fn positive_immediate() {
                let andi = RiscVInstruction::Andi {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", andi), "andi x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let andi = RiscVInstruction::Andi {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", andi), "andi x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let andi = RiscVInstruction::Andi {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", andi), "andi x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let andi_min = RiscVInstruction::Andi {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", andi_min), "andi x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let andi_max = RiscVInstruction::Andi {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", andi_max), "andi x31, x31, 2047");
            }
        }

        mod slli {
            use super::*;

            #[test]
            fn positive_immediate() {
                let slli = RiscVInstruction::Slli {
                    rd: 1,
                    rs1: 2,
                    imm: 5,
                };
                assert_eq!(format!("{}", slli), "slli x1, x2, 5");
            }

            #[test]
            fn zero_immediate() {
                let slli = RiscVInstruction::Slli {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", slli), "slli x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let slli_min = RiscVInstruction::Slli {
                    rd: 0,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", slli_min), "slli x0, x0, 0");
            }

            #[test]
            fn max_values() {
                let slli_max = RiscVInstruction::Slli {
                    rd: 31,
                    rs1: 31,
                    imm: 31,
                };
                assert_eq!(format!("{}", slli_max), "slli x31, x31, 31");
            }
        }

        mod slri {
            use super::*;

            #[test]
            fn positive_immediate() {
                let slri = RiscVInstruction::Slri {
                    rd: 1,
                    rs1: 2,
                    imm: 5,
                };
                assert_eq!(format!("{}", slri), "slri x1, x2, 5");
            }

            #[test]
            fn zero_immediate() {
                let slri = RiscVInstruction::Slri {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", slri), "slri x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let slri_min = RiscVInstruction::Slri {
                    rd: 0,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", slri_min), "slri x0, x0, 0");
            }

            #[test]
            fn max_values() {
                let slri_max = RiscVInstruction::Slri {
                    rd: 31,
                    rs1: 31,
                    imm: 31,
                };
                assert_eq!(format!("{}", slri_max), "slri x31, x31, 31");
            }
        }

        mod srai {
            use super::*;

            #[test]
            fn positive_immediate() {
                let srai = RiscVInstruction::Srai {
                    rd: 1,
                    rs1: 2,
                    imm: 5,
                };
                assert_eq!(format!("{}", srai), "srai x1, x2, 5");
            }

            #[test]
            fn zero_immediate() {
                let srai = RiscVInstruction::Srai {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", srai), "srai x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let srai_min = RiscVInstruction::Srai {
                    rd: 0,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", srai_min), "srai x0, x0, 0");
            }

            #[test]
            fn max_values() {
                let srai_max = RiscVInstruction::Srai {
                    rd: 31,
                    rs1: 31,
                    imm: 31,
                };
                assert_eq!(format!("{}", srai_max), "srai x31, x31, 31");
            }
        }

        mod slti {
            use super::*;

            #[test]
            fn positive_immediate() {
                let slti = RiscVInstruction::Slti {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", slti), "slti x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let slti = RiscVInstruction::Slti {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", slti), "slti x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let slti = RiscVInstruction::Slti {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", slti), "slti x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let slti_min = RiscVInstruction::Slti {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", slti_min), "slti x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let slti_max = RiscVInstruction::Slti {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", slti_max), "slti x31, x31, 2047");
            }
        }

        mod sltiu {
            use super::*;

            #[test]
            fn positive_immediate() {
                let sltiu = RiscVInstruction::Sltiu {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", sltiu), "sltiu x1, x2, 100");
            }

            #[test]
            fn negative_immediate() {
                let sltiu = RiscVInstruction::Sltiu {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", sltiu), "sltiu x0, x1, -4");
            }

            #[test]
            fn zero_immediate() {
                let sltiu = RiscVInstruction::Sltiu {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", sltiu), "sltiu x31, x0, 0");
            }

            #[test]
            fn min_values() {
                let sltiu_min = RiscVInstruction::Sltiu {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", sltiu_min), "sltiu x0, x0, -2048");
            }

            #[test]
            fn max_values() {
                let sltiu_max = RiscVInstruction::Sltiu {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", sltiu_max), "sltiu x31, x31, 2047");
            }
        }

        mod lb {
            use super::*;

            #[test]
            fn positive_immediate() {
                let lb = RiscVInstruction::Lb {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", lb), "lb x1, 100(x2)");
            }

            #[test]
            fn negative_immediate() {
                let lb = RiscVInstruction::Lb {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", lb), "lb x0, -4(x1)");
            }

            #[test]
            fn zero_immediate() {
                let lb = RiscVInstruction::Lb {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", lb), "lb x31, 0(x0)");
            }

            #[test]
            fn min_values() {
                let lb_min = RiscVInstruction::Lb {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", lb_min), "lb x0, -2048(x0)");
            }

            #[test]
            fn max_values() {
                let lb_max = RiscVInstruction::Lb {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", lb_max), "lb x31, 2047(x31)");
            }
        }

        mod lh {
            use super::*;

            #[test]
            fn positive_immediate() {
                let lh = RiscVInstruction::Lh {
                    rd: 1,
                    rs1: 2,
                    imm: 100,
                };
                assert_eq!(format!("{}", lh), "lh x1, 100(x2)");
            }

            #[test]
            fn negative_immediate() {
                let lh = RiscVInstruction::Lh {
                    rd: 0,
                    rs1: 1,
                    imm: -4,
                };
                assert_eq!(format!("{}", lh), "lh x0, -4(x1)");
            }

            #[test]
            fn zero_immediate() {
                let lh = RiscVInstruction::Lh {
                    rd: 31,
                    rs1: 0,
                    imm: 0,
                };
                assert_eq!(format!("{}", lh), "lh x31, 0(x0)");
            }

            #[test]
            fn min_values() {
                let lh_min = RiscVInstruction::Lh {
                    rd: 0,
                    rs1: 0,
                    imm: -2048,
                };
                assert_eq!(format!("{}", lh_min), "lh x0, -2048(x0)");
            }

            #[test]
            fn max_values() {
                let lh_max = RiscVInstruction::Lh {
                    rd: 31,
                    rs1: 31,
                    imm: 2047,
                };
                assert_eq!(format!("{}", lh_max), "lh x31, 2047(x31)");
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
