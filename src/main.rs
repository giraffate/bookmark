use clap::{App, Arg};
use dirs;
use toml;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("bookmark")
        .subcommand(
            App::new("add")
                .about("Add bookmark")
                .arg(Arg::new("input").index(1).required(true)),
        )
        .subcommand(App::new("get").about("Get bookmark"))
        .get_matches();

    let home_dir = dirs::home_dir().unwrap();
    let path = home_dir.join(".bookmark.toml");
    let body = get_bookmark(path.clone());
    let mut map: HashMap<&str, &str> = toml::from_str(&body).unwrap();
    let current_dir = std::env::current_dir().unwrap();
    if let Some(ref matches) = matches.subcommand_matches("add") {
        let input = matches.value_of("input").unwrap();
        map.insert(current_dir.to_str().unwrap(), input);
        let toml = toml::to_string(&map).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        write!(file, "{}", toml)?;
        println!("succeeded");
    } else if let Some(_) = matches.subcommand_matches("get") {
        if let Some(value) = map.get(current_dir.to_str().unwrap()) {
            println!("{}", value);
        }
    }

    Ok(())
}

fn get_bookmark(path: PathBuf) -> String {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    body
}
