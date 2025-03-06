use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Afshin Karimi afshinnkarimi@gmail.com")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .long("omit-newline")
                .help("Do not print newline")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    let text: Vec<String> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}