use tui_realm_stdlib::Paragraph;
use tuirealm::{
    props::{Alignment, BorderType, Borders, Color, TextSpan},
    Component, MockComponent,
};

use crate::tui::{model::UserEvent, Msg};

#[derive(MockComponent)]
pub struct DocBlock {
    component: Paragraph,
}

impl DocBlock {
    pub fn new(text: &[TextSpan]) -> Self {
        Self {
            component: Paragraph::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .text(text)
                .wrap(true)
                .alignment(Alignment::Left),
        }
    }
}

impl Component<Msg, UserEvent> for DocBlock {
    fn on(&mut self, _ev: tuirealm::Event<UserEvent>) -> Option<Msg> {
        None
    }
}
