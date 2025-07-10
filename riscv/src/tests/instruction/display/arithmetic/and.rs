use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let and_instruction = RiscVInstruction::And {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    let display = format!("{}", and_instruction);
    assert_eq!(display, "and x1, x2, x3");
}

#[test]
fn min_values() {
    let and_instruction = RiscVInstruction::And {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    let display = format!("{}", and_instruction);
    assert_eq!(display, "and x0, x0, x0");
}

#[test]
fn max_values() {
    let and_instruction = RiscVInstruction::And {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    let display = format!("{}", and_instruction);
    assert_eq!(display, "and x31, x31, x31");
}

#[test]
fn mixed_values() {
    let and_instruction = RiscVInstruction::And {
        rd: 10,
        rs1: 5,
        rs2: 20,
    };
    let display = format!("{}", and_instruction);
    assert_eq!(display, "and x10, x5, x20");
}

#[test]
fn same_registers() {
    let and_instruction = RiscVInstruction::And {
        rd: 7,
        rs1: 7,
        rs2: 7,
    };
    let display = format!("{}", and_instruction);
    assert_eq!(display, "and x7, x7, x7");
}
