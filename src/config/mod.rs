use std::path::PathBuf;
use std::{env, fs};

pub use check::check_config;
use keybind::Keybind;
use leftwm_layouts::Layout;
use serde::{Deserialize, Serialize};
use values::LayoutMode;

use crate::config::modifier::Modifier;
use crate::config::structs::{ScratchPad, WindowHook, Workspace};
use crate::config::values::{FocusBehaviour, InsertBehavior};

mod check;
pub mod command;
mod default;
pub mod filehandler;
pub mod keybind;
// pub mod layout;
pub mod modifier;
pub mod structs;
pub mod values;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Language {
    Ron,
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum Backend {
    #[default]
    XLib,
    X11rb,
}

/// Controls behaviour for window activation. Default is to mark the window as urgent.
#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusOnActivationBehaviour {
    /// Do nothing.
    DoNothing,
    /// Mark the window as urgent.
    #[default]
    MarkUrgent,
    /// Switch to the window.
    SwitchTo,
}

/// The stategy used to hide windows when switching tags in the backend
#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum WindowHidingStrategy {
    /// The common behaviour for a window manager, but it prevents hidden windows from being
    /// captured by other applications
    Unmap,
    /// Move the windows out of the visible area, so it can still be captured by some applications.
    /// We still inform the window that it is in a "minimized"-like state, so it can probably
    /// decide to not render its content as if it was focused.
    MoveMinimize,
    /// Move the windows out of the visible area and don't minilize them.
    /// This should allow all applications to be captured by any other applications.
    /// This could result in higher resource usage, since windows will render their content like
    /// normal even if hidden.
    #[default]
    MoveOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct Config {
    pub log_level: String, // Done
    pub backend: Backend,  // Done

    pub mousekey: Option<Modifier>,                // Done
    pub disable_tile_drag: bool,                   // Done
    pub disable_window_snap: bool,                 // Done
    pub disable_current_tag_swap: bool,            // Done
    pub disable_cursor_reposition_on_resize: bool, // Done

    pub focus_behaviour: FocusBehaviour,                 // Done
    pub focus_new_windows: bool,                         // Done
    pub sloppy_mouse_follows_focus: bool,                // Done
    pub focus_on_activation: FocusOnActivationBehaviour, // Done ?

    pub insert_behavior: InsertBehavior,     // Done
    pub create_follows_cursor: Option<bool>, // Done

    pub layout_mode: LayoutMode,         // Done
    pub layouts: Vec<String>,            // Complex
    pub layout_definitions: Vec<Layout>, // Complex

    pub auto_derive_workspaces: bool,       // Done
    pub workspaces: Option<Vec<Workspace>>, // Complex

    pub scratchpad: Option<Vec<ScratchPad>>, // Complex

    pub tags: Option<Vec<String>>,                    // Complex
    pub window_hiding_strategy: WindowHidingStrategy, // Done

    pub window_rules: Option<Vec<WindowHook>>, // Complex
    pub single_window_border: bool,            // Done

    pub modkey: String,        // Done
    pub keybind: Vec<Keybind>, // Complex

    pub state_path: Option<PathBuf>, // Done
}

#[must_use]
pub fn is_program_in_path(program: &str) -> bool {
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
