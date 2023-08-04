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

#[derive(MockComponent)]
pub struct ModKeyEditor {
    component: Table,
}

impl ModKeyEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("Modkey", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .column_spacing(3)
                .widths(&[100])
                .table(Self::build_inner(config)),
        }
    }

    fn build_inner(config: &Config) -> Vec<Vec<TextSpan>> {
        TableBuilder::default()
            .add_col(if config.modkey == "None" {
                TextSpan::from("None").underlined()
            } else {
                TextSpan::from("None")
            })
            .add_row()
            .add_col(if config.modkey == "Shift" {
                TextSpan::from("Shift").underlined()
            } else {
                TextSpan::from("Shift")
            })
            .add_row()
            .add_col(if config.modkey == "Control" {
                TextSpan::from("Control").underlined()
            } else {
                TextSpan::from("Control")
            })
            .add_row()
            .add_col(if config.modkey == "Alt" || config.modkey == "Mod1" {
                TextSpan::from("Alt").underlined()
            } else {
                TextSpan::from("Alt")
            })
            .add_row()
            .add_col(if config.modkey == "Mod3" {
                TextSpan::from("Mod3").underlined()
            } else {
                TextSpan::from("Mod3")
            })
            .add_row()
            .add_col(if config.modkey == "Super" || config.modkey == "Mod4" {
                TextSpan::from("Super").underlined()
            } else {
                TextSpan::from("Super")
            })
            .add_row()
            .add_col(if config.modkey == "Mod5" {
                TextSpan::from("Mod5").underlined()
            } else {
                TextSpan::from("Mod5")
            })
            .build()
    }
}

impl Component<Msg, NoUserEvent> for ModKeyEditor {
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
                    0 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("None".to_string()),
                        true,
                    )),
                    1 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Shift".to_string()),
                        true,
                    )),
                    2 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Control".to_string()),
                        true,
                    )),
                    3 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Alt".to_string()),
                        true,
                    )),
                    4 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Mod3".to_string()),
                        true,
                    )),
                    5 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Super".to_string()),
                        true,
                    )),
                    6 => Some(Msg::UpdateConfig(
                        ConfigUpdate::ModKey("Mod5".to_string()),
                        true,
                    )),
                    _ => unreachable!(),
                }
            }
            _ => CmdResult::None,
        };
        None
    }
}
