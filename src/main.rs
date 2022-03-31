mod config;
mod utils;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use clap::{App, Arg};
use anyhow::Result;
use xdg::BaseDirectories;
use crate::config::{check_config, Config};

fn main() -> Result<()> {
    let matches = App::new("LeftWM Command")
        .author("BlackDragon2447 <blackdragon2447@e.email>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("a tool for managing your LeftWM config")
        .arg(
            Arg::with_name("New")
                .short("n")
                .long("new")
                .help("Generate a new config file"),
        )
        .arg(
            Arg::with_name("Editor")
                .short("e")
                .long("editor")
                .help("Open the current config file in the default editor (default)"),
        )
        .arg(
            Arg::with_name("TUI")
                .short("t")
                .long("tui")
                .help("Open the current config file in the TUI"),
        )
        .arg(
            Arg::with_name("Check")
                .short("c")
                .long("check")
                .help("Check if the current config is valid"),
        ).arg(
        Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Outputs received configuration file."),
    )
        .get_matches();

    let verbose = matches.occurrences_of("verbose") >= 1;

    if matches.is_present("New"){
        generate_new_config()?;
    } else if matches.is_present("Check") {
        check_config(verbose)?;
    }

    Ok(())
}

fn generate_new_config() -> Result<()> {
    let path = BaseDirectories::with_prefix("leftwm")?.place_config_file("config.toml")?;

    if Path::new(&path).exists() {
        println!(
            "\x1b[0;94m::\x1b[0m A config file already exists, do you want to override it? [y/N]"
        );
        let mut line = String::new();
        let _ = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.contains('y') || line.contains('Y') {
            let config = Config::default();
            let toml = toml::to_string(&config)?;
            let mut file = File::create(&path)?;
            file.write_all(toml.as_bytes())?;
        }
    }

    Ok(())
}
