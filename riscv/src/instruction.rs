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
    /// Add instruction (RV32I base instruction set)
    ///
    /// Adds the values in registers `rs1` and `rs2` and stores the result in `rd`.
    /// Performs 32-bit arithmetic addition with overflow wrapping.
    Add { rd: u8, rs1: u8, rs2: u8 },

    /// Subtract instruction (RV32I base instruction set)
    ///
    /// Subtracts the value in register `rs2` from register `rs1` and stores the result in `rd`.
    /// Performs 32-bit arithmetic subtraction with overflow wrapping.
    Sub { rd: u8, rs1: u8, rs2: u8 },

    /// XOR instruction (RV32I base instruction set)
    ///
    /// Performs bitwise XOR between registers `rs1` and `rs2` and stores the result in `rd`.
    /// Each bit in the result is 1 if the corresponding bits in the operands are different.
    Xor { rd: u8, rs1: u8, rs2: u8 },

    /// OR instruction (RV32I base instruction set)
    ///
    /// Performs bitwise OR between registers `rs1` and `rs2` and stores the result in `rd`.
    /// Each bit in the result is 1 if either corresponding bit in the operands is 1.
    Or { rd: u8, rs1: u8, rs2: u8 },

    /// AND instruction (RV32I base instruction set)
    ///
    /// Performs bitwise AND between registers `rs1` and `rs2` and stores the result in `rd`.
    /// Each bit in the result is 1 if both corresponding bits in the operands are 1.
    And { rd: u8, rs1: u8, rs2: u8 },

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
    Srli { rd: u8, rs1: u8, imm: i16 },

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

    /// Load Word instruction (RV32I base instruction set)
    ///
    /// Loads a 32-bit value from memory address `rs1 + imm` and stores it in `rd`.
    /// No sign extension is needed as it's a full 32-bit load.
    Lw { rd: u8, rs1: u8, imm: i16 },

    /// Load Byte Unsigned instruction (RV32I base instruction set)
    ///
    /// Loads an 8-bit value from memory address `rs1 + imm` and zero-extends it to 32 bits,
    /// storing the result in `rd`.
    Lbu { rd: u8, rs1: u8, imm: i16 },

    /// Load Halfword Unsigned instruction (RV32I base instruction set)
    ///
    /// Loads a 16-bit value from memory address `rs1 + imm` and zero-extends it to 32 bits,
    /// storing the result in `rd`.
    Lhu { rd: u8, rs1: u8, imm: i16 },

    /// Jump and Link Register instruction (RV32I base instruction set)
    ///
    /// Jumps to address `rs1 + imm` and saves return address in `rd`.
    /// If `rd = x0`, the return address is discarded (simple jump).
    Jalr { rd: u8, rs1: u8, imm: i16 },

    /// Environment Call instruction (RV32I base instruction set)
    ///
    /// Transfers control to the operating system to handle a system call.
    /// This instruction has no operands and is encoded as a specific system instruction.
    Ecall,

    /// Environment Break instruction (RV32I base instruction set)
    ///
    /// Transfers control to a debugger or trap handler.
    /// This instruction has no operands and is encoded as a specific system instruction.
    Ebreak,

    /// Unsupported instruction
    ///
    /// Contains the raw 32-bit instruction word for debugging purposes.
    Unsupported(u32),
}

impl fmt::Display for RiscVInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiscVInstruction::Add { rd, rs1, rs2 } => {
                write!(f, "add x{}, x{}, x{}", rd, rs1, rs2)
            }
            RiscVInstruction::Sub { rd, rs1, rs2 } => {
                write!(f, "sub x{}, x{}, x{}", rd, rs1, rs2)
            }
            RiscVInstruction::Xor { rd, rs1, rs2 } => {
                write!(f, "xor x{}, x{}, x{}", rd, rs1, rs2)
            }
            RiscVInstruction::Or { rd, rs1, rs2 } => {
                write!(f, "or x{}, x{}, x{}", rd, rs1, rs2)
            }
            RiscVInstruction::And { rd, rs1, rs2 } => {
                write!(f, "and x{}, x{}, x{}", rd, rs1, rs2)
            }
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
            RiscVInstruction::Srli { rd, rs1, imm } => {
                write!(f, "srli x{}, x{}, {}", rd, rs1, imm)
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
            RiscVInstruction::Lw { rd, rs1, imm } => {
                write!(f, "lw x{}, {}(x{})", rd, imm, rs1)
            }
            RiscVInstruction::Lbu { rd, rs1, imm } => {
                write!(f, "lbu x{}, {}(x{})", rd, imm, rs1)
            }
            RiscVInstruction::Lhu { rd, rs1, imm } => {
                write!(f, "lhu x{}, {}(x{})", rd, imm, rs1)
            }
            RiscVInstruction::Jalr { rd, rs1, imm } => {
                write!(f, "jalr x{}, x{}, {}", rd, rs1, imm)
            }
            RiscVInstruction::Ecall => {
                write!(f, "ecall")
            }
            RiscVInstruction::Ebreak => {
                write!(f, "ebreak")
            }
            RiscVInstruction::Unsupported(opcode) => {
                write!(f, "unsupported(0x{:08x})", opcode)
            }
        }
    }
}

