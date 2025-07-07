use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let sub_x1_x2_x3 = 0x403100b3;
    let decoded = RiscVInstruction::decode(sub_x1_x2_x3);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn min_rd() {
    let sub_x0_x1_x2 = 0x40208033;
    let decoded = RiscVInstruction::decode(sub_x0_x1_x2);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn max_rd() {
    let sub_x31_x1_x2 = 0x40208033 | (31 << 7);
    let decoded = RiscVInstruction::decode(sub_x31_x1_x2);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn min_rs1() {
    let sub_x1_x0_x2 = 0x40200033 | (1 << 7);
    let decoded = RiscVInstruction::decode(sub_x1_x0_x2);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn max_rs1() {
    let sub_x1_x31_x2 = 0x402f8033 | (1 << 7);
    let decoded = RiscVInstruction::decode(sub_x1_x31_x2);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn min_rs2() {
    let sub_x1_x2_x0 = 0x40010033 | (1 << 7);
    let decoded = RiscVInstruction::decode(sub_x1_x2_x0);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn max_rs2() {
    let sub_x1_x2_x31 = 0x41f10033 | (1 << 7);
    let decoded = RiscVInstruction::decode(sub_x1_x2_x31);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn all_max_values() {
    let sub_x31_x31_x31 = 0x41ff8fb3;
    let decoded = RiscVInstruction::decode(sub_x31_x31_x31);

    match decoded {
        RiscVInstruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected SUB instruction"),
    }
}

#[test]
fn invalid_funct7_should_be_unsupported() {
    // SUB with invalid funct7 (0x00 instead of 0x20)
    let invalid_sub = 0x003100b3;
    let decoded = RiscVInstruction::decode(invalid_sub);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            // This should decode as ADD, not SUB
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected ADD instruction (not SUB)"),
    }
}
