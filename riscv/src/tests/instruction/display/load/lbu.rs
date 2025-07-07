use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let lbu = RiscVInstruction::Lbu {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", lbu), "lbu x1, 100(x2)");
}

#[test]
fn negative_immediate() {
    let lbu = RiscVInstruction::Lbu {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", lbu), "lbu x0, -4(x1)");
}

#[test]
fn zero_immediate() {
    let lbu = RiscVInstruction::Lbu {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", lbu), "lbu x31, 0(x0)");
}

#[test]
fn min_values() {
    let lbu_min = RiscVInstruction::Lbu {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", lbu_min), "lbu x0, -2048(x0)");
}

#[test]
fn max_values() {
    let lbu_max = RiscVInstruction::Lbu {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", lbu_max), "lbu x31, 2047(x31)");
}
