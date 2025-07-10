use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let and_x1_x2_x3 = 0x003170b3;
    let decoded = RiscVInstruction::decode(and_x1_x2_x3);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn min_rd() {
    let and_x0_x1_x2 = 0x0020f033;
    let decoded = RiscVInstruction::decode(and_x0_x1_x2);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn max_rd() {
    let and_x31_x1_x2 = 0x0020f033 | (31 << 7);
    let decoded = RiscVInstruction::decode(and_x31_x1_x2);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn min_rs1() {
    let and_x1_x0_x2 = 0x00207033 | (1 << 7);
    let decoded = RiscVInstruction::decode(and_x1_x0_x2);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn max_rs1() {
    let and_x1_x31_x2 = 0x002ff033 | (1 << 7);
    let decoded = RiscVInstruction::decode(and_x1_x31_x2);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn min_rs2() {
    let and_x1_x2_x0 = 0x00017033 | (1 << 7);
    let decoded = RiscVInstruction::decode(and_x1_x2_x0);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn max_rs2() {
    let and_x1_x2_x31 = 0x01f17033 | (1 << 7);
    let decoded = RiscVInstruction::decode(and_x1_x2_x31);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn all_max_values() {
    let and_x31_x31_x31 = 0x01ffffb3;
    let decoded = RiscVInstruction::decode(and_x31_x31_x31);

    match decoded {
        RiscVInstruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected AND instruction"),
    }
}

#[test]
fn invalid_funct7_should_be_unsupported() {
    // AND with invalid funct7 (0x20 instead of 0x00)
    let invalid_and = 0x203170b3;
    let decoded = RiscVInstruction::decode(invalid_and);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x203170b3);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}

#[test]
fn invalid_funct3_should_be_unsupported() {
    // REG_OPCODE (0x33) with invalid funct3 (0x1 instead of 0x7 for AND)
    // This tests the catch-all case in the REG_OPCODE match
    let invalid_funct3 = 0x00109033; // funct3=0x1, funct7=0x00
    let decoded = RiscVInstruction::decode(invalid_funct3);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x00109033);
        }
        _ => panic!("Expected unsupported instruction for invalid funct3"),
    }
}
