use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let lb = RiscVInstruction::Lb {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", lb), "lb x1, 100(x2)");
}

#[test]
fn negative_immediate() {
    let lb = RiscVInstruction::Lb {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", lb), "lb x0, -4(x1)");
}

#[test]
fn zero_immediate() {
    let lb = RiscVInstruction::Lb {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", lb), "lb x31, 0(x0)");
}

#[test]
fn min_values() {
    let lb_min = RiscVInstruction::Lb {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", lb_min), "lb x0, -2048(x0)");
}

#[test]
fn max_values() {
    let lb_max = RiscVInstruction::Lb {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", lb_max), "lb x31, 2047(x31)");
}
