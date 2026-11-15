use crate::vm::Vm;

#[test]
fn test_halt() {
    let mut vm = Vm::new(b"41C5").unwrap();

    assert!(vm.run().is_ok());
}

#[test]
fn test_while_loop() {
    let mut vm = Vm::new(b"%++%++[%--]41C5").unwrap();

    assert!(vm.run().is_ok());
}

#[test]
fn test_invalid_instruction() {
    let mut vm = Vm::new(b"INVALID").unwrap();

    assert!(vm.run().is_err());
}
