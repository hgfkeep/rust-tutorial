pub mod add {
    pub fn adder(a: i32, b: i32) -> i32 {
        a + b
    }

    //internal_adder 无法在模块外部被访问
    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
}

///单元测试
#[cfg(test)]
pub mod test{
    //all module root : crate
     use super::add::*;

    #[test]
    fn test_adder(){
        assert_eq!(1, adder(2, -1));
        assert_ne!(0, adder(-1, 2));
    }
}

//单元测试独立到一个文件中
#[cfg(test)]
mod testfile;

#[cfg(test)]
#[path="../tests/test.rs"]
mod test_inte;

fn main() {
    use add::adder;
    println!("adder: {}", adder(1, 5));
}
