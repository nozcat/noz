use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let xor = RiscVInstruction::Xor {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", xor), "xor x1, x2, x3");
}

#[test]
fn min_values() {
    let xor_min = RiscVInstruction::Xor {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", xor_min), "xor x0, x0, x0");
}

#[test]
fn max_values() {
    let xor_max = RiscVInstruction::Xor {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", xor_max), "xor x31, x31, x31");
}

#[test]
fn mixed_registers() {
    let xor_mixed = RiscVInstruction::Xor {
        rd: 5,
        rs1: 10,
        rs2: 15,
    };
    assert_eq!(format!("{}", xor_mixed), "xor x5, x10, x15");
}

#[test]
fn same_registers() {
    let xor_same = RiscVInstruction::Xor {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    assert_eq!(format!("{}", xor_same), "xor x7, x7, x7");
}
