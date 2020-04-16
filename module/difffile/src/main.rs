/// 下级模块需要在入口文件声明，
/// 会自动查找main.rs所在的目录中的add.rs
/// 或 add文件夹中包含mod.rs
mod add;

/// 声明测试模块， 则可以在main.rs 同意文件夹下创建独立的测试文件
#[cfg(test)]
mod testfile;

fn main() {
    println!("res = {}", add::adder(1,5));
}
