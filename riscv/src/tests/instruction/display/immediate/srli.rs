use crate::instruction::RiscVInstruction;

#[test]
fn positive_immediate() {
    let srli = RiscVInstruction::Srli {
        rd: 1,
        rs1: 2,
        imm: 5,
    };
    assert_eq!(format!("{}", srli), "srli x1, x2, 5");
}

#[test]
fn zero_immediate() {
    let srli = RiscVInstruction::Srli {
        rd: 31,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", srli), "srli x31, x0, 0");
}

#[test]
fn min_values() {
    let srli_min = RiscVInstruction::Srli {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", srli_min), "srli x0, x0, 0");
}

#[test]
fn max_values() {
    let srli_max = RiscVInstruction::Srli {
        rd: 31,
        rs1: 31,
        imm: 31,
    };
    assert_eq!(format!("{}", srli_max), "srli x31, x31, 31");
}
