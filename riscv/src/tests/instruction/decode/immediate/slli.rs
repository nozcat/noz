use crate::instruction::RiscVInstruction;

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
