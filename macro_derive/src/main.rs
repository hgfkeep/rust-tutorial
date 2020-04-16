// 使用扩展编译器的宏实现
#[macro_use]
extern crate hello_world_macro_derive;

// 声明trait
trait HelloWorld{
  fn hello();
}

// 使用derive 实现HelloWorld trait
#[derive(HelloWorld)]
struct FrenchToast;

fn main() {
  FrenchToast::hello();
}
