use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let slti_x1_x2_100 = 0x06412093;
    let decoded = RiscVInstruction::decode(slti_x1_x2_100);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn min_rd() {
    let slti_x0_x1_0 = 0x0000a013;
    let decoded = RiscVInstruction::decode(slti_x0_x1_0);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn max_rd() {
    let slti_x31_x1_0 = 0x0000a013 | (31 << 7);
    let decoded = RiscVInstruction::decode(slti_x31_x1_0);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn min_rs1() {
    let slti_x1_x0_0 = 0x00002093;
    let decoded = RiscVInstruction::decode(slti_x1_x0_0);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn max_rs1() {
    let slti_x1_x31_0 = 0x000fa093;
    let decoded = RiscVInstruction::decode(slti_x1_x31_0);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn negative_imm() {
    let slti_x0_x1_neg4 = 0xffc0a013;
    let decoded = RiscVInstruction::decode(slti_x0_x1_neg4);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, -4);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn zero_imm() {
    let slti_x1_x2_0 = 0x00012093;
    let decoded = RiscVInstruction::decode(slti_x1_x2_0);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn max_positive_imm() {
    let slti_x1_x0_2047 = 0x7ff02093;
    let decoded = RiscVInstruction::decode(slti_x1_x0_2047);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}

#[test]
fn min_negative_imm() {
    let slti_x1_x0_neg2048 = 0x80002093;
    let decoded = RiscVInstruction::decode(slti_x1_x0_neg2048);

    match decoded {
        RiscVInstruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected SLTI instruction"),
    }
}
