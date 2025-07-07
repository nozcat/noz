use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let sltiu = RiscVInstruction::Sltiu {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", sltiu), "sltiu x1, x2, 100");
}

#[test]
fn negative_immediate() {
    let sltiu = RiscVInstruction::Sltiu {
        rd: 0,
        rs1: 1,
        imm: -4,
    };
    assert_eq!(format!("{}", sltiu), "sltiu x0, x1, -4");
}

#[test]
fn zero_immediate() {
    let sltiu = RiscVInstruction::Sltiu {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", sltiu), "sltiu x31, x0, 0");
}

#[test]
fn min_values() {
    let sltiu_min = RiscVInstruction::Sltiu {
        rd: 0,
        rs1: 0,
        imm: -2048,
    };
    assert_eq!(format!("{}", sltiu_min), "sltiu x0, x0, -2048");
}

#[test]
fn max_values() {
    let sltiu_max = RiscVInstruction::Sltiu {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert_eq!(format!("{}", sltiu_max), "sltiu x31, x31, 2047");
}