const REG_OPCODE: u32 = 0x33;
const ADD_FUNCT3: u8 = 0x0;
const ADD_FUNCT7: u32 = 0x00;
const SUB_FUNCT7: u32 = 0x20;
const XOR_FUNCT3: u8 = 0x4;
const XOR_FUNCT7: u32 = 0x00;
const OR_FUNCT3: u8 = 0x6;
const OR_FUNCT7: u32 = 0x00;
const AND_FUNCT3: u8 = 0x7;
const AND_FUNCT7: u32 = 0x00;

const IMM_OPCODE: u32 = 0x13;
const ADDI_FUNCT3: u8 = 0x0;
const SLTI_FUNCT3: u8 = 0x2;
const SLTIU_FUNCT3: u8 = 0x3;
const XORI_FUNCT3: u8 = 0x4;
const ORI_FUNCT3: u8 = 0x6;
const ANDI_FUNCT3: u8 = 0x7;
const SLLI_FUNCT3: u8 = 0x1;
const SRLI_FUNCT3: u8 = 0x5;
const SLLI_FUNCT7: u32 = 0x00;
const SRLI_FUNCT7: u32 = 0x00;
const SRAI_FUNCT7: u32 = 0x20;

const LOAD_OPCODE: u32 = 0x03;
const LB_FUNCT3: u8 = 0x0;
const LH_FUNCT3: u8 = 0x1;
const LW_FUNCT3: u8 = 0x2;
const LBU_FUNCT3: u8 = 0x4;
const LHU_FUNCT3: u8 = 0x5;

const JALR_OPCODE: u32 = 0x67;
const JALR_FUNCT3: u32 = 0x0;

const SYSTEM_OPCODE: u32 = 0x73;
const SYSTEM_FUNCT3: u32 = 0x0;
const ECALL_IMM: u32 = 0x0;
const EBREAK_IMM: u32 = 0x1;

const OPCODE_MASK: u32 = 0x7f;
const FUNCT3_MASK: u32 = 0x7000;
const RD_MASK: u32 = 0xf80;
const RS1_MASK: u32 = 0xf8000;
const RS2_MASK: u32 = 0x1f00000;
const IMM_I_MASK: u32 = 0xfff00000;
const FUNCT7_MASK: u32 = 0xfe000000;

const FUNCT3_SHIFT: u32 = 12;
const RD_SHIFT: u32 = 7;
const RS1_SHIFT: u32 = 15;
const RS2_SHIFT: u32 = 20;
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
            REG_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let funct7 = (word & FUNCT7_MASK) >> FUNCT7_SHIFT;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let rs2 = ((word & RS2_MASK) >> RS2_SHIFT) as u8;

                match funct3 {
                    ADD_FUNCT3 => {
                        if funct7 == ADD_FUNCT7 {
                            RiscVInstruction::Add { rd, rs1, rs2 }
                        } else if funct7 == SUB_FUNCT7 {
                            RiscVInstruction::Sub { rd, rs1, rs2 }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    XOR_FUNCT3 => {
                        if funct7 == XOR_FUNCT7 {
                            RiscVInstruction::Xor { rd, rs1, rs2 }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    OR_FUNCT3 => {
                        if funct7 == OR_FUNCT7 {
                            RiscVInstruction::Or { rd, rs1, rs2 }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    AND_FUNCT3 => {
                        if funct7 == AND_FUNCT7 {
                            RiscVInstruction::And { rd, rs1, rs2 }
                        } else {
                            RiscVInstruction::Unsupported(word)
                        }
                    }
                    _ => RiscVInstruction::Unsupported(word),
                }
            }
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
                    SRLI_FUNCT3 => {
                        if funct7 == SRAI_FUNCT7 {
                            let shift_imm = imm & 0x1f;
                            RiscVInstruction::Srai {
                                rd,
                                rs1,
                                imm: shift_imm,
                            }
                        } else if funct7 == SRLI_FUNCT7 {
                            let shift_imm = imm & 0x1f;
                            RiscVInstruction::Srli {
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
            LOAD_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let imm = ((word & IMM_I_MASK) as i32 >> IMM_I_SHIFT) as i16;

                match funct3 {
                    LB_FUNCT3 => RiscVInstruction::Lb { rd, rs1, imm },
                    LH_FUNCT3 => RiscVInstruction::Lh { rd, rs1, imm },
                    LW_FUNCT3 => RiscVInstruction::Lw { rd, rs1, imm },
                    LBU_FUNCT3 => RiscVInstruction::Lbu { rd, rs1, imm },
                    LHU_FUNCT3 => RiscVInstruction::Lhu { rd, rs1, imm },
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
            SYSTEM_OPCODE => {
                let funct3 = (word & FUNCT3_MASK) >> FUNCT3_SHIFT;
                if funct3 == SYSTEM_FUNCT3 {
                    let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                    let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                    let imm = (word & IMM_I_MASK) >> IMM_I_SHIFT;

                    // ECALL and EBREAK require rd=0 and rs1=0
                    if rd == 0 && rs1 == 0 {
                        match imm {
                            ECALL_IMM => RiscVInstruction::Ecall,
                            EBREAK_IMM => RiscVInstruction::Ebreak,
                            _ => RiscVInstruction::Unsupported(word),
                        }
                    } else {
                        RiscVInstruction::Unsupported(word)
                    }
                } else {
                    RiscVInstruction::Unsupported(word)
                }
            }
            _ => RiscVInstruction::Unsupported(word),
        }
    }
}
