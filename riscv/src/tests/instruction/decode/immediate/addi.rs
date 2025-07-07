use crate::instruction::RiscVInstruction;

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
