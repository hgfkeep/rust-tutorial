
pub fn adder(a: i32, b: i32) -> i32 {
    a + b
}

///希望生成add模块独有的测试文件，
/// 那么必须在add文件夹中，添加模块说明和模块文件
#[cfg(test)]
mod testfile;

#[cfg(test)]
pub mod test{
    //all module root : crate
     use super::*;

    #[test]
    fn test_adder(){
        assert_eq!(1, adder(2, -1));
        assert_ne!(0, adder(-1, 2));
    }
}