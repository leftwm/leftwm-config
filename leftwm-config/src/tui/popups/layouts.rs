use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    AttrValue, Attribute, Component, Event, MockComponent, NoUserEvent,
};

use crate::tui::{ConfigUpdate, Msg};
use leftwm_config_core::{Config, Layout};

#[derive(MockComponent)]
pub struct LayoutsEditor {
    component: Table,
    layouts: Vec<Layout>,
}

impl LayoutsEditor {
    pub fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("Layouts", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .column_spacing(3)
                .widths(&[100])
                .table(Self::build_inner(config)),
            layouts: config.layouts.clone(),
        }
    }

    fn build_inner(config: &Config) -> Vec<Vec<TextSpan>> {
        TableBuilder::default()
            .add_col(if config.layouts.contains(&Layout::MainAndVertStack) {
                TextSpan::from("MainAndVertStack").underlined()
            } else {
                TextSpan::from("MainAndVertStack")
            })
            .add_row()
            .add_col(
                if config.layouts.contains(&Layout::MainAndHorizontalStack) {
                    TextSpan::from("MainAndHorizontalStack").underlined()
                } else {
                    TextSpan::from("MainAndHorizontalStack")
                },
            )
            .add_row()
            .add_col(if config.layouts.contains(&Layout::MainAndDeck) {
                TextSpan::from("MainAndDeck").underlined()
            } else {
                TextSpan::from("MainAndDeck")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::GridHorizontal) {
                TextSpan::from("GridHorizontal").underlined()
            } else {
                TextSpan::from("GridHorizontal")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::EvenHorizontal) {
                TextSpan::from("EvenHorizontal").underlined()
            } else {
                TextSpan::from("EvenHorizontal")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::EvenVertical) {
                TextSpan::from("EvenVertical").underlined()
            } else {
                TextSpan::from("EvenVertical")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::Fibonacci) {
                TextSpan::from("Fibonacci").underlined()
            } else {
                TextSpan::from("Fibonacci")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::LeftMain) {
                TextSpan::from("LeftMain").underlined()
            } else {
                TextSpan::from("LeftMain")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::CenterMain) {
                TextSpan::from("CenterMain").underlined()
            } else {
                TextSpan::from("CenterMain")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::CenterMainBalanced) {
                TextSpan::from("CenterMainBalanced").underlined()
            } else {
                TextSpan::from("CenterMainBalanced")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::CenterMainFluid) {
                TextSpan::from("CenterMainFluid").underlined()
            } else {
                TextSpan::from("CenterMainFluid")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::Monocle) {
                TextSpan::from("Monocle").underlined()
            } else {
                TextSpan::from("Monocle")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::RightWiderLeftStack) {
                TextSpan::from("RightWiderLeftStack").underlined()
            } else {
                TextSpan::from("RightWiderLeftStack")
            })
            .add_row()
            .add_col(if config.layouts.contains(&Layout::LeftWiderRightStack) {
                TextSpan::from("LeftWiderRightStack").underlined()
            } else {
                TextSpan::from("LeftWiderRightStack")
            })
            .build()
    }

    fn update(&mut self) {
        self.component.attr(
            Attribute::Content,
            AttrValue::Table(
                TableBuilder::default()
                    .add_col(if self.layouts.contains(&Layout::MainAndVertStack) {
                        TextSpan::from("MainAndVertStack").underlined()
                    } else {
                        TextSpan::from("MainAndVertStack")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::MainAndHorizontalStack) {
                        TextSpan::from("MainAndHorizontalStack").underlined()
                    } else {
                        TextSpan::from("MainAndHorizontalStack")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::MainAndDeck) {
                        TextSpan::from("MainAndDeck").underlined()
                    } else {
                        TextSpan::from("MainAndDeck")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::GridHorizontal) {
                        TextSpan::from("GridHorizontal").underlined()
                    } else {
                        TextSpan::from("GridHorizontal")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::EvenHorizontal) {
                        TextSpan::from("EvenHorizontal").underlined()
                    } else {
                        TextSpan::from("EvenHorizontal")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::EvenVertical) {
                        TextSpan::from("EvenVertical").underlined()
                    } else {
                        TextSpan::from("EvenVertical")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::Fibonacci) {
                        TextSpan::from("Fibonacci").underlined()
                    } else {
                        TextSpan::from("Fibonacci")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::LeftMain) {
                        TextSpan::from("LeftMain").underlined()
                    } else {
                        TextSpan::from("LeftMain")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::CenterMain) {
                        TextSpan::from("CenterMain").underlined()
                    } else {
                        TextSpan::from("CenterMain")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::CenterMainBalanced) {
                        TextSpan::from("CenterMainBalanced").underlined()
                    } else {
                        TextSpan::from("CenterMainBalanced")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::CenterMainFluid) {
                        TextSpan::from("CenterMainFluid").underlined()
                    } else {
                        TextSpan::from("CenterMainFluid")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::Monocle) {
                        TextSpan::from("Monocle").underlined()
                    } else {
                        TextSpan::from("Monocle")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::RightWiderLeftStack) {
                        TextSpan::from("RightWiderLeftStack").underlined()
                    } else {
                        TextSpan::from("RightWiderLeftStack")
                    })
                    .add_row()
                    .add_col(if self.layouts.contains(&Layout::LeftWiderRightStack) {
                        TextSpan::from("LeftWiderRightStack").underlined()
                    } else {
                        TextSpan::from("LeftWiderRightStack")
                    })
                    .build(),
            ),
        );
    }

    fn update_layouts(&mut self, layout: Layout) {
        if self.layouts.contains(&layout) {
            self.layouts = self
                .layouts
                .clone()
                .into_iter()
                .filter(|l| *l != layout)
                .collect();
        } else {
            self.layouts.push(layout);
        }
    }
}

impl Component<Msg, NoUserEvent> for LayoutsEditor {
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
                code: Key::Char(' '),
                ..
            }) => {
                match self.component.states.list_index {
                    0 => self.update_layouts(Layout::MainAndVertStack),
                    1 => self.update_layouts(Layout::MainAndHorizontalStack),
                    2 => self.update_layouts(Layout::MainAndDeck),
                    3 => self.update_layouts(Layout::GridHorizontal),
                    4 => self.update_layouts(Layout::EvenHorizontal),
                    5 => self.update_layouts(Layout::EvenVertical),
                    6 => self.update_layouts(Layout::Fibonacci),
                    7 => self.update_layouts(Layout::LeftMain),
                    8 => self.update_layouts(Layout::CenterMain),
                    9 => self.update_layouts(Layout::CenterMainBalanced),
                    10 => self.update_layouts(Layout::CenterMainFluid),
                    11 => self.update_layouts(Layout::Monocle),
                    12 => self.update_layouts(Layout::RightWiderLeftStack),
                    13 => self.update_layouts(Layout::LeftWiderRightStack),
                    _ => unreachable!(),
                }
                self.update();
                CmdResult::None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                return Some(Msg::UpdateConfig(
                    ConfigUpdate::Layouts(self.layouts.clone()),
                    true,
                ));
            }
            _ => CmdResult::None,
        };
        None
    }
}
