use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap};
use tui::Frame;

use crate::utils::centered_rect;
use crate::utils::tui::PopupState;

pub fn text_input(
    current_popup_state: &mut PopupState,
    name: String,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title(name);

    let area = centered_rect(60, 4, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(area);

    let string = if let PopupState::String(s) = current_popup_state {
        s.clone()
    } else {
        "".to_string()
    };

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));
}

pub fn layouts(current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Layouts");
    let area = centered_rect(60, 20, f.size());

    let mut layout_list = vec![
        ListItem::new("MainAndVertStack"),
        ListItem::new("MainAndHorizontalStack"),
        ListItem::new("MainAndDeck"),
        ListItem::new("GridHorizontal"),
        ListItem::new("EvenHorizontal"),
        ListItem::new("EvenVertical"),
        ListItem::new("Fibonacci"),
        ListItem::new("LeftMain"),
        ListItem::new("CenterMain"),
        ListItem::new("CenterMainBalanced"),
        ListItem::new("CenterMainFluid"),
        ListItem::new("Monocle"),
        ListItem::new("RightWiderLeftStack"),
        ListItem::new("LeftWiderRightStack"),
    ];

    if let PopupState::MultiList(e) = current_popup_state {
        for i in &e.selected {
            match i {
                0 => {
                    layout_list.insert(
                        0,
                        ListItem::new("MainAndVertStack").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(1);
                }
                1 => {
                    layout_list.insert(
                        1,
                        ListItem::new("MainAndHorizontalStack")
                            .style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(2);
                }
                2 => {
                    layout_list.insert(
                        2,
                        ListItem::new("MainAndDeck").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(3);
                }
                3 => {
                    layout_list.insert(
                        3,
                        ListItem::new("GridHorizontal").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(4);
                }
                4 => {
                    layout_list.insert(
                        4,
                        ListItem::new("EvenHorizontal").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(5);
                }
                5 => {
                    layout_list.insert(
                        5,
                        ListItem::new("EvenVertical").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(6);
                }
                6 => {
                    layout_list.insert(
                        6,
                        ListItem::new("Fibonacci").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(7);
                }
                7 => {
                    layout_list.insert(
                        7,
                        ListItem::new("LeftMain").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(8);
                }
                8 => {
                    layout_list.insert(
                        8,
                        ListItem::new("CenterMain").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(9);
                }
                9 => {
                    layout_list.insert(
                        9,
                        ListItem::new("CenterMainBalanced")
                            .style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(10);
                }
                10 => {
                    layout_list.insert(
                        10,
                        ListItem::new("CenterMainFluid").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(11);
                }
                11 => {
                    layout_list.insert(
                        11,
                        ListItem::new("Monocle").style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(12);
                }
                12 => {
                    layout_list.insert(
                        12,
                        ListItem::new("RightWiderLeftStack")
                            .style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(13);
                }
                13 => {
                    layout_list.insert(
                        13,
                        ListItem::new("LeftWiderRightStack")
                            .style(Style::default().fg(Color::Green)),
                    );
                    layout_list.remove(14);
                }
                _ => {}
            }
        }
    }
    let list = List::new(layout_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    if let PopupState::MultiList(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(75, 70, area), &mut e.liststate);
    } else {
        panic!("popup state incorrectly set")
    }
}
