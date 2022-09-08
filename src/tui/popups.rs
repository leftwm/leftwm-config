use std::io::Stdout;
use std::mem;

use anyhow::{bail, Result};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap};
use tui::Frame;

use crate::config::modifier::Modifier as KeyModifier;
use crate::config::modifier::Modifier::Single;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode};
use crate::config::Config;
use crate::tui::PopupState;
use crate::utils::{centered_rect, AnyhowUnwrap};

pub fn modkey(
    current_config: &Config,
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
    is_mousekey: bool,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title(if is_mousekey { "Mousekey" } else { "Modkey" });
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    let modkey_list = if is_mousekey {
        [
            if current_config.mousekey.is_none() {
                ListItem::new("None").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("None")
            },
            if check_modifier(&current_config.mousekey, "Shift") {
                ListItem::new("Shift").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Shift")
            },
            if check_modifier(&current_config.mousekey, "Control") {
                ListItem::new("Control").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Control")
            },
            if check_modifier(&current_config.mousekey, "Mod1")
                || check_modifier(&current_config.mousekey, "Alt")
            {
                ListItem::new("Alt").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Alt")
            },
            if check_modifier(&current_config.mousekey, "Mod3") {
                ListItem::new("Mod3").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Mod3")
            },
            if check_modifier(&current_config.mousekey, "Mod4")
                || check_modifier(&current_config.mousekey, "Super")
            {
                ListItem::new("Super").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Super")
            },
            if check_modifier(&current_config.mousekey, "Mod5") {
                ListItem::new("Mod5").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Mod5")
            },
        ]
    } else {
        [
            if current_config.modkey == "None" {
                ListItem::new("None").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("None")
            },
            if current_config.modkey == "Shift" {
                ListItem::new("Shift").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Shift")
            },
            if current_config.modkey == "Control" {
                ListItem::new("Control").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Control")
            },
            if current_config.modkey == "Alt" || current_config.modkey == "Mod1" {
                ListItem::new("Alt").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Alt")
            },
            if current_config.modkey == "Mod3" {
                ListItem::new("Mod3").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Mod3")
            },
            if current_config.modkey == "Super" || current_config.modkey == "Mod4" {
                ListItem::new("Super").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Super")
            },
            if current_config.modkey == "Mod5" {
                ListItem::new("Mod5").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Mod5")
            },
        ]
    };
    let list = List::new(modkey_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(30, 50, area), e);
    } else {
        bail!("Invalid popup state");
    }

    Ok(())
}

fn check_modifier(modifier: &Option<KeyModifier>, name: &str) -> bool {
    modifier.is_some_and(|m| if let Single(s) = m { *s == name } else { false })
}

pub fn max_window_width(
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Max Window Width");

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
        bail!("Invalid popup state");
    };

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));

    Ok(())
}

pub fn focus_behavior(
    current_config: &Config,
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Focus Behavior");
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    let mode_list = [
        {
            if current_config.focus_behaviour == FocusBehaviour::Sloppy {
                ListItem::new("Sloppy").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Sloppy")
            }
        },
        {
            if current_config.focus_behaviour == FocusBehaviour::ClickTo {
                ListItem::new("Click To").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Click To")
            }
        },
        {
            if current_config.focus_behaviour == FocusBehaviour::Driven {
                ListItem::new("Driven").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Driven")
            }
        },
    ];
    let list = List::new(mode_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(30, 50, area), e);
    } else {
        bail!("Invalid popup state");
    }

    Ok(())
}

pub fn insert_behavior(
    current_config: &Config,
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Insert Behavior");
    let area = centered_rect(60, 20, f.size());
    let mode_list = [
        {
            if current_config.insert_behavior == InsertBehavior::Top {
                ListItem::new("Top").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Top")
            }
        },
        {
            if current_config.insert_behavior == InsertBehavior::Bottom {
                ListItem::new("Bottom").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Bottom")
            }
        },
        {
            if current_config.insert_behavior == InsertBehavior::BeforeCurrent {
                ListItem::new("Before Current").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Before Current")
            }
        },
        {
            if current_config.insert_behavior == InsertBehavior::AfterCurrent {
                ListItem::new("After Current").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("After Current")
            }
        },
    ];
    let list = List::new(mode_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(30, 50, area), e);
    } else {
        bail!("Invalid popup state");
    }

    Ok(())
}

