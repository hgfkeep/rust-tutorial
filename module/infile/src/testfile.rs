use crate::add::adder;

#[test]
fn test_adder() {
    assert_eq!(1, adder(2, -1));
    assert_ne!(0, adder(-1, 2));
}
