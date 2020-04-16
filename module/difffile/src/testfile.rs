//all module root : crate
/// 独立的test文件，单元测试，需要在main.rs中声明测试模块
use crate::add::adder;

#[test]
fn test_adder() {
    assert_eq!(1, adder(2, -1));
    assert_ne!(0, adder(-1, 2));
}
