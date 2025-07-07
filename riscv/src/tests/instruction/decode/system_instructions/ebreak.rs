
use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let ebreak = 0x00100073;
    let decoded = RiscVInstruction::decode(ebreak);

    match decoded {
        RiscVInstruction::Ebreak => {}
        _ => panic!("Expected EBREAK instruction"),
    }
}

#[test]
fn with_correct_opcode_and_fields() {
    // EBREAK: opcode=0x73, funct3=0x0, rd=0, rs1=0, imm=1
    let ebreak = 0x00100073;
    let decoded = RiscVInstruction::decode(ebreak);

    match decoded {
        RiscVInstruction::Ebreak => {}
        _ => panic!("Expected EBREAK instruction"),
    }
}

#[test]
fn with_non_zero_rd_should_be_unsupported() {
    // EBREAK with rd=1 should be unsupported
    let invalid_ebreak = 0x00100073 | (1 << 7);
    let decoded = RiscVInstruction::decode(invalid_ebreak);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, invalid_ebreak);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}

#[test]
fn with_non_zero_rs1_should_be_unsupported() {
    // EBREAK with rs1=1 should be unsupported
    let invalid_ebreak = 0x00100073 | (1 << 15);
    let decoded = RiscVInstruction::decode(invalid_ebreak);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, invalid_ebreak);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}
