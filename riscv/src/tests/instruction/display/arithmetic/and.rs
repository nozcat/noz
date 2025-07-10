use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let and = RiscVInstruction::And {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", and), "and x1, x2, x3");
}

#[test]
fn min_registers() {
    let and = RiscVInstruction::And {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", and), "and x0, x0, x0");
}

#[test]
fn max_registers() {
    let and = RiscVInstruction::And {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", and), "and x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let and = RiscVInstruction::And {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", and), "and x5, x10, x15");
}
