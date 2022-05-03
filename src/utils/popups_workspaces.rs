use std::io::Stdout;

use anyhow::Result;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap};

use crate::utils::tui::PopupState;
use crate::utils::centered_rect;

pub fn text_input(current_popup_state: &mut PopupState, name: String, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title(name);

    let area = centered_rect(60, 4, f.size());

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)].as_ref())
        .split(area);

    let string = if let PopupState::String(s) = current_popup_state {
        s.clone()
    } else {
        "".to_string()
    };

    let text = vec![Spans::from(
        vec![
            Span::raw(string),
        ])
    ];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));
    Ok(())
}
