use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let slli = RiscVInstruction::Slli {
        rd: 1,
        rs1: 2,
        imm: 5,
    };
    assert_eq!(format!("{}", slli), "slli x1, x2, 5");
}

#[test]
fn zero_immediate() {
    let slli = RiscVInstruction::Slli {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", slli), "slli x31, x0, 0");
}

#[test]
fn min_values() {
    let slli_min = RiscVInstruction::Slli {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", slli_min), "slli x0, x0, 0");
}

#[test]
fn max_values() {
    let slli_max = RiscVInstruction::Slli {
        rd: 31,
        rs1: 31,
        imm: 31,
    };
    assert_eq!(format!("{}", slli_max), "slli x31, x31, 31");
}
