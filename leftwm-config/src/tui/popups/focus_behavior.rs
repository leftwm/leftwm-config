use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    Component, Event, MockComponent, NoUserEvent,
};

use crate::tui::{ConfigUpdate, Msg};

use leftwm_config_core::{values::FocusBehaviour, Config};

#[derive(MockComponent)]
pub struct FocusBehaviorEditor {
    component: Table,
}

impl FocusBehaviorEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("Focus Behavior", Alignment::Center)
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
            .add_col(if config.focus_behaviour == FocusBehaviour::Sloppy {
                TextSpan::from("Sloppy").underlined()
            } else {
                TextSpan::from("Sloppy")
            })
            .add_row()
            .add_col(if config.focus_behaviour == FocusBehaviour::Driven {
                TextSpan::from("Driven").underlined()
            } else {
                TextSpan::from("Driven")
            })
            .add_row()
            .add_col(if config.focus_behaviour == FocusBehaviour::ClickTo {
                TextSpan::from("Click To").underlined()
            } else {
                TextSpan::from("Click To")
            })
            .build()
    }
}

impl Component<Msg, NoUserEvent> for FocusBehaviorEditor {
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
            }) => match self.component.states.list_index {
                0 => {
                    return Some(Msg::UpdateConfig(
                        ConfigUpdate::FocusBehaviour(FocusBehaviour::Sloppy),
                        true,
                    ))
                }
                1 => {
                    return Some(Msg::UpdateConfig(
                        ConfigUpdate::FocusBehaviour(FocusBehaviour::Driven),
                        true,
                    ))
                }
                2 => {
                    return Some(Msg::UpdateConfig(
                        ConfigUpdate::FocusBehaviour(FocusBehaviour::ClickTo),
                        true,
                    ))
                }
                _ => unreachable!(),
            },
            _ => CmdResult::None,
        };
        None
    }
}
