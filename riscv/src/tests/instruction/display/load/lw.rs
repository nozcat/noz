use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let lw = RiscVInstruction::Lw {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", lw), "lw x1, 100(x2)");
}

#[test]
fn negative_immediate() {
    let lw = RiscVInstruction::Lw {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", lw), "lw x0, -4(x1)");
}

#[test]
fn zero_immediate() {
    let lw = RiscVInstruction::Lw {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", lw), "lw x31, 0(x0)");
}

#[test]
fn min_values() {
    let lw_min = RiscVInstruction::Lw {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", lw_min), "lw x0, -2048(x0)");
}

#[test]
fn max_values() {
    let lw_max = RiscVInstruction::Lw {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", lw_max), "lw x31, 2047(x31)");
}
