/// testfile.rs在add模块中，
///  adder方法和本测试文件共属于同一个模块，可以使用super 导入
use super::*;

#[test]
fn test_adder() {
    assert_eq!(1, adder(2, -1));
    assert_ne!(0, adder(-1, 2));
}
