use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let lh = RiscVInstruction::Lh {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", lh), "lh x1, 100(x2)");
}

#[test]
fn negative_immediate() {
    let lh = RiscVInstruction::Lh {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", lh), "lh x0, -4(x1)");
}

#[test]
fn zero_immediate() {
    let lh = RiscVInstruction::Lh {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", lh), "lh x31, 0(x0)");
}

#[test]
fn min_values() {
    let lh_min = RiscVInstruction::Lh {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", lh_min), "lh x0, -2048(x0)");
}

#[test]
fn max_values() {
    let lh_max = RiscVInstruction::Lh {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", lh_max), "lh x31, 2047(x31)");
}
