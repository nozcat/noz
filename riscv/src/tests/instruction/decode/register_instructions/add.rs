use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let add_x1_x2_x3 = 0x003100b3;
    let decoded = RiscVInstruction::decode(add_x1_x2_x3);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn min_rd() {
    let add_x0_x1_x2 = 0x00208033;
    let decoded = RiscVInstruction::decode(add_x0_x1_x2);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn max_rd() {
    let add_x31_x1_x2 = 0x00208033 | (31 << 7);
    let decoded = RiscVInstruction::decode(add_x31_x1_x2);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn min_rs1() {
    let add_x1_x0_x2 = 0x00200033 | (1 << 7);
    let decoded = RiscVInstruction::decode(add_x1_x0_x2);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn max_rs1() {
    let add_x1_x31_x2 = 0x002f8033 | (1 << 7);
    let decoded = RiscVInstruction::decode(add_x1_x31_x2);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 2);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn min_rs2() {
    let add_x1_x2_x0 = 0x00010033 | (1 << 7);
    let decoded = RiscVInstruction::decode(add_x1_x2_x0);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn max_rs2() {
    let add_x1_x2_x31 = 0x01f10033 | (1 << 7);
    let decoded = RiscVInstruction::decode(add_x1_x2_x31);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn all_max_values() {
    let add_x31_x31_x31 = 0x01ff8fb3;
    let decoded = RiscVInstruction::decode(add_x31_x31_x31);

    match decoded {
        RiscVInstruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected ADD instruction"),
    }
}

#[test]
fn invalid_funct7_should_be_unsupported() {
    // ADD with invalid funct7 (0x20 instead of 0x00)
    let invalid_add = 0x203100b3;
    let decoded = RiscVInstruction::decode(invalid_add);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x203100b3);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}
