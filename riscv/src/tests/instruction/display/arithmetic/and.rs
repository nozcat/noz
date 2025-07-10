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
fn min_values() {
    let and_min = RiscVInstruction::And {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", and_min), "and x0, x0, x0");
}

#[test]
fn max_values() {
    let and_max = RiscVInstruction::And {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", and_max), "and x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let and_mixed = RiscVInstruction::And {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", and_mixed), "and x5, x10, x15");
}

#[test]
fn same_registers() {
    let and_same = RiscVInstruction::And {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    assert_eq!(format!("{}", and_same), "and x7, x7, x7");
}
