use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let or_x1_x2_x3 = 0x003160b3;
    let decoded = RiscVInstruction::decode(or_x1_x2_x3);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn min_rd() {
    let or_x0_x1_x2 = 0x0020e033;
    let decoded = RiscVInstruction::decode(or_x0_x1_x2);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn max_rd() {
    let or_x31_x1_x2 = 0x0020e033 | (31 << 7);
    let decoded = RiscVInstruction::decode(or_x31_x1_x2);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn min_rs1() {
    let or_x1_x0_x2 = 0x00206033 | (1 << 7);
    let decoded = RiscVInstruction::decode(or_x1_x0_x2);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn max_rs1() {
    let or_x1_x31_x2 = 0x002fe033 | (1 << 7);
    let decoded = RiscVInstruction::decode(or_x1_x31_x2);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn min_rs2() {
    let or_x1_x2_x0 = 0x00016033 | (1 << 7);
    let decoded = RiscVInstruction::decode(or_x1_x2_x0);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn max_rs2() {
    let or_x1_x2_x31 = 0x01f16033 | (1 << 7);
    let decoded = RiscVInstruction::decode(or_x1_x2_x31);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn all_max_values() {
    let or_x31_x31_x31 = 0x01ffefb3;
    let decoded = RiscVInstruction::decode(or_x31_x31_x31);

    match decoded {
        RiscVInstruction::Or { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected OR instruction"),
    }
}

#[test]
fn invalid_funct7_should_be_unsupported() {
    // OR with invalid funct7 (0x20 instead of 0x00)
    let invalid_or = 0x203160b3;
    let decoded = RiscVInstruction::decode(invalid_or);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x203160b3);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}
