use clap::{App, Arg};
use dirs;
use toml;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("bookmark")
        .subcommand(
            App::new("add")
                .about("Add bookmark")
                .arg(Arg::new("input").index(1).required(true)),
        )
        .subcommand(App::new("get").about("Get bookmark"))
        .get_matches();

    let mut file = get_bookmark_file();
    let mut body = String::new();
    file.read_to_string(&mut body).unwrap();
    let mut map: HashMap<&str, &str> = toml::from_str(&body).unwrap();

    let current_dir = std::env::current_dir().unwrap();
    if let Some(ref matches) = matches.subcommand_matches("add") {
        let input = matches.value_of("input").unwrap();
        map.insert(current_dir.to_str().unwrap(), input);
        let toml = toml::to_string(&map).unwrap();
        write!(file, "{}", toml)?;
        println!("succeeded");
    } else if let Some(_) = matches.subcommand_matches("get") {
        if let Some(value) = map.get(current_dir.to_str().unwrap()) {
            println!("{}", value);
        }
    }

    Ok(())
}

fn get_bookmark_file() -> File {
    let home_dir = dirs::home_dir().unwrap();
    let target_file = home_dir.join(".bookmark.toml");
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(target_file)
        .unwrap()
}
