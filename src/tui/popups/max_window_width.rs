use tui_realm_stdlib::{Input, Paragraph};
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TextSpan},
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent,
};

use std::str::FromStr;

use crate::{
    config::{values::Size, Config},
    tui::{ConfigUpdate, Msg},
};

#[derive(MockComponent)]
pub struct MaxWindowWidthEditor {
    component: Input,
}

impl MaxWindowWidthEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("Max Window Width", Alignment::Center)
                .value(format_max_window_width(config.max_window_width)),
        }
    }
}

impl Component<Msg, NoUserEvent> for MaxWindowWidthEditor {
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
            }) => {
                if "0123456789,.%".contains(c) {
                    self.perform(Cmd::Type(c));
                    match parse_string_to_max_window_width(self.component.states.get_value()) {
                        Ok(_) => {
                            self.component.attr(
                                Attribute::Borders,
                                AttrValue::Borders(
                                    Borders::default()
                                        .modifiers(BorderType::Rounded)
                                        .color(Color::White),
                                ),
                            );
                        }
                        Err(_) => {
                            self.component.attr(
                                Attribute::Borders,
                                AttrValue::Borders(
                                    Borders::default()
                                        .modifiers(BorderType::Rounded)
                                        .color(Color::Red),
                                ),
                            );
                        }
                    };
                }
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                if let Ok(s) = parse_string_to_max_window_width(self.component.states.get_value()) {
                    return Some(Msg::UpdateConfig(ConfigUpdate::MaxWindowWidth(s), true));
                } else {
                    return Some(Msg::SetPopup(None));
                }
            }
            _ => CmdResult::None,
        };
        None
    }
}

#[derive(MockComponent)]
pub struct MaxWindowWidthHint {
    component: Paragraph,
}

impl MaxWindowWidthHint {
    pub fn new() -> Self {
        Self {
            component: Paragraph::default()
            .text(&[
                    TextSpan::new("Enter: Save"),
                    TextSpan::new("A red border indicates and invalid input. An empty value unsets the max window width."),
                    TextSpan::new("LeftWM-Config will try to parse the entered value as either a fraction between 0 and 1, a percentage (if ending in a pecent sign) or as an absolute value."),
                    TextSpan::new("You can configure a max_window_width to limit the width of the tiled windows (or rather, the width of columns in a layout). This feature comes in handy when working on ultra-wide monitors where you don't want a single window to take the complete workspace width.")
                ])
        }
    }
}

impl Component<Msg, NoUserEvent> for MaxWindowWidthHint {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

fn format_max_window_width(mww: Option<Size>) -> String {
    match mww {
        Some(Size::Pixel(s)) => s.to_string(),
        Some(Size::Ratio(s)) => s.to_string(),
        None => "".to_string(),
    }
}

fn parse_string_to_max_window_width(mww: String) -> Result<Option<Size>, String> {
    if mww.is_empty() {
        Ok(None)
    } else if mww.contains('.') {
        let number = f32::from_str(mww.as_str()).map_err(|e| format!("{}", e))?;
        if !(0.0..=1.0).contains(&number) {
            return Err("Ratio should be between 0 and 1".to_string());
        }
        Ok(Some(Size::Ratio(number)))
    } else if mww.clone().pop() == Some('%') {
        let number = i32::from_str(mww.replace('%', "").as_str()).map_err(|e| format!("{}", e))?;
        if !(0..=100).contains(&number) {
            return Err("Percentages should be between 0 and 100".to_string());
        }
        Ok(Some(Size::Ratio(number as f32 / 100.0)))
    } else {
        let number = i32::from_str(mww.as_str()).map_err(|e| format!("{}", e))?;
        Ok(Some(Size::Pixel(number)))
    }
}
