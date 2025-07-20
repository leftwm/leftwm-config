use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use xdg::BaseDirectories;

use crate::config::Config;

#[must_use]
pub fn load() -> Config {
    load_from_file(None, false)
        .map_err(|err| eprintln!("ERROR LOADING CONFIG: {:?}", err))
        .unwrap_or_default()
}

/// Loads configuration from either specified file (preferred) or default.
/// # Errors
///
/// Errors if file cannot be read. Indicates filesystem error
/// (inadequate permissions, disk full, etc.)
/// If a path is specified and does not exist, returns `LeftError`.
pub fn load_from_file(fspath: Option<&str>, verbose: bool) -> Result<Config> {
    let config_filename = if let Some(fspath) = fspath {
        println!("\x1b[1;35mNote: Using file {} \x1b[0m", fspath);
        PathBuf::from(fspath)
    } else {
        let ron_file = BaseDirectories::with_prefix("leftwm")
            .place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?;
        let toml_file = BaseDirectories::with_prefix("leftwm")
            .place_config_file(crate::CONFIG_NAME.to_string() + ".toml")?;
        if Path::new(&ron_file).exists() {
            ron_file
        } else if Path::new(&toml_file).exists() {
            println!(
                "\x1b[1;93mWARN: TOML as config format is about to be deprecated.
      Please consider migrating to RON manually or by using `leftwm-check -m`.\x1b[0m"
            );
            toml_file
        } else {
            let config = Config::default();
            write_to_file(&ron_file, &config)?;
            return Ok(config);
        }
    };
    if verbose {
        dbg!(&config_filename);
    }
    let contents = fs::read_to_string(&config_filename)?;
    if verbose {
        dbg!(&contents);
    }
    if config_filename.as_path().extension() == Some(std::ffi::OsStr::new("ron")) {
        let config = ron::from_str(&contents)?;
        Ok(config)
    } else {
        let config = toml::from_str(&contents)?;
        Ok(config)
    }
}

pub fn get_config_file() -> Result<PathBuf> {
    let ron_file = BaseDirectories::with_prefix("leftwm")
        .place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?;
    let toml_file = BaseDirectories::with_prefix("leftwm")
        .place_config_file(crate::CONFIG_NAME.to_string() + ".toml")?;
    if Path::new(&ron_file).exists() {
        Ok(ron_file)
    } else if Path::new(&toml_file).exists() {
        println!(
            "\x1b[1;93mWARN: TOML as config format is about to be deprecated.
      Please consider migrating to RON manually or by using `leftwm-config --migrate`.\x1b[0m"
        );
        Ok(toml_file)
    } else {
        let config = Config::default();
        write_to_file(&ron_file, &config)?;
        Ok(ron_file)
    }
}

pub fn save_to_file(config: &Config) -> Result<()> {
    write_to_file(
        &BaseDirectories::with_prefix("leftwm")
            .place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?,
        config,
    )
}

pub fn write_to_file(ron_file: &PathBuf, config: &Config) -> Result<(), anyhow::Error> {
    let ron_pretty_conf = ron::ser::PrettyConfig::new()
        .depth_limit(2)
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
    let ron = ron::ser::to_string_pretty(&config, ron_pretty_conf)?;
    let comment_header = String::from(
        r#"//  _        ___                                      ___ _
// | |      / __)_                                   / __|_)
// | | ____| |__| |_ _ _ _ ____      ____ ___  ____ | |__ _  ____    ____ ___  ____
// | |/ _  )  __)  _) | | |    \    / ___) _ \|  _ \|  __) |/ _  |  / ___) _ \|  _ \
// | ( (/ /| |  | |_| | | | | | |  ( (__| |_| | | | | |  | ( ( | |_| |  | |_| | | | |
// |_|\____)_|   \___)____|_|_|_|   \____)___/|_| |_|_|  |_|\_|| (_)_|   \___/|_| |_|
// A WindowManager for Adventurers                         (____/
// For info about configuration please visit https://github.com/leftwm/leftwm/wiki
"#,
    );
    let ron_with_header = comment_header + &ron;
    let mut file = File::create(ron_file)?;
    file.write_all(ron_with_header.as_bytes())?;
    Ok(())
}

pub fn generate_new_config() -> Result<()> {
    let file = BaseDirectories::with_prefix("leftwm")
        .place_config_file(crate::CONFIG_NAME.to_string() + ".ron")?;

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
            let ron_pretty_conf = ron::ser::PrettyConfig::new()
                .depth_limit(2)
                .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
            let text = ron::ser::to_string_pretty(&config, ron_pretty_conf)?;
            let mut file = File::create(&file)?;
            file.write_all(text.as_bytes())?;
        }
    }

    Ok(())
}
