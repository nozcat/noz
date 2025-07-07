use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let or = RiscVInstruction::Or {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", or), "or x1, x2, x3");
}

#[test]
fn min_values() {
    let or_min = RiscVInstruction::Or {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", or_min), "or x0, x0, x0");
}

#[test]
fn max_values() {
    let or_max = RiscVInstruction::Or {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", or_max), "or x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let or_mixed = RiscVInstruction::Or {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", or_mixed), "or x5, x10, x15");
}

#[test]
fn same_registers() {
    let or_same = RiscVInstruction::Or {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    assert_eq!(format!("{}", or_same), "or x7, x7, x7");
}
