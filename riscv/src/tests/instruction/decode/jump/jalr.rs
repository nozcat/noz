use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let jalr_x1_x2_4 = 0x004100e7;
    let decoded = RiscVInstruction::decode(jalr_x1_x2_4);

    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 4);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn negative_imm() {
    let jalr_x0_x1_neg4 = 0xffc08067;
    let decoded = RiscVInstruction::decode(jalr_x0_x1_neg4);

    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, -4);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn min_rd() {
    let jalr_x0 = 0x00008067;
    let decoded = RiscVInstruction::decode(jalr_x0);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn max_rd() {
    let jalr_x31 = 0x000080e7 | (31 << 7);
    let decoded = RiscVInstruction::decode(jalr_x31);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn min_rs1() {
    let jalr_rs1_0 = 0x00000067;
    let decoded = RiscVInstruction::decode(jalr_rs1_0);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn max_rs1() {
    let jalr_rs1_31 = 0x000f8067;
    let decoded = RiscVInstruction::decode(jalr_rs1_31);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn zero_imm() {
    let jalr_imm_0 = 0x00008067;
    let decoded = RiscVInstruction::decode(jalr_imm_0);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn max_positive_imm() {
    let jalr_imm_2047 = 0x7ff08067;
    let decoded = RiscVInstruction::decode(jalr_imm_2047);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn min_negative_imm() {
    let jalr_imm_neg2048 = 0x80008067;
    let decoded = RiscVInstruction::decode(jalr_imm_neg2048);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected JALR instruction"),
    }
}

#[test]
fn neg_one_imm() {
    let jalr_imm_neg1 = 0xfff08067;
    let decoded = RiscVInstruction::decode(jalr_imm_neg1);
    match decoded {
        RiscVInstruction::Jalr { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, -1);
        }
        _ => panic!("Expected JALR instruction"),
    }
}
