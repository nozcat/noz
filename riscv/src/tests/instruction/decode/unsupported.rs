use crate::instruction::RiscVInstruction;

#[test]
fn opcode() {
    let unsupported = 0x12345678;
    let decoded = RiscVInstruction::decode(unsupported);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x12345678);
        }
        _ => panic!("Expected unsupported instruction"),
    }
}

#[test]
fn slli_invalid_funct7() {
    let slli_with_invalid_funct7 = 0x02109093;
    let decoded = RiscVInstruction::decode(slli_with_invalid_funct7);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x02109093);
        }
        _ => panic!("Expected unsupported instruction for SLLI with invalid funct7"),
    }
}

#[test]
fn jalr_funct3() {
    let jalr_with_invalid_funct3 = 0x004110e7;
    let decoded = RiscVInstruction::decode(jalr_with_invalid_funct3);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x004110e7);
        }
        _ => panic!("Expected unsupported instruction for JALR with invalid funct3"),
    }
}

#[test]
fn load_invalid_funct3() {
    let load_with_invalid_funct3 = 0x0031b083;
    let decoded = RiscVInstruction::decode(load_with_invalid_funct3);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x0031b083);
        }
        _ => panic!("Expected unsupported instruction for LOAD with invalid funct3"),
    }
}

#[test]
fn system_invalid_imm() {
    // SYSTEM instruction with invalid imm value (neither 0 for ECALL nor 1 for EBREAK)
    let system_invalid_imm = 0x00200073; // imm=2
    let decoded = RiscVInstruction::decode(system_invalid_imm);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x00200073);
        }
        _ => panic!("Expected unsupported instruction for SYSTEM with invalid imm"),
    }
}

#[test]
fn system_invalid_funct3() {
    // SYSTEM instruction with invalid funct3 (not 0)
    let system_invalid_funct3 = 0x00001073; // funct3=1
    let decoded = RiscVInstruction::decode(system_invalid_funct3);

    match decoded {
        RiscVInstruction::Unsupported(word) => {
            assert_eq!(word, 0x00001073);
        }
        _ => panic!("Expected unsupported instruction for SYSTEM with invalid funct3"),
    }
}
