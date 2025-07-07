use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let sub = RiscVInstruction::Sub {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", sub), "sub x1, x2, x3");
}

#[test]
fn min_values() {
    let sub_min = RiscVInstruction::Sub {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", sub_min), "sub x0, x0, x0");
}

#[test]
fn max_values() {
    let sub_max = RiscVInstruction::Sub {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", sub_max), "sub x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let sub_mixed = RiscVInstruction::Sub {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", sub_mixed), "sub x5, x10, x15");
}

#[test]
fn same_registers() {
    let sub_same = RiscVInstruction::Sub {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    assert_eq!(format!("{}", sub_same), "sub x7, x7, x7");
}
