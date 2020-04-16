use clap::Clap;

/// 结构体的注释会作为about出现在程序描述信息；
/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K.")]
struct Opts {
    /// 设置参数， 注释会作为参数的说明信息，会自动换行
    /// Sets a custom config file. 可以设置为 Option<T> 类型，这样可以没有默认配置
    #[clap(short = "c", long = "config", default_value = "default.conf")]
    config: String,
    /// 不带有#[clap()]属性说明的，作为输入字段，字段类型非 Option<T>，为必须字段。
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times， 通过parse 获取和解析参数值
    #[clap(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,

    ///设置了子命令
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap)]
enum SubCommand {
    /// A help message for the Test subcommand
    #[clap(name = "test", version = "1.3", author = "Someone Else")]
    Test(Test),
}

/// A subcommand for controlling testing
#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short = "d")]
    debug: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Value for config: {}", opts.config);
    println!("Using input file: {}", opts.input);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        SubCommand::Test(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }
    }

    // more program logic goes here...
}