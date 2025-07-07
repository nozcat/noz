use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let jalr = RiscVInstruction::Jalr {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", jalr), "jalr x1, x2, 100");
}

#[test]
fn negative_immediate() {
    let jalr = RiscVInstruction::Jalr {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", jalr), "jalr x0, x1, -4");
}

#[test]
fn zero_immediate() {
    let jalr = RiscVInstruction::Jalr {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", jalr), "jalr x31, x0, 0");
}

#[test]
fn min_values() {
    let jalr_min = RiscVInstruction::Jalr {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", jalr_min), "jalr x0, x0, -2048");
}

#[test]
fn max_values() {
    let jalr_max = RiscVInstruction::Jalr {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", jalr_max), "jalr x31, x31, 2047");
}
