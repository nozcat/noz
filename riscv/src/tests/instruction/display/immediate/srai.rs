use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let srai = RiscVInstruction::Srai {
        rd: 1,
        rs1: 2,
        imm: 5,
    };
    assert_eq!(format!("{}", srai), "srai x1, x2, 5");
}

#[test]
fn zero_immediate() {
    let srai = RiscVInstruction::Srai {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", srai), "srai x31, x0, 0");
}

#[test]
fn min_values() {
    let srai_min = RiscVInstruction::Srai {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", srai_min), "srai x0, x0, 0");
}

#[test]
fn max_values() {
    let srai_max = RiscVInstruction::Srai {
        rd: 31,
        rs1: 31,
        imm: 31,
    };
    assert_eq!(format!("{}", srai_max), "srai x31, x31, 31");
}
