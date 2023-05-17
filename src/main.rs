use clap::*;
use open;

fn main() {

    let my_arguments = Command::new("google")
        .arg(Arg::new("query").help("A query to Google Search").required(true))
        .get_matches();

    let mut query: String = my_arguments.get_one::<String>("query").expect("Could not parse argument").to_owned();

    let url: String = format!("https://google.com/search?q={}", query);

    println!("Searching Google For: {}", query);

    open::that(url).expect("Failed to open url")
}
