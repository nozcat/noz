use crate::instruction::RiscVInstruction;

#[test]
fn basic() {
    let ebreak = RiscVInstruction::Ebreak;
    assert_eq!(format!("{}", ebreak), "ebreak");
}
