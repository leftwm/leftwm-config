use crate::config;
use crate::config::Config;
use crate::config::{all_ids_some, all_ids_unique, get_workspace_ids};
use anyhow::bail;
use anyhow::Result;
use std::collections::HashSet;
use std::{env, fs};

pub fn check_config(path: Option<&str>, verbose: bool) -> Result<()> {
    println!(
        "\x1b[0;94m::\x1b[0m LeftWM Config version: {}",
        std::env::var("CARGO_PKG_VERSON").unwrap_or("unknown".to_string())
    );
    println!(
        "\x1b[0;94m::\x1b[0m LeftWM git hash: {}",
        option_env!("GIT_HASH").unwrap_or(git_version::git_version!(fallback = "unknown"))
    );
    println!("\x1b[0;94m::\x1b[0m Loading configuration . . .");
    match config::filehandler::load_from_file(path, verbose) {
        Ok(config) => {
            println!("\x1b[0;92m    -> Configuration loaded OK \x1b[0m");
            if verbose {
                dbg!(&config);
            }
            config.check_mousekey(verbose);
            config.check_workspace_ids(verbose);
            config.check_keybinds(verbose);
        }
        Err(e) => {
            println!("Configuration failed. Reason: {e:?}");
            bail!("Configuration failed. Reason: {:?}", e);
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
                println!("Your mousekey is set to nothing, this will cause windows to move/resize with just a mouse press.");
                return;
            }
            if verbose {
                println!("Mousekey is okay.");
            }
        }
    }

    /// Checks defined workspaces to ensure no ID collisions occur.
    pub fn check_workspace_ids(&self, verbose: bool) {
        if let Some(wss) = self.workspaces.as_ref() {
            if verbose {
                println!("Checking config for valid workspace definitions.");
            }
            let ids = get_workspace_ids(wss);
            if ids.iter().any(std::option::Option::is_some) {
                if all_ids_some(&ids) {
                    if !all_ids_unique(&ids) {
                        println!("Your config.toml contains duplicate workspace IDs. Please assign unique IDs to workspaces. The default config will be used instead.");
                    }
                } else {
                    println!("Your config.toml specifies an ID for some but not all workspaces. This can lead to ID collisions and is not allowed. The default config will be used instead.");
                }
            }
        }
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
                println!("Keybind: {:?} {}", keybind, keybind.value.is_empty());
            }
            if let Err(err) = keybind.try_convert_to_core_keybind(self) {
                returns.push((Some(keybind.clone()), err.to_string()));
            }
            if crate::utils::xkeysym_lookup::into_keysym(&keybind.key).is_none() {
                returns.push((
                    Some(keybind.clone()),
                    format!("Key `{}` is not valid", keybind.key),
                ));
            }

            let mut modkey = keybind.modifier.as_ref().unwrap_or(&"None".into()).clone();
            for m in &modkey.clone() {
                if m != "modkey"
                    && m != "mousekey"
                    && crate::utils::xkeysym_lookup::into_mod(&m) == 0
                {
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
                            "\x1b[1;91mERROR: {} for keybind {:?}\x1b[0m",
                            error.1, binding
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

#[must_use]
fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{p}/{program}");
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}
