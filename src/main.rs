#![feature(is_some_with)]

extern crate core;

mod config;
mod utils;

use anyhow::{anyhow, bail, Result};
use clap::{App, Arg};
use glob::glob;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use xdg::BaseDirectories;
use crate::config::{check_config, Config, Language};

const CONFIG_NAME: &'static str = "test_config";

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
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Outputs received configuration file."),
        )
        .get_matches();

    let verbose = matches.occurrences_of("verbose") >= 1;

    let (config_file, config_lang) = get_config_language_and_file()?;

    if matches.is_present("Editor") {
        run_editor(config_file)?;
    } else if matches.is_present("TUI") {
        crate::utils::tui::run(config_file, config_lang)?;
    } else if matches.is_present("New") {
        generate_new_config(config_file, config_lang)?;
    } else if matches.is_present("Check") {
        check_config(verbose, config_file, config_lang)?;
    } else {
        run_editor(config_file)?;
    }

    Ok(())
}

fn get_config_language_and_file() -> Result<(PathBuf, config::Language)> {
    let config_dir = BaseDirectories::new()?.create_config_directory("leftwm")?;
    let files = glob(
        &(config_dir
            .to_str()
            .ok_or(anyhow!("That path does not exsist"))?
            .to_owned()
            + "/"
            + CONFIG_NAME
            + ".*"),
    )?;
    for file in files {
        let file = file?;
        let filename = file
            .clone()
            .file_name()
            .ok_or(anyhow!("Error"))?
            .to_os_string()
            .to_str()
            .ok_or(anyhow!("failed to convert to str"))?
            .to_string();
        match filename
            .split('.')
            .last()
            .ok_or(anyhow!("failed to split string"))?
        {
            "ron" => return Ok((file.clone(), config::Language::RON)),
            _ => bail!("no valid config file found"),
        }
    }
    unreachable!();
}

fn generate_new_config(file: PathBuf, lang: Language) -> Result<()> {
    if file.exists() {
        println!(
            "\x1b[0;94m::\x1b[0m A config file already exists, do you want to override it? [y/N]"
        );
        let mut line = String::new();
        let _ = std::io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if line.contains('y') || line.contains('Y') {
            let config = Config::default();
            let text = match lang {
                Ron => {
                    let ron_pretty_conf = ron::ser::PrettyConfig::new()
                        .depth_limit(2)
                        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
                    ron::ser::to_string_pretty(&config, ron_pretty_conf)?
                }
                _ => bail!("Unsupported or unknow config language"),
            };
            let mut file = File::create(&file)?;
            file.write_all(text.as_bytes())?;
        }
    }

    Ok(())
}

fn run_editor(file: PathBuf) -> Result<()> {
    let editor = env::var("EDITOR")?;

    let mut process = Command::new(&editor).arg(file.as_os_str()).spawn()?;
    match process.wait()?.success() {
        true => Ok(()),
        false => Err(anyhow::Error::msg(format!("Failed to run {}", &editor))),
    }?;

    Ok(())
}
