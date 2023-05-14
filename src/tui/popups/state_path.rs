use std::path::PathBuf;

use tui_realm_stdlib::Input;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color},
    Component, Event, MockComponent, NoUserEvent,
};

use crate::{
    config::Config,
    tui::{ConfigUpdate, Msg},
};

#[derive(MockComponent)]
pub struct StatePathEditor {
    component: Input,
}

impl StatePathEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("State Path", Alignment::Center)
                .value(
                    config
                        .state_path
                        .clone()
                        .unwrap_or(PathBuf::default())
                        .to_str()
                        .unwrap_or(""),
                ),
        }
    }
}

impl Component<Msg, NoUserEvent> for StatePathEditor {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(Msg::SetPopup(None));
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char(c), ..
            }) => self.perform(Cmd::Type(c)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if !self.component.states.get_value().is_empty() {
                    return Some(Msg::UpdateConfig(
                        ConfigUpdate::StatePath(Some(PathBuf::from(
                            self.component.states.get_value().as_str(),
                        ))),
                        true,
                    ));
                } else {
                    return Some(Msg::UpdateConfig(ConfigUpdate::StatePath(None), true));
                }
            }
            _ => CmdResult::None,
        };
        None
    }
}
