use crate::instruction::RiscVInstruction;

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

#[test]
fn invalid_funct7() {
    let srai_with_invalid_funct7 = 0x60105093;
    let decoded = RiscVInstruction::decode(srai_with_invalid_funct7);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x60105093);
        }
        _ => panic!("Expected unsupported instruction for SRAI with invalid funct7"),
    }
}
