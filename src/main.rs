use clap::*;
use colored::Colorize;
use open;

fn main() {
    let rainbow_google_logo: String = format!(
        "{}{}{}{}{}{}",
        "G".bold().blue(),
        "o".bold().red(),
        "o".bold().yellow(),
        "g".bold().blue(),
        "l".bold().green(),
        "e".bold().red()
    );

    let my_arguments = Command::new("google")
        .arg(
            Arg::new("query")
                .help("A query to Google Search")
                .required(true),
        )
        .get_matches();

    let query: String = my_arguments
        .get_one::<String>("query")
        .expect("Could not parse argument")
        .to_owned();

    let url: String = format!("https://google.com/search?q={}", query);

    println!("Searching {} For: {}", rainbow_google_logo, query);

    open::that(url).expect("Failed to open url")
}
