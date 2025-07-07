use crate::instruction::RiscVInstruction;

#[test]
fn unsupported() {
    let unsupported = RiscVInstruction::Unsupported(0x12345678);
    assert_eq!(format!("{}", unsupported), "unsupported(0x12345678)");

    let unsupported_zero = RiscVInstruction::Unsupported(0x00000000);
    assert_eq!(format!("{}", unsupported_zero), "unsupported(0x00000000)");

    let unsupported_max = RiscVInstruction::Unsupported(0xffffffff);
    assert_eq!(format!("{}", unsupported_max), "unsupported(0xffffffff)");
}
