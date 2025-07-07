use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let andi = RiscVInstruction::Andi {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", andi), "andi x1, x2, 100");
}

#[test]
fn negative_immediate() {
    let andi = RiscVInstruction::Andi {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", andi), "andi x0, x1, -4");
}

#[test]
fn zero_immediate() {
    let andi = RiscVInstruction::Andi {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", andi), "andi x31, x0, 0");
}

#[test]
fn min_values() {
    let andi_min = RiscVInstruction::Andi {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", andi_min), "andi x0, x0, -2048");
}

#[test]
fn max_values() {
    let andi_max = RiscVInstruction::Andi {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", andi_max), "andi x31, x31, 2047");
}
