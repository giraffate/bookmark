use clap::{App, Arg};

fn main() {
    let matches = App::new("bookmark")
        .subcommand(
            App::new("add")
                .about("Add bookmark")
                .arg(Arg::new("input").index(1).required(true)),
        )
        .subcommand(App::new("get").about("Get bookmark"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("add") {
        println!("add: {}", matches.value_of("input").unwrap());
    } else if let Some(_) = matches.subcommand_matches("get") {
        println!("get");
    }
}
