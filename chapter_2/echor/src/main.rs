use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("nyi nyi")
        .about("echo")
        .arg(
            Arg::new("omit newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("text")
                .help("Input text")
                .required(true)
                .action(ArgAction::Append),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    let endl = if matches.get_flag("omit newline") {
        ""
    } else {
        "\n"
    };

    print!("{text}{endl}");
}
