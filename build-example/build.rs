use std::env;
use std::fs::File;
use std::io::Write; // file.write_all 需要使用次trait
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn main() {
    let out_path: String = env::var("OUT_DIR").unwrap();
    println!("{:?}", out_path);
    let out_file: PathBuf = Path::new(&out_path).join("commit_hash.rs");
    let mut f: File = File::create(out_file).unwrap();

    let command_res: Output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output() // 子进程执行命令，并将命令的输出信息返回， 返回类型为Result<Output>
        .expect("Faild execut git command"); //Unwrap reault 类型，如果unwrap异常，输出参数信息

    //将命令行输出转为字符串
    let commit = String::from_utf8(command_res.stdout)
    .expect("Invalid utf8 string");

    //生成源代码字符串
    let output = format!(r#" pub const CURRENT_COMMIT_ID: &'static str = "{}";"#, commit);
    f.write_all(output.as_bytes()).unwrap(); //将生成的源代码写入到文件
}