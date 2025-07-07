use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let add = RiscVInstruction::Add {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", add), "add x1, x2, x3");
}

#[test]
fn min_values() {
    let add_min = RiscVInstruction::Add {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", add_min), "add x0, x0, x0");
}

#[test]
fn max_values() {
    let add_max = RiscVInstruction::Add {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", add_max), "add x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let add_mixed = RiscVInstruction::Add {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", add_mixed), "add x5, x10, x15");
}

#[test]
fn same_registers() {
    let add_same = RiscVInstruction::Add {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    assert_eq!(format!("{}", add_same), "add x7, x7, x7");
}
