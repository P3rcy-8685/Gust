use clap::{App, Arg};

fn main() {
    let matches = App::new("Your CLI App")
        .arg(
            Arg::with_name("command")
                .required(true)
                .takes_value(true)
                .index(1),
        )

        .arg(
            Arg::with_name("directory")
                .takes_value(true)
                .required(false)
                .index(2)
                .default_value("."),
        )
        .get_matches();

    match matches.value_of("command") {
        Some("init") => cmd_init(matches),
        _ => eprintln!("Invalid command"),
    }
}

fn cmd_init(matches: clap::ArgMatches) {
    if let Some(directory) = matches.value_of("directory") {
        println!("Running 'init' command with directory: {}", directory);
    }
}



