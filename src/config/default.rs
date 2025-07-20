use crate::config::{
    command::BaseCommand,
    keybind::Keybind,
    structs::ScratchPad,
    values::{FocusBehaviour, InsertBehavior, LayoutMode, Size},
    Backend, FocusOnActivationBehaviour, WindowHidingStrategy,
};

use super::{is_program_in_path, Config};

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

        let tags = ["1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();

        let scratchpad = ScratchPad {
            name: "Alacritty".into(),
            args: None,
            value: "alacritty".to_string(),
            x: Some(Size::Pixel(860)),
            y: Some(Size::Pixel(390)),
            height: Some(Size::Pixel(300)),
            width: Some(Size::Pixel(200)),
        };

        let layouts = leftwm_layouts::layouts::Layouts::default();

        Self {
            log_level: String::from("debug"),
            // Using Backend's feature fallback
            backend: Backend::default(),
            workspaces: Some(vec![]),
            tags: Some(tags),
            layouts: layouts.names(),
            layout_definitions: layouts.layouts,
            layout_mode: LayoutMode::Tag,
            // TODO: add sane default for scratchpad config.
            // Currently default values are set in sane_dimension fn.
            scratchpad: Some(vec![scratchpad]),
            window_rules: Some(vec![]),
            disable_current_tag_swap: false,
            disable_tile_drag: false,
            disable_window_snap: true,
            focus_behaviour: FocusBehaviour::Sloppy, // default behaviour: mouse move auto-focuses window
            focus_on_activation: FocusOnActivationBehaviour::default(),
            focus_new_windows: true, // default behaviour: focuses windows on creation
            single_window_border: true,
            insert_behavior: InsertBehavior::Bottom,
            window_hiding_strategy: WindowHidingStrategy::default(),
            modkey: "Mod4".to_owned(),     // win key
            mousekey: Some("Mod4".into()), // win key
            keybind: commands,
            state_path: None,
            sloppy_mouse_follows_focus: true,
            create_follows_cursor: None,
            disable_cursor_reposition_on_resize: false,
            auto_derive_workspaces: true,
        }
    }
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

fn exit_strategy<'s>() -> &'s str {
    if is_program_in_path("loginctl") {
        return "loginctl kill-session $XDG_SESSION_ID";
    }
    "pkill leftwm"
}
