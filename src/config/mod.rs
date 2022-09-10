use std::{env, fs};

pub use check::check_config;
use layout::Layout;
use serde::{Deserialize, Serialize};

use crate::config::keybind::Keybind;
use crate::config::layout::LAYOUTS;
use crate::config::modifier::Modifier;
use crate::config::structs::{ScratchPad, WindowHook, Workspace};
use crate::config::values::{BaseCommand, FocusBehaviour, InsertBehavior, LayoutMode, Size};

mod check;
mod command;
pub mod filehandler;
mod keybind;
pub mod layout;
pub mod modifier;
pub(crate) mod structs;
pub mod values;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Language {
    Ron,
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub modkey: String,
    pub mousekey: Option<Modifier>,
    pub tags: Option<Vec<String>>,
    pub max_window_width: Option<Size>,
    pub layouts: Vec<Layout>,
    pub layout_mode: LayoutMode,
    pub insert_behavior: InsertBehavior,
    pub scratchpad: Option<Vec<ScratchPad>>,
    pub window_rules: Option<Vec<WindowHook>>,
    //of you are on tag "1" and you goto tag "1" this takes you to the previous tag
    pub disable_current_tag_swap: bool,
    pub disable_tile_drag: bool,
    pub focus_behaviour: FocusBehaviour,
    pub focus_new_windows: bool,
    pub keybind: Vec<Keybind>,
    pub workspaces: Option<Vec<Workspace>>,
}

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

impl Default for Config {
    // We allow this because this function would be difficult to reduce. If someone would like to
    // move the commands builder out, perhaps make a macro, this function could be reduced in size
    // considerably.
    #[allow(clippy::too_many_lines)]
    fn default() -> Self {
        const WORKSPACES_NUM: usize = 10;
        let mut commands = vec![
            // Mod + p => Open dmenu
            Keybind {
                command: BaseCommand::Execute,
                value: "dmenu_run".to_owned(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "p".to_owned(),
            },
            // Mod + Shift + Enter => Open A Shell
            Keybind {
                command: BaseCommand::Execute,
                value: default_terminal().to_owned(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "Return".to_owned(),
            },
            // Mod + Shift + q => kill focused window
            Keybind {
                command: BaseCommand::CloseWindow,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "q".to_owned(),
            },
            // Mod + Shift + r => soft reload leftwm
            Keybind {
                command: BaseCommand::SoftReload,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "r".to_owned(),
            },
            // Mod + Shift + x => exit leftwm
            Keybind {
                command: BaseCommand::Execute,
                value: exit_strategy().to_owned(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "x".to_owned(),
            },
            // Mod + Ctrl + l => lock the screen
            Keybind {
                command: BaseCommand::Execute,
                value: "slock".to_owned(),
                modifier: Some(vec!["modkey".to_owned(), "Control".to_owned()].into()),
                key: "l".to_owned(),
            },
            // Mod + Shift + w => swap the tags on the last to active workspaces
            Keybind {
                command: BaseCommand::MoveToLastWorkspace,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "w".to_owned(),
            },
            // Mod + w => move the active window to the previous workspace
            Keybind {
                command: BaseCommand::SwapTags,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "w".to_owned(),
            },
            Keybind {
                command: BaseCommand::MoveWindowUp,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "k".to_owned(),
            },
            Keybind {
                command: BaseCommand::MoveWindowDown,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "j".to_owned(),
            },
            Keybind {
                command: BaseCommand::MoveWindowTop,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "Return".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWindowUp,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "k".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWindowDown,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "j".to_owned(),
            },
            Keybind {
                command: BaseCommand::NextLayout,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Control".to_owned()].into()),
                key: "k".to_owned(),
            },
            Keybind {
                command: BaseCommand::PreviousLayout,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Control".to_owned()].into()),
                key: "j".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWorkspaceNext,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "l".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWorkspacePrevious,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "h".to_owned(),
            },
            Keybind {
                command: BaseCommand::MoveWindowUp,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "Up".to_owned(),
            },
            Keybind {
                command: BaseCommand::MoveWindowDown,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: "Down".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWindowUp,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "Up".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWindowDown,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "Down".to_owned(),
            },
            Keybind {
                command: BaseCommand::NextLayout,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Control".to_owned()].into()),
                key: "Up".to_owned(),
            },
            Keybind {
                command: BaseCommand::PreviousLayout,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned(), "Control".to_owned()].into()),
                key: "Down".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWorkspaceNext,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "Right".to_owned(),
            },
            Keybind {
                command: BaseCommand::FocusWorkspacePrevious,
                value: String::default(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: "Left".to_owned(),
            },
        ];

        // add "goto workspace"
        for i in 1..WORKSPACES_NUM {
            commands.push(Keybind {
                command: BaseCommand::GotoTag,
                value: i.to_string(),
                modifier: Some(vec!["modkey".to_owned()].into()),
                key: i.to_string(),
            });
        }

        // and "move to workspace"
        for i in 1..WORKSPACES_NUM {
            commands.push(Keybind {
                command: BaseCommand::MoveToTag,
                value: i.to_string(),
                modifier: Some(vec!["modkey".to_owned(), "Shift".to_owned()].into()),
                key: i.to_string(),
            });
        }

        let tags = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();

        Self {
            workspaces: Some(vec![]),
            tags: Some(tags),
            layouts: LAYOUTS.to_vec(),
            layout_mode: LayoutMode::Workspace,
            // TODO: add sane default for scratchpad config.
            // Currently default values are set in sane_dimension fn.
            scratchpad: Some(vec![]),
            window_rules: Some(vec![]),
            disable_current_tag_swap: false,
            disable_tile_drag: false,
            focus_behaviour: FocusBehaviour::Sloppy, // default behaviour: mouse move auto-focuses window
            focus_new_windows: true, // default behaviour: focuses windows on creation
            insert_behavior: InsertBehavior::default(),
            modkey: "Mod4".to_owned(),     //win key
            mousekey: Some("Mod4".into()), //win key
            keybind: commands,
            max_window_width: None,
        }
    }
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
    wss.iter().map(|ws| ws.id).collect()
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
