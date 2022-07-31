use anyhow::Result;
use xdg::BaseDirectories;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::config;
use crate::config::Config;

#[must_use]
pub fn load() -> Config {
    load_from_file(false)
        .map_err(|err| eprintln!("ERROR LOADING CONFIG: {:?}", err))
        .unwrap_or_default()
}

/// # Panics
///
/// Function can only panic if toml cannot be serialized. This should not occur as it is defined
/// globally.
///
/// # Errors
///
/// Function will throw an error if `BaseDirectories` doesn't exist, if user doesn't have
/// permissions to place config.toml, if config.toml cannot be read (access writes, malformed file,
/// etc.).
/// Function can also error from inability to save config.toml (if it is the first time running
/// `LeftWM`).
pub fn load_from_file(verbose: bool) -> Result<Config> {
    let file = BaseDirectories::with_prefix("leftwm")?
        .place_config_file(crate::CONFIG_NAME.to_owned() + ".ron")?;

    if verbose {
        log::debug!("{:?}", &file);
    }
    if file.exists() {
        let contents = fs::read_to_string(&file)?;
        if verbose {
            log::debug!("{:?}", &contents);
        }
        let config = ron::from_str(&contents)?;
        if config::check_workspace_ids(&config) {
            Ok(config)
        } else {
            log::warn!("Invalid workspace ID configuration in config.toml. Falling back to default config.");
            Ok(Config::default())
        }
    } else {
        let config = Config::default();
        let ron_pretty_conf = ron::ser::PrettyConfig::new()
            .depth_limit(2)
            .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
        let ron = ron::ser::to_string_pretty(&config, ron_pretty_conf).unwrap();
        let mut file = File::create(&file)?;
        file.write_all(ron.as_bytes())?;
        Ok(config)
    }
}

pub fn get_config_file() -> Result<PathBuf> {
    Ok(BaseDirectories::with_prefix("leftwm")?
        .place_config_file(crate::CONFIG_NAME.to_owned() + ".ron")?)
}

pub fn save_to_file(config: &Config) -> Result<()> {
    let file = BaseDirectories::with_prefix("leftwm")?
        .place_config_file(crate::CONFIG_NAME.to_owned() + ".ron")?;

    let ron_pretty_conf = ron::ser::PrettyConfig::new()
        .depth_limit(2)
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
    let text = ron::ser::to_string_pretty(&config, ron_pretty_conf).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(&file)?;
    file.set_len(text.as_bytes().len().try_into().unwrap_or(0))?;
    file.write_all(text.as_bytes())?;

    Ok(())
}

pub fn generate_new_config() -> Result<()> {
    let file = BaseDirectories::with_prefix("leftwm")?
        .place_config_file(crate::CONFIG_NAME.to_owned() + ".ron")?;
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
