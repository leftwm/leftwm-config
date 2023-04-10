// during dev
#![allow(dead_code)]

extern crate core;

mod config;
mod tui;
mod utils;

use crate::config::check_config;
use crate::config::filehandler::{load_from_file, write_to_file};
use anyhow::Result;
use clap::{App, Arg};
use std::env;
use std::path::Path;
use std::process::Command;
use xdg::BaseDirectories;

#[cfg(debug_assertions)]
const CONFIG_NAME: &str = "test_config";
#[cfg(not(debug_assertions))]
const CONFIG_NAME: &str = "config";

fn main() -> Result<()> {
    let matches = App::new("LeftWM Command")
        .author("BlackDragon2447 <blackdragon2447@e.email>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("a tool for managing your LeftWM config")
        .arg(
            Arg::with_name("New")
                .short('n')
                .long("new")
                .help("Generate a new config file"),
        )
        .arg(
            Arg::with_name("Editor")
                .short('e')
                .long("editor")
                .help("Open the current config file in the default editor (default)"),
        )
        .arg(
            Arg::with_name("TUI")
                .short('t')
                .long("tui")
                .help("Open the current config file in the TUI"),
        )
        .arg(
            Arg::with_name("Check")
                .short('c')
                .long("check")
                .help("Check if the current config is valid"),
        )
        .arg(
            Arg::with_name("Verbose")
                .short('v')
                .long("verbose")
                .help("Outputs received configuration file."),
        )
        .arg(
            Arg::with_name("Migrate")
                .long("migrate")
                .help("Migrate an old .toml config to the RON format."),
        )
        .get_matches();

    let verbose = matches.occurrences_of("Verbose") >= 1;

    if matches.is_present("Migrate") {
        println!("\x1b[0;94m::\x1b[0m Migrating configuration . . .");
        let path = BaseDirectories::with_prefix("leftwm")?;
        let ron_file = path.place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?;
        let toml_file = path.place_config_file(crate::CONFIG_NAME.to_string() + ".toml")?;

        let config = load_from_file(toml_file.as_os_str().to_str(), verbose)?;

        write_to_file(&ron_file, &config)?;

        return Ok(());
    } else if matches.is_present("Editor") {
        run_editor(config::filehandler::get_config_file()?.as_path())?;
    } else if matches.is_present("TUI") {
        crate::tui::run()?;
    } else if matches.is_present("New") {
        config::filehandler::generate_new_config()?;
    } else if matches.is_present("Check") {
        check_config(verbose)?;
    } else {
        run_editor(config::filehandler::get_config_file()?.as_path())?;
    }

    Ok(())
}

fn run_editor(file: &Path) -> Result<()> {
    let editor = env::var("EDITOR")?;

    let mut process = Command::new(&editor).arg(file.as_os_str()).spawn()?;
    if process.wait()?.success() {
        Ok(())
    } else {
        Err(anyhow::Error::msg(format!("Failed to run {}", &editor)))
    }?;

    Ok(())
}
