use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    AttrValue, Attribute, Component, Event, MockComponent, State,
};

use crate::{
    config::{modifier::Modifier, Config},
    tui::{model::UserEvent, ConfigUpdate, Msg},
};

#[derive(MockComponent)]
pub struct MouseKeyEditor {
    component: Table,
    mousekey: Option<Modifier>,
}

impl MouseKeyEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("Mousekey", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .column_spacing(3)
                .widths(&[100])
                .table(Self::build_inner(config)),
            mousekey: config.mousekey.clone(),
        }
    }

    fn build_inner(config: &Config) -> Vec<Vec<TextSpan>> {
        TableBuilder::default()
            .add_col(if config.mousekey.is_none() {
                TextSpan::from("None").underlined()
            } else {
                TextSpan::from("None")
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m)) if m == &"Shift".to_string() => {
                    TextSpan::from("Shift").underlined()
                }
                Some(Modifier::List(m)) if m.contains(&"Shift".to_string()) => {
                    TextSpan::from("Shift").underlined()
                }
                _ => TextSpan::from("Shift"),
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m)) if m == &"Control".to_string() => {
                    TextSpan::from("Control").underlined()
                }
                Some(Modifier::List(m)) if m.contains(&"Control".to_string()) => {
                    TextSpan::from("Control").underlined()
                }
                _ => TextSpan::from("Control"),
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m))
                    if m == &"Alt".to_string() || m == &"Mod1".to_string() =>
                {
                    TextSpan::from("Alt").underlined()
                }
                Some(Modifier::List(m))
                    if m.contains(&"Alt".to_string()) || m.contains(&"Mod1".to_string()) =>
                {
                    TextSpan::from("Alt").underlined()
                }
                _ => TextSpan::from("Alt"),
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m)) if m == &"Mod3".to_string() => {
                    TextSpan::from("Mod3").underlined()
                }
                Some(Modifier::List(m)) if m.contains(&"Mod3".to_string()) => {
                    TextSpan::from("Mod3").underlined()
                }
                _ => TextSpan::from("Mod3"),
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m))
                    if m == &"Super".to_string() || m == &"Mod4".to_string() =>
                {
                    TextSpan::from("Super").underlined()
                }
                Some(Modifier::List(m))
                    if m.contains(&"Super".to_string()) || m.contains(&"Mod4".to_string()) =>
                {
                    TextSpan::from("Super").underlined()
                }
                _ => TextSpan::from("Super"),
            })
            .add_row()
            .add_col(match &config.mousekey {
                Some(Modifier::Single(m)) if m == &"Mod5".to_string() => {
                    TextSpan::from("Mod5").underlined()
                }
                Some(Modifier::List(m)) if m.contains(&"Mod5".to_string()) => {
                    TextSpan::from("Mod5").underlined()
                }
                _ => TextSpan::from("Mod5"),
            })
            .build()
    }

    fn update(&mut self) {
        self.component.attr(
            Attribute::Content,
            AttrValue::Table(
                TableBuilder::default()
                    .add_col(if self.mousekey.is_none() {
                        TextSpan::from("None").underlined()
                    } else {
                        TextSpan::from("None")
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m)) if m == &"Shift".to_string() => {
                            TextSpan::from("Shift").underlined()
                        }
                        Some(Modifier::List(m)) if m.contains(&"Shift".to_string()) => {
                            TextSpan::from("Shift").underlined()
                        }
                        _ => TextSpan::from("Shift"),
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m)) if m == &"Control".to_string() => {
                            TextSpan::from("Control").underlined()
                        }
                        Some(Modifier::List(m)) if m.contains(&"Control".to_string()) => {
                            TextSpan::from("Control").underlined()
                        }
                        _ => TextSpan::from("Control"),
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m))
                            if m == &"Alt".to_string() || m == &"Mod1".to_string() =>
                        {
                            TextSpan::from("Alt").underlined()
                        }
                        Some(Modifier::List(m))
                            if m.contains(&"Alt".to_string())
                                || m.contains(&"Mod1".to_string()) =>
                        {
                            TextSpan::from("Alt").underlined()
                        }
                        _ => TextSpan::from("Alt"),
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m)) if m == &"Mod3".to_string() => {
                            TextSpan::from("Mod3").underlined()
                        }
                        Some(Modifier::List(m)) if m.contains(&"Mod3".to_string()) => {
                            TextSpan::from("Mod3").underlined()
                        }
                        _ => TextSpan::from("Mod3"),
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m))
                            if m == &"Super".to_string() || m == &"Mod4".to_string() =>
                        {
                            TextSpan::from("Super").underlined()
                        }
                        Some(Modifier::List(m))
                            if m.contains(&"Super".to_string())
                                || m.contains(&"Mod4".to_string()) =>
                        {
                            TextSpan::from("Super").underlined()
                        }
                        _ => TextSpan::from("Super"),
                    })
                    .add_row()
                    .add_col(match &self.mousekey {
                        Some(Modifier::Single(m)) if m == &"Mod5".to_string() => {
                            TextSpan::from("Mod5").underlined()
                        }
                        Some(Modifier::List(m)) if m.contains(&"Mod5".to_string()) => {
                            TextSpan::from("Mod5").underlined()
                        }
                        _ => TextSpan::from("Mod5"),
                    })
                    .build(),
            ),
        );
    }

    fn update_mousekey(&mut self, mousekey: String) {
        match &mut self.mousekey {
            Some(Modifier::Single(m)) if *m != mousekey => {
                self.mousekey = Some(Modifier::List(vec![m.clone(), mousekey]))
            }
            Some(Modifier::Single(m)) if *m == mousekey => self.mousekey = None,
            Some(Modifier::List(l)) if !l.contains(&mousekey) => l.push(mousekey),
            Some(Modifier::List(l)) if l.contains(&mousekey) && l.len() > 1 => {
                self.mousekey = Some(Modifier::List(
                    l.iter_mut()
                        .filter(|m| **m != mousekey)
                        .map(|m| m.clone())
                        .collect(),
                ));
            }
            Some(Modifier::List(l)) if l.contains(&mousekey) && l.len() <= 1 => {
                self.mousekey = None;
            }
            Some(_) => unreachable!(),
            None => self.mousekey = Some(Modifier::Single(mousekey)),
        }
    }
}

impl Component<Msg, UserEvent> for MouseKeyEditor {
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
                code: Key::Char(' '),
                ..
            }) => {
                match self.component.states.list_index {
                    0 => self.mousekey = None,
                    1 => self.update_mousekey("Shift".to_string()),
                    2 => self.update_mousekey("Control".to_string()),
                    3 => self.update_mousekey("Alt".to_string()),
                    4 => self.update_mousekey("Mod3".to_string()),
                    5 => self.update_mousekey("Super".to_string()),
                    6 => self.update_mousekey("Mod5".to_string()),
                    _ => unreachable!(),
                }
                self.update();
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                return Some(Msg::UpdateConfig(
                    ConfigUpdate::MouseKey(self.mousekey.clone()),
                    true,
                ));
            }
            Event::User(UserEvent::ConfigUpdate(ConfigUpdate::MouseKey(m))) => {
                self.mousekey = m;
                self.update();
                CmdResult::Changed(State::None)
            }
            _ => CmdResult::None,
        };
        None
    }
}
