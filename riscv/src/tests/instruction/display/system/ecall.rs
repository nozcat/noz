use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let ecall = RiscVInstruction::Ecall;
    assert_eq!(format!("{}", ecall), "ecall");
}
