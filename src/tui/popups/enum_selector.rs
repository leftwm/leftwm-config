use std::marker::PhantomData;

use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    tui::layout::Rect,
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, State,
};

use crate::{
    config::Config,
    tui::{ConfigUpdate, Msg},
};

pub trait SelectorEnum: Sized + 'static {
    const ALL_VARIANTS: &'static [Self];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate;

    fn variant_name(&self) -> &str;

    fn name<'a>() -> &'a str;

    fn is_enabled(&self, config: &Config) -> bool;
}

pub struct EnumSelector<E: SelectorEnum> {
    component: Table,
    phantom: PhantomData<E>,
}

// Manual impl, since the derive is stupid and doesn't handle the generic
impl<E: SelectorEnum> MockComponent for EnumSelector<E> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.component.view(frame, area);
    }
    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.component.query(attr)
    }
    fn attr(&mut self, query: Attribute, attr: AttrValue) {
        self.component.attr(query, attr)
    }
    fn state(&self) -> State {
        self.component.state()
    }
    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.component.perform(cmd)
    }
}

impl<E: SelectorEnum> EnumSelector<E> {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title(E::name(), Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .column_spacing(3)
                .widths(&[100])
                .table(Self::build_inner(config)),
            phantom: PhantomData,
        }
    }

    fn build_inner(config: &Config) -> Vec<Vec<TextSpan>> {
        let mut builder = TableBuilder::default();
        E::ALL_VARIANTS
            .iter()
            .take(E::ALL_VARIANTS.len() - 1)
            .for_each(|v| {
                builder.add_col(if v.is_enabled(config) {
                    TextSpan::from(v.variant_name()).underlined()
                } else {
                    TextSpan::from(v.variant_name())
                });
                builder.add_row();
            });
        E::ALL_VARIANTS.last().inspect(|v| {
            builder.add_col(if v.is_enabled(config) {
                TextSpan::from(v.variant_name()).underlined()
            } else {
                TextSpan::from(v.variant_name())
            });
        });
        builder.build()
    }
}

impl<E: SelectorEnum + Clone> Component<Msg, NoUserEvent> for EnumSelector<E> {
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
                return Some(Msg::UpdateConfig(
                    E::CONFIG_UPDATE(E::ALL_VARIANTS[self.component.states.list_index].clone()),
                    true,
                ));
            }
            _ => CmdResult::None,
        };
        None
    }
}
