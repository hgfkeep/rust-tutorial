#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0.0")
        (author: "heguangfu")
        (about: "demo app about clap")
        (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
        (@arg debug: -d ... "Set a level of debug info")
        (@subcommand test =>
            (about: "control testing feature")
            (@arg verbose: -V --version "print test information verbosely")
        )
    )
    .get_matches();

    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("config={}", config);

    if let Some(m) = matches.subcommand_matches("test") {
        if m.is_present("verbose") {
            println!("print verbosely");
        } else {
            println!("not print verbosely");
        }
    }
}
