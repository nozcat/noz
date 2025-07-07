use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let ori = RiscVInstruction::Ori {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", ori), "ori x1, x2, 100");
}

#[test]
fn negative_immediate() {
    let ori = RiscVInstruction::Ori {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", ori), "ori x0, x1, -4");
}

#[test]
fn zero_immediate() {
    let ori = RiscVInstruction::Ori {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", ori), "ori x31, x0, 0");
}

#[test]
fn min_values() {
    let ori_min = RiscVInstruction::Ori {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", ori_min), "ori x0, x0, -2048");
}

#[test]
fn max_values() {
    let ori_max = RiscVInstruction::Ori {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", ori_max), "ori x31, x31, 2047");
}
