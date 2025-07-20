use crate::config;
use crate::config::Config;
use crate::utils::xkeysym_lookup;
use anyhow::{bail, Error, Result};
use std::collections::HashSet;
use std::process::{Command, Stdio};

use super::is_program_in_path;

pub fn check_config(verbose: bool) -> Result<()> {
    let version = get_leftwm_version()?;

    println!("\x1b[0;94m::\x1b[0m LeftWM version: {}", version.0);
    println!("\x1b[0;94m::\x1b[0m LeftWM git hash: {}", version.1);
    println!("\x1b[0;94m::\x1b[0m Loading configuration . . .");
    match config::filehandler::load_from_file(None, verbose) {
        Ok(config) => {
            println!("\x1b[0;92m    -> Configuration loaded OK \x1b[0m");
            if verbose {
                dbg!(&config);
            }
            config.check_mousekey(verbose);
            config.check_keybinds(verbose);
        }
        Err(e) => {
            println!("Configuration failed. Reason: {e:?}",);
        }
    }
    println!("\x1b[0;94m::\x1b[0m Checking environment . . .");
    check_elogind(verbose)?;

    Ok(())
}

impl Config {
    pub fn check_mousekey(&self, verbose: bool) {
        if verbose {
            println!("Checking if mousekey is set.");
        }
        if let Some(mousekey) = &self.mousekey {
            if verbose {
                println!("Mousekey is set.");
            }
            if mousekey.is_empty() {
                println!(
                    "Your mousekey is set to nothing, this will cause windows to move/resize with just a mouse press."
                );
                return;
            }
            if verbose {
                println!("Mousekey is okay.");
            }
        }
    }

    pub fn check_log_level(&self, verbose: bool) {
        if verbose {
            println!("Trying to parse log_level.");
        }
        let _ = &self
            .log_level
            .parse::<usize>()
            .ok()
            .and_then(|num| match num {
                n @ 0..=5 => Some(n),

                _ => {
                    println!("Numeric log levels must be in 0..=5");
                    None
                }
            })
            .or_else(|| match self.log_level.as_str() {
                "" => Some(1),
                s if s.eq_ignore_ascii_case("error") => Some(1),
                s if s.eq_ignore_ascii_case("warn") => Some(2),
                s if s.eq_ignore_ascii_case("info") => Some(3),
                s if s.eq_ignore_ascii_case("debug") => Some(4),
                s if s.eq_ignore_ascii_case("trace") => Some(5),
                s if s.eq_ignore_ascii_case("off") => Some(0),
                _ => {
                    println!("Loglevel name must be one of \"error\", \"warn\", \"info\", \"debug\", \"trace\" or \"off\" (Case insensitive) ");
                    None
                },
            });
    }

    /// Check all keybinds to ensure that required values are provided
    /// Checks to see if value is provided (if required)
    /// Checks to see if keys are valid against Xkeysym
    /// Ideally, we will pass this to the command handler with a dummy config
    pub fn check_keybinds(&self, verbose: bool) {
        let mut returns = Vec::new();
        println!("\x1b[0;94m::\x1b[0m Checking keybinds . . .");
        let mut bindings = HashSet::new();
        for keybind in &self.keybind {
            if verbose {
                println!(
                    "Keybind: {:?} value field is empty: {}",
                    keybind,
                    keybind.value.is_empty()
                );
            }
            if let Err(err) = keybind.try_convert_to_lefthk_keybind(self) {
                returns.push((Some(keybind.clone()), err.to_string()));
            }
            if xkeysym_lookup::into_keysym(&keybind.key).is_none() {
                returns.push((
                    Some(keybind.clone()),
                    format!("Key `{}` is not valid", keybind.key),
                ));
            }

            let mut modkey = keybind.modifier.as_ref().unwrap_or(&"None".into()).clone();
            for m in &modkey.clone() {
                if m != "modkey" && m != "mousekey" && xkeysym_lookup::into_mod(&m) == 0 {
                    returns.push((
                        Some(keybind.clone()),
                        format!("Modifier `{m}` is not valid"),
                    ));
                }
            }

            modkey.sort_unstable();
            if let Some(conflict_key) = bindings.replace((modkey.clone(), &keybind.key)) {
                returns.push((
                    None,
                    format!(
                        "\x1b[0m\x1b[1mMultiple commands bound to key combination {} + {}:\
                    \n\x1b[1;91m    -> {:?}\
                    \n    -> {:?}\
                    \n\x1b[0mHelp: change one of the keybindings to something else.\n",
                        modkey, keybind.key, conflict_key, keybind.command,
                    ),
                ));
            }
        }
        if returns.is_empty() {
            println!("\x1b[0;92m    -> All keybinds OK\x1b[0m");
        } else {
            for error in returns {
                match error.0 {
                    Some(binding) => {
                        println!(
                            "\x1b[1;91mERROR: {} for keybind {binding:?}\x1b[0m",
                            error.1
                        );
                    }
                    None => {
                        println!("\x1b[1;91mERROR: {} \x1b[0m", error.1);
                    }
                }
            }
        }
    }
}

fn check_elogind(verbose: bool) -> Result<()> {
    // We assume that if it is in the path it's all good
    // We also cross-reference the ENV variable
    match (
        std::env::var("XDG_RUNTIME_DIR"),
        is_program_in_path("loginctl"),
    ) {
        (Ok(val), true) => {
            if verbose {
                println!(":: XDG_RUNTIME_DIR: {val}, LOGINCTL OKAY");
            }

            println!("\x1b[0;92m    -> Environment OK \x1b[0m");

            Ok(())
        }
        (Ok(val), false) => {
            if verbose {
                println!(":: XDG_RUNTIME_DIR: {val}, LOGINCTL not installed");
            }

            println!("\x1b[0;92m    -> Environment OK (has XDG_RUNTIME_DIR) \x1b[0m");

            Ok(())
        }
        (Err(e), false) => {
            if verbose {
                println!(":: XDG_RUNTIME_DIR_ERROR: {e:?}, LOGINCTL BAD");
            }

            bail!(
                "Elogind not installed/operating and no alternative XDG_RUNTIME_DIR is set. \
                See https://github.com/leftwm/leftwm/wiki/XDG_RUNTIME_DIR for more information."
            );
        }
        (Err(e), true) => {
            if verbose {
                println!(":: XDG_RUNTIME_DIR: {e:?}, LOGINCTL OKAY");
            }
            println!(
                "\x1b[1;93mWARN: Elogind/systemd installed but XDG_RUNTIME_DIR not set.\nThis may be because elogind isn't started. \x1b[0m",
            );
            Ok(())
        }
    }
}

fn get_leftwm_version() -> Result<(String, String)> {
    match Command::new("leftwm")
        .args(vec!["--version"])
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(child) => {
            let buffer = String::from_utf8(child.wait_with_output()?.stdout)?;
            let stuff: Vec<&str> = buffer.split(' ').collect();
            Ok((
                stuff.get(1).unwrap_or(&"5.4.0,").replace('\n', ""),
                (*stuff.get(3).unwrap_or(&"Unknown")).to_string(),
            ))
        }
        Err(e) => Err(Error::msg(format!("failed to run leftwm --version. {e}"))),
    }
}
