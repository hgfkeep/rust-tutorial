#[macro_export]
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
    // 重载宏定义
    ($a:expr) => {
        $a + 1
    };
}

#[macro_export]
macro_rules! ident_example {
    ($name:ident) => {
        fn $name() {
            /// 不能直接使用$name， 此时的$name不是expr，是ident，无法直接打印到标准输出
            /// 使用stringifi! 将ident 转换为字符串
            println!("called function: {}()", stringify!($name));
        }
    };
}


#[macro_export]
macro_rules! helped {
    () => { helper!() } // 这可能会有问题，'helper'宏 还不在作用域.引用了当前crate 后面作用域定义的宏
    // () => { $crate::helper!() }
}

#[macro_export]
macro_rules! helper {
    () => { () }
}

#[macro_export]
// 仅在本some() 合法的作用范围 调用 unpub_fn_call， 因为some()的作用域，限制了unpub_fn_call作用域。
macro_rules! unpub_fn_call {
    () => {
        
        $crate::some()
    };
}

fn some(){
    println!("macro called unpub some()", );
}


#[test]
fn unpub() {
    unpub_fn_call!();
}