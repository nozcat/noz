use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let srli_x1_x2_5 = 0x00515093;
    let decoded = RiscVInstruction::decode(srli_x1_x2_5);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 5);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn min_rd() {
    let srli_x0_x1_1 = 0x0010d013;
    let decoded = RiscVInstruction::decode(srli_x0_x1_1);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 1);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn max_rd() {
    let srli_x31_x1_1 = 0x0010d013 | (31 << 7);
    let decoded = RiscVInstruction::decode(srli_x31_x1_1);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 1);
            assert_eq!(imm, 1);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn min_rs1() {
    let srli_x1_x0_1 = 0x00105093;
    let decoded = RiscVInstruction::decode(srli_x1_x0_1);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 1);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn max_rs1() {
    let srli_x1_x31_1 = 0x001fd093;
    let decoded = RiscVInstruction::decode(srli_x1_x31_1);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 1);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn zero_imm() {
    let srli_x1_x2_0 = 0x00015093;
    let decoded = RiscVInstruction::decode(srli_x1_x2_0);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn max_shift_amount() {
    let srli_x1_x2_31 = 0x01f15093;
    let decoded = RiscVInstruction::decode(srli_x1_x2_31);

    match decoded {
        RiscVInstruction::Srli { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 31);
        }
        _ => panic!("Expected SRLI instruction"),
    }
}

#[test]
fn invalid_funct7() {
    let srli_with_invalid_funct7 = 0x02105093;
    let decoded = RiscVInstruction::decode(srli_with_invalid_funct7);

    match decoded {
        RiscVInstruction::Unsupported(_) => {
            // This is expected
        }
        _ => panic!("Expected unsupported instruction for SRLI with invalid funct7"),
    }
}
