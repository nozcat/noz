use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let xor_x1_x2_x3 = 0x003140b3;
    let decoded = RiscVInstruction::decode(xor_x1_x2_x3);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn min_rd() {
    let xor_x0_x1_x2 = 0x0020c033;
    let decoded = RiscVInstruction::decode(xor_x0_x1_x2);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn max_rd() {
    let xor_x31_x1_x2 = 0x0020c033 | (31 << 7);
    let decoded = RiscVInstruction::decode(xor_x31_x1_x2);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn min_rs1() {
    let xor_x1_x0_x2 = 0x00204033 | (1 << 7);
    let decoded = RiscVInstruction::decode(xor_x1_x0_x2);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn max_rs1() {
    let xor_x1_x31_x2 = 0x002fc033 | (1 << 7);
    let decoded = RiscVInstruction::decode(xor_x1_x31_x2);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn min_rs2() {
    let xor_x1_x2_x0 = 0x00014033 | (1 << 7);
    let decoded = RiscVInstruction::decode(xor_x1_x2_x0);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn max_rs2() {
    let xor_x1_x2_x31 = 0x01f14033 | (1 << 7);
    let decoded = RiscVInstruction::decode(xor_x1_x2_x31);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn all_max_values() {
    let xor_x31_x31_x31 = 0x01ffcfb3;
    let decoded = RiscVInstruction::decode(xor_x31_x31_x31);

    match decoded {
        RiscVInstruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected XOR instruction"),
    }
}

#[test]
fn invalid_funct7_should_be_unsupported() {
    // XOR with invalid funct7 (0x20 instead of 0x00)
    let invalid_xor = 0x203140b3;
    let decoded = RiscVInstruction::decode(invalid_xor);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x203140b3);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}
