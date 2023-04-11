use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    Component, Event, MockComponent, NoUserEvent,
};

use crate::{
    config::Config,
    tui::{ConfigUpdate, Msg},
};

pub enum Setting {
    DisableTagSwap,
    DisableTileDrag,
    DisableWindowSnap,
    FocusNewWindows,
    SloppyMouseFollowsFocus,
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
            Setting::DisableTagSwap => TableBuilder::default()
                .add_col(if config.disable_current_tag_swap {
                    TextSpan::from("True").underlined()
                } else {
                    TextSpan::from("True")
                })
                .add_row()
                .add_col(if !config.disable_current_tag_swap {
                    TextSpan::from("False").underlined()
                } else {
                    TextSpan::from("False")
                })
                .build(),
            Setting::DisableTileDrag => TableBuilder::default()
                .add_col(if config.disable_tile_drag {
                    TextSpan::from("True").underlined()
                } else {
                    TextSpan::from("True")
                })
                .add_row()
                .add_col(if !config.disable_tile_drag {
                    TextSpan::from("False").underlined()
                } else {
                    TextSpan::from("False")
                })
                .build(),
            Setting::DisableWindowSnap => TableBuilder::default()
                .add_col(if config.disable_window_snap {
                    TextSpan::from("True").underlined()
                } else {
                    TextSpan::from("True")
                })
                .add_row()
                .add_col(if !config.disable_window_snap {
                    TextSpan::from("False").underlined()
                } else {
                    TextSpan::from("False")
                })
                .build(),
            Setting::FocusNewWindows => TableBuilder::default()
                .add_col(if config.focus_new_windows {
                    TextSpan::from("True").underlined()
                } else {
                    TextSpan::from("True")
                })
                .add_row()
                .add_col(if !config.focus_new_windows {
                    TextSpan::from("False").underlined()
                } else {
                    TextSpan::from("False")
                })
                .build(),
            Setting::SloppyMouseFollowsFocus => TableBuilder::default()
                .add_col(if config.sloppy_mouse_follows_focus {
                    TextSpan::from("True").underlined()
                } else {
                    TextSpan::from("True")
                })
                .add_row()
                .add_col(if !config.sloppy_mouse_follows_focus {
                    TextSpan::from("False").underlined()
                } else {
                    TextSpan::from("False")
                })
                .build(),
        }
    }
}

impl Component<Msg, NoUserEvent> for ToggleValueEditor {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(Msg::SetPopup(None));
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
                    },
                    _ => unreachable!(),
                }
            }

            _ => CmdResult::None,
        };
        None
    }
}
