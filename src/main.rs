#![allow(
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    //a lot of unimplemented arms
    clippy::match_same_arms,
)]

extern crate core;

mod config;
mod tui;
mod utils;

use crate::config::check_config;
use crate::config::filehandler::{load_from_file, write_to_file};
use anyhow::Result;
use clap::{Arg, Command as ClapCmd};
use std::path::Path;
use std::process::Command;
use std::{env, fs, io};
use xdg::BaseDirectories;

#[cfg(debug_assertions)]
const CONFIG_NAME: &str = "test_config";
#[cfg(not(debug_assertions))]
const CONFIG_NAME: &str = "config";

fn main() -> Result<()> {
    let matches = ClapCmd::new("LeftWM Command")
        .author("BlackDragon2447 <blackdragon2447@e.email>")
        .version(env!("CARGO_PKG_VERSION"))
        .about("a tool for managing your LeftWM config")
        .arg(
            Arg::new("New")
                .short('n')
                .long("new")
                .help("Generate a new config file"),
        )
        .arg(
            Arg::new("Editor")
                .short('e')
                .long("editor")
                .help("Open the current config file in the default editor (default)"),
        )
        .arg(
            Arg::new("TUI")
                .short('t')
                .long("tui")
                .help("Open the current config file in the TUI"),
        )
        .arg(
            Arg::new("Check")
                .short('c')
                .long("check")
                .help("Check if the current config is valid"),
        )
        .arg(
            Arg::new("Verbose")
                .short('v')
                .long("verbose")
                .help("Outputs received configuration file."),
        )
        .arg(
            Arg::new("Migrate")
                .long("migrate")
                .help("Migrate an old .toml config to the RON format."),
        )
        .get_matches();

    let verbose = matches.get_flag("Verbose");

    if matches.get_flag("Migrate") {
        println!("\x1b[0;94m::\x1b[0m Migrating configuration . . .");
        let path = BaseDirectories::with_prefix("leftwm");
        let ron_file = path.place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?;
        let toml_file = path.place_config_file(crate::CONFIG_NAME.to_string() + ".toml")?;

        let config = load_from_file(toml_file.as_os_str().to_str(), verbose)?;

        write_to_file(&ron_file, &config)?;

        return Ok(());
    } else if matches.get_flag("Editor") {
        run_editor(config::filehandler::get_config_file()?.as_path())?;
    } else if matches.get_flag("TUI") {
        crate::tui::run()?;
    } else if matches.get_flag("New") {
        config::filehandler::generate_new_config()?;
    } else if matches.get_flag("Check") {
        check_config(None, verbose)?;
    } else {
        run_editor(config::filehandler::get_config_file()?.as_path())?;
    }

    Ok(())
}

fn run_editor(file: &Path) -> Result<()> {
    let editor = env::var("EDITOR")?;

    let tmp_file = Path::new("/tmp/leftwm-config.ron");
    fs::copy(file, tmp_file)?;

    let run_internal = || -> Result<()> {
        let mut process = Command::new(&editor).arg(tmp_file.as_os_str()).spawn()?;
        if process.wait()?.success() {
            Ok(())
        } else {
            Err(anyhow::Error::msg(format!("Failed to run {}", &editor)))
        }?;
        Ok(())
    };

    run_internal()?;

    while check_config(Some("/tmp/leftwm-config.ron"), false).is_err() {
        println!("Do you want to reopen your editor? [Y/n] ");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;

        if let Some("y" | "Y") = buffer.get(0..1) {
            run_internal()?;
        } else if let Some("n" | "N") = buffer.get(0..1) {
            break;
        } else {
            run_internal()?;
        }
    }

    fs::copy(tmp_file, file)?;

    Ok(())
}
