use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    Attribute, Component, Event, MockComponent, State,
};

use crate::{
    config::Config,
    tui::{model::UserEvent, ConfigUpdate, Msg},
};

pub enum Setting {
    DisableTagSwap,
    DisableTileDrag,
    DisableWindowSnap,
    FocusNewWindows,
    SloppyMouseFollowsFocus,
    AutoDeriveWorkspace,
    DisableCursorRepositionOnResize,
    SingleWindowBorder,
}

#[derive(MockComponent)]
pub struct ToggleValueEditor {
    component: Table,
    setting: Setting,
}

impl ToggleValueEditor {
    pub fn new(config: &Config, setting: Setting) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title(
                    match setting {
                        Setting::DisableTagSwap => "Disable Current Tag Swap",
                        Setting::DisableTileDrag => "Disable Tile Drag",
                        Setting::DisableWindowSnap => "Disable Window Snap",
                        Setting::FocusNewWindows => "Focus New Windows",
                        Setting::SloppyMouseFollowsFocus => "Sloppy Mouse Flollows Focus",
                        Setting::AutoDeriveWorkspace => "Auto Derive Workspace",
                        Setting::DisableCursorRepositionOnResize => {
                            "Disable Cursor Reposition On Resize"
                        }
                        Setting::SingleWindowBorder => "Single Window Border",
                    },
                    Alignment::Left,
                )
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .column_spacing(3)
                .widths(&[100])
                .table(Self::build_inner(config, &setting)),
            setting,
        }
    }

    fn build_inner(config: &Config, setting: &Setting) -> Vec<Vec<TextSpan>> {
        match setting {
            Setting::DisableTagSwap => table_from_value(config.disable_current_tag_swap),
            Setting::DisableTileDrag => table_from_value(config.disable_tile_drag),
            Setting::DisableWindowSnap => table_from_value(config.disable_window_snap),
            Setting::FocusNewWindows => table_from_value(config.focus_new_windows),
            Setting::SloppyMouseFollowsFocus => table_from_value(config.sloppy_mouse_follows_focus),
            Setting::AutoDeriveWorkspace => table_from_value(config.auto_derive_workspaces),
            Setting::DisableCursorRepositionOnResize => {
                table_from_value(config.disable_cursor_reposition_on_resize)
            }
            Setting::SingleWindowBorder => table_from_value(config.single_window_border),
        }
    }
}

fn table_from_value(value: bool) -> Vec<Vec<TextSpan>> {
    TableBuilder::default()
        .add_col(if value {
            TextSpan::from("True").underlined()
        } else {
            TextSpan::from("True")
        })
        .add_row()
        .add_col(if !value {
            TextSpan::from("False").underlined()
        } else {
            TextSpan::from("False")
        })
        .build()
}

impl Component<Msg, UserEvent> for ToggleValueEditor {
    fn on(&mut self, ev: tuirealm::Event<UserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(Msg::SetHomePopup(None));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                return match self.component.states.list_index {
                    0 => match self.setting {
                        Setting::DisableTagSwap => {
                            Some(Msg::UpdateConfig(ConfigUpdate::DisableTagSwap(true), true))
                        }
                        Setting::DisableTileDrag => {
                            Some(Msg::UpdateConfig(ConfigUpdate::DisableTileDrag(true), true))
                        }
                        Setting::DisableWindowSnap => Some(Msg::UpdateConfig(
                            ConfigUpdate::DisableWindowSnap(true),
                            true,
                        )),
                        Setting::FocusNewWindows => {
                            Some(Msg::UpdateConfig(ConfigUpdate::FocusNewWindows(true), true))
                        }
                        Setting::SloppyMouseFollowsFocus => Some(Msg::UpdateConfig(
                            ConfigUpdate::SloppyMouseFollowsFocus(true),
                            true,
                        )),
                        Setting::AutoDeriveWorkspace => Some(Msg::UpdateConfig(
                            ConfigUpdate::AutoDeriveWorkspaces(true),
                            true,
                        )),
                        Setting::DisableCursorRepositionOnResize => Some(Msg::UpdateConfig(
                            ConfigUpdate::DisableCursorRepositionOnResize(true),
                            true,
                        )),
                        Setting::SingleWindowBorder => Some(Msg::UpdateConfig(
                            ConfigUpdate::SingleWindowBorder(true),
                            true,
                        )),
                    },
                    1 => match self.setting {
                        Setting::DisableTagSwap => {
                            Some(Msg::UpdateConfig(ConfigUpdate::DisableTagSwap(false), true))
                        }
                        Setting::DisableTileDrag => Some(Msg::UpdateConfig(
                            ConfigUpdate::DisableTileDrag(false),
                            true,
                        )),
                        Setting::DisableWindowSnap => Some(Msg::UpdateConfig(
                            ConfigUpdate::DisableWindowSnap(false),
                            true,
                        )),
                        Setting::FocusNewWindows => Some(Msg::UpdateConfig(
                            ConfigUpdate::FocusNewWindows(false),
                            true,
                        )),
                        Setting::SloppyMouseFollowsFocus => Some(Msg::UpdateConfig(
                            ConfigUpdate::SloppyMouseFollowsFocus(false),
                            true,
                        )),
                        Setting::AutoDeriveWorkspace => Some(Msg::UpdateConfig(
                            ConfigUpdate::AutoDeriveWorkspaces(false),
                            true,
                        )),
                        Setting::DisableCursorRepositionOnResize => Some(Msg::UpdateConfig(
                            ConfigUpdate::DisableCursorRepositionOnResize(false),
                            true,
                        )),
                        Setting::SingleWindowBorder => Some(Msg::UpdateConfig(
                            ConfigUpdate::SingleWindowBorder(false),
                            true,
                        )),
                    },
                    _ => unreachable!(),
                }
            }
            Event::User(UserEvent::ConfigUpdate(u)) => {
                let b = match (&self.setting, u) {
                    (Setting::DisableTagSwap, ConfigUpdate::DisableTagSwap(b)) => Some(b),
                    (Setting::DisableTileDrag, ConfigUpdate::DisableTileDrag(b)) => Some(b),
                    (Setting::DisableWindowSnap, ConfigUpdate::DisableWindowSnap(b)) => Some(b),
                    (Setting::FocusNewWindows, ConfigUpdate::FocusNewWindows(b)) => Some(b),
                    (
                        Setting::SloppyMouseFollowsFocus,
                        ConfigUpdate::SloppyMouseFollowsFocus(b),
                    ) => Some(b),
                    (Setting::AutoDeriveWorkspace, ConfigUpdate::AutoDeriveWorkspaces(b)) => {
                        Some(b)
                    }
                    (
                        Setting::DisableCursorRepositionOnResize,
                        ConfigUpdate::DisableCursorRepositionOnResize(b),
                    ) => Some(b),
                    (Setting::SingleWindowBorder, ConfigUpdate::SingleWindowBorder(b)) => Some(b),
                    _ => None,
                };
                match b {
                    Some(b) => {
                        self.attr(
                            Attribute::Content,
                            tuirealm::AttrValue::Table(table_from_value(b)),
                        );
                        CmdResult::Changed(State::None)
                    }
                    None => CmdResult::None,
                }
            }

            _ => CmdResult::None,
        };
        None
    }
}
