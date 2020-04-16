use clap::{App, Arg, SubCommand};

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);
    let matches = App::new("demo-app")
        .version("1.0.0")
        .author("heguangfu")
        .about("demo app about clap")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"),
        )
        .args_from_usage("-p, --path=[FILE] 'Target file you want to change'")
        .subcommand(
            SubCommand::with_name("test")
                .about("do test things")
                .arg_from_usage("-l, --list 'list test values'"),
        )
        .get_matches();

    if let Some(f) = matches.value_of("path") {
        println!("path={}", f);
    }

    if let Some(m) = matches.subcommand_matches("test") {
        if m.is_present("list") {
            println!("print test list");
        } else {
            println!("not print test list");
        }
    }
}
