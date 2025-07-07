use crate::instruction::RiscVInstruction;

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