pub fn layout_mode(
    current_config: &Config,
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Layout Mode");
    let area = centered_rect(60, 20, f.size());
    let mode_list = [
        {
            if current_config.layout_mode == LayoutMode::Tag {
                ListItem::new("Tag").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Tag")
            }
        },
        {
            if current_config.layout_mode == LayoutMode::Workspace {
                ListItem::new("Workspace").style(Style::default().fg(Color::Green))
            } else {
                ListItem::new("Workspace")
            }
        },
    ];
    let list = List::new(mode_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(30, 50, area), e);
    } else {
        bail!("Invalid popup state");
    }

    Ok(())
}

pub fn layouts(
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
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
            // we allow this here because clippy thinks
            // we are initializing a new thing here (probably because of the _ => {..})
            // while we are just using the let _ to get rid of the result of mem::replace()
            #[allow(clippy::let_underscore_drop)]
            let _ = match i {
                0 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(0).unwrap_anyhow()?,
                    ListItem::new("MainAndVertStack").style(Style::default().fg(Color::Green)),
                ),
                1 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(1).unwrap_anyhow()?,
                    ListItem::new("MainAndHorizontalStack")
                        .style(Style::default().fg(Color::Green)),
                ),
                2 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(2).unwrap_anyhow()?,
                    ListItem::new("MainAndDeck").style(Style::default().fg(Color::Green)),
                ),
                3 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(3).unwrap_anyhow()?,
                    ListItem::new("GridHorizontal").style(Style::default().fg(Color::Green)),
                ),
                4 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(4).unwrap_anyhow()?,
                    ListItem::new("EvenHorizontal").style(Style::default().fg(Color::Green)),
                ),
                5 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(5).unwrap_anyhow()?,
                    ListItem::new("EvenVertical").style(Style::default().fg(Color::Green)),
                ),
                6 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(6).unwrap_anyhow()?,
                    ListItem::new("Fibonacci").style(Style::default().fg(Color::Green)),
                ),
                7 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(7).unwrap_anyhow()?,
                    ListItem::new("LeftMain").style(Style::default().fg(Color::Green)),
                ),
                8 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(8).unwrap_anyhow()?,
                    ListItem::new("CenterMain").style(Style::default().fg(Color::Green)),
                ),
                9 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(9).unwrap_anyhow()?,
                    ListItem::new("CenterMainBalanced").style(Style::default().fg(Color::Green)),
                ),
                10 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(10).unwrap_anyhow()?,
                    ListItem::new("CenterMainFluid").style(Style::default().fg(Color::Green)),
                ),
                11 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(11).unwrap_anyhow()?,
                    ListItem::new("Monocle").style(Style::default().fg(Color::Green)),
                ),
                12 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(12).unwrap_anyhow()?,
                    ListItem::new("RightWiderLeftStack").style(Style::default().fg(Color::Green)),
                ),
                13 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(13).unwrap_anyhow()?,
                    ListItem::new("LeftWiderRightStack").style(Style::default().fg(Color::Green)),
                ),
                _ => ListItem::new(""),
            };
        }
    } else {
        bail!("Invalid popup state");
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
        bail!("Invalid popup state");
    }

    Ok(())
}

//we allow this case if saves having an explicit `Ok(())` after every call to this function
#[allow(clippy::unnecessary_wraps)]
pub fn saved(f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    let mut area = centered_rect(60, 20, f.size());
    area.height = 3;

    let text = vec![Spans::from(Span::raw("Saved"))];

    let message = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    area.y += 1;
    f.render_widget(message, area);

    Ok(())
}

pub fn text_input(
    current_popup_state: &mut PopupState,
    name: String,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
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
        bail!("Invalid popup state")
    };

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));

    Ok(())
}

pub fn counter(
    current_popup_state: &mut PopupState,
    name: String,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
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

    let string = if let PopupState::Int { current, min, max } = current_popup_state {
        if current <= min {
            format!("  {} >", (*current))
        } else if current >= max {
            format!("< {}  ", (*current))
        } else {
            format!("< {} >", (*current))
        }
    } else {
        bail!("Invalid popup state")
    };

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));

    Ok(())
}
