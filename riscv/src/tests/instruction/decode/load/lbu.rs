use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let lbu_x1_x2_100 = 0x06414083;
    let decoded = RiscVInstruction::decode(lbu_x1_x2_100);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn min_rd() {
    let lbu_x0_x1_0 = 0x0000c003;
    let decoded = RiscVInstruction::decode(lbu_x0_x1_0);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn max_rd() {
    let lbu_x31_x1_0 = 0x0000c003 | (31 << 7);
    let decoded = RiscVInstruction::decode(lbu_x31_x1_0);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn min_rs1() {
    let lbu_x1_x0_0 = 0x00004083;
    let decoded = RiscVInstruction::decode(lbu_x1_x0_0);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn max_rs1() {
    let lbu_x1_x31_0 = 0x0000c083 | (31 << 15);
    let decoded = RiscVInstruction::decode(lbu_x1_x31_0);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn negative_imm() {
    let lbu_x1_x2_neg4 = 0xffc14083;
    let decoded = RiscVInstruction::decode(lbu_x1_x2_neg4);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -4);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn zero_imm() {
    let lbu_x1_x2_0 = 0x00014083;
    let decoded = RiscVInstruction::decode(lbu_x1_x2_0);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn max_positive_imm() {
    let lbu_x1_x0_2047 = 0x7ff04083;
    let decoded = RiscVInstruction::decode(lbu_x1_x0_2047);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected LBU instruction"),
    }
}

#[test]
fn min_negative_imm() {
    let lbu_x1_x0_neg2048 = 0x80004083;
    let decoded = RiscVInstruction::decode(lbu_x1_x0_neg2048);

    match decoded {
        RiscVInstruction::Lbu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected LBU instruction"),
    }
}
