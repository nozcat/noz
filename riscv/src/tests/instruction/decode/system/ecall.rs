use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let ecall = 0x00000073;
    let decoded = RiscVInstruction::decode(ecall);

    match decoded {
        RiscVInstruction::Ecall => {}
        _ => panic!("Expected ECALL instruction"),
    }
}

#[test]
fn with_correct_opcode_and_fields() {
    // ECALL: opcode=0x73, funct3=0x0, rd=0, rs1=0, imm=0
    let ecall = 0x00000073;
    let decoded = RiscVInstruction::decode(ecall);

    match decoded {
        RiscVInstruction::Ecall => {}
        _ => panic!("Expected ECALL instruction"),
    }
}

#[test]
fn with_non_zero_rd_should_be_unsupported() {
    // ECALL with rd=1 should be unsupported
    let invalid_ecall = 0x00000073 | (1 << 7);
    let decoded = RiscVInstruction::decode(invalid_ecall);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, invalid_ecall);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}

#[test]
fn with_non_zero_rs1_should_be_unsupported() {
    // ECALL with rs1=1 should be unsupported
    let invalid_ecall = 0x00000073 | (1 << 15);
    let decoded = RiscVInstruction::decode(invalid_ecall);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, invalid_ecall);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}
