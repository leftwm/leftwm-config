use std::path::PathBuf;
use std::{env, fs};

pub use check::check_config;
use keybind::Keybind;
use keydaemon_config::KeyDaemonConfig;
use leftwm_core::config::{InsertBehavior, Workspace};
use leftwm_core::layouts::{Layout, LAYOUTS};
use leftwm_core::models::{FocusBehaviour, LayoutMode, ScratchPad, Size};
use modifier::Modifier;
use serde::{Deserialize, Serialize};
use structs::WindowHook;
use wayland_config::WaylandConfig;
use wm_config::WMConfig;

pub mod reexports {
    //reexports
    pub use leftwm_core::{
        config::InsertBehavior,
        layouts::Layout,
        models::{FocusBehaviour, Gutter, LayoutMode, Margins, ScratchPad, Size, Window},
        State,
    };

    pub use lefthk_core::config::Keybind;
}

use crate::command::BaseCommand;

mod check;
pub mod command;
pub mod filehandler;
pub mod keybind;
pub mod keydaemon_config;
pub mod layout;
pub mod modifier;
pub mod structs;
mod utils;
pub mod values;
pub mod wayland_config;
pub mod wm_config;

#[cfg(debug_assertions)]
const CONFIG_NAME: &str = "test_config";
#[cfg(not(debug_assertions))]
const CONFIG_NAME: &str = "config";

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Language {
    Ron,
    Unsupported,
}

pub trait Config: WMConfig + KeyDaemonConfig + WaylandConfig {}

fn default_terminal<'s>() -> &'s str {
    // order from least common to most common.
    // the thinking is if a machine has an uncommon terminal installed, it is intentional
    let terms = &[
        "alacritty",
        "termite",
        "kitty",
        "urxvt",
        "rxvt",
        "st",
        "roxterm",
        "eterm",
        "xterm",
        "terminator",
        "terminology",
        "gnome-terminal",
        "xfce4-terminal",
        "konsole",
        "uxterm",
        "guake", // at the bottom because of odd behaviour. guake wants F12 and should really be
                 // started using autostart instead of LeftWM keybind.
    ];

    // If no terminal found in path, default to a good one
    terms
        .iter()
        .find(|terminal| is_program_in_path(terminal))
        .unwrap_or(&"termite")
}

#[must_use]
pub fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn exit_strategy<'s>() -> &'s str {
    if is_program_in_path("loginctl") {
        return "loginctl kill-session $XDG_SESSION_ID";
    }
    "pkill leftwm"
}

#[allow(dead_code)]
#[must_use]
pub fn check_workspace_ids(config: &Config) -> bool {
    config.workspaces.clone().map_or(true, |wss| {
        let ids = get_workspace_ids(&wss);
        if ids.iter().any(Option::is_some) {
            all_ids_some(&ids) && all_ids_unique(&ids)
        } else {
            true
        }
    })
}

#[must_use]
pub fn get_workspace_ids(wss: &[Workspace]) -> Vec<Option<i32>> {
    // wss.iter().map(|ws| ws.id).collect()
    vec![] //TODO: Fix
}

pub fn all_ids_some(ids: &[Option<i32>]) -> bool {
    ids.iter().all(Option::is_some)
}

#[must_use]
pub fn all_ids_unique(ids: &[Option<i32>]) -> bool {
    let mut sorted = ids.to_vec();
    sorted.sort();
    sorted.dedup();
    ids.len() == sorted.len()
}
