use std::io::Stdout;

use anyhow::Result;

use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph, Wrap};

use crate::Config;
use crate::config::modifier::Modifier::Single;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode};
use crate::utils::tui::PopupState;
use crate::utils::centered_rect;

pub fn modkey(current_config: &Config, current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>, is_mousekey: bool) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title(if !is_mousekey { "Modkey" } else { "Mousekey" });
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    let modkey_list = if !is_mousekey {
        [
            { if current_config.modkey == "None" { ListItem::new("None").style(Style::default().fg(Color::Green)) } else { ListItem::new("None") } },
            { if current_config.modkey == "Shift" { ListItem::new("Shift").style(Style::default().fg(Color::Green)) } else { ListItem::new("Shift") } },
            { if current_config.modkey == "Control" { ListItem::new("Control").style(Style::default().fg(Color::Green)) } else { ListItem::new("Control") } },
            { if current_config.modkey == "Alt" || current_config.modkey == "Mod1" { ListItem::new("Alt").style(Style::default().fg(Color::Green)) } else { ListItem::new("Alt") } },
            { if current_config.modkey == "Mod3" { ListItem::new("Mod3").style(Style::default().fg(Color::Green)) } else { ListItem::new("Mod3") } },
            { if current_config.modkey == "Super" || current_config.modkey == "Mod4" { ListItem::new("Super").style(Style::default().fg(Color::Green)) } else { ListItem::new("Super") } },
            { if current_config.modkey == "Mod5" { ListItem::new("Mod5").style(Style::default().fg(Color::Green)) } else { ListItem::new("Mod5") } },
        ]
    } else {
        [
            { if current_config.mousekey.is_none() { ListItem::new("None").style(Style::default().fg(Color::Green)) } else { ListItem::new("None") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Shift" } else { false } }) { ListItem::new("Shift").style(Style::default().fg(Color::Green)) } else { ListItem::new("Shift") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Control" } else { false } }) { ListItem::new("Control").style(Style::default().fg(Color::Green)) } else { ListItem::new("Control") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Alt" || s == "Mod1" } else { false } }) { ListItem::new("Alt").style(Style::default().fg(Color::Green)) } else { ListItem::new("Alt") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Mod3" } else { false } }) { ListItem::new("Mod3").style(Style::default().fg(Color::Green)) } else { ListItem::new("Mod3") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Super" || s == "Mod4" } else { false } }) { ListItem::new("Super").style(Style::default().fg(Color::Green)) } else { ListItem::new("Super") } },
            { if current_config.mousekey.is_some_and(|m| { if let Single(s) = m { s == "Mod5" } else { false } }) { ListItem::new("Mod5").style(Style::default().fg(Color::Green)) } else { ListItem::new("Mod5") } },
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
        panic!("popup state incorrectly set")
    }

    Ok(())
}

pub fn max_window_width(current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Max Window Width");

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

pub fn focus_behavior(current_config: &Config, current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
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
        { if current_config.focus_behaviour == FocusBehaviour::Sloppy { ListItem::new("Sloppy").style(Style::default().fg(Color::Green)) } else { ListItem::new("Sloppy") } },
        { if current_config.focus_behaviour == FocusBehaviour::ClickTo { ListItem::new("Click To").style(Style::default().fg(Color::Green)) } else { ListItem::new("Click To") } },
        { if current_config.focus_behaviour == FocusBehaviour::Driven { ListItem::new("Driven").style(Style::default().fg(Color::Green)) } else { ListItem::new("Driven") } },
    ];
    let list = List::new(mode_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(30, 50, area), e);
    } else {
        panic!("popup state incorrectly set")
    }

    Ok(())
}

pub fn insert_behavior(current_config: &Config, current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Insert Behavior");
    let area = centered_rect(60, 20, f.size());
    let mode_list = [
        { if current_config.insert_behavior == InsertBehavior::Top { ListItem::new("Top").style(Style::default().fg(Color::Green)) } else { ListItem::new("Top") } },
        { if current_config.insert_behavior == InsertBehavior::Bottom { ListItem::new("Bottom").style(Style::default().fg(Color::Green)) } else { ListItem::new("Bottom") } },
        { if current_config.insert_behavior == InsertBehavior::BeforeCurrent { ListItem::new("Before Current").style(Style::default().fg(Color::Green)) } else { ListItem::new("Before Current") } },
        { if current_config.insert_behavior == InsertBehavior::AfterCurrent { ListItem::new("After Current").style(Style::default().fg(Color::Green)) } else { ListItem::new("After Current") } },
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
        panic!("popup state incorrectly set")
    }

    Ok(())
}


pub fn layout_mode(current_config: &Config, current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Layout Mode");
    let area = centered_rect(60, 20, f.size());
    let mode_list = [
        { if current_config.layout_mode == LayoutMode::Tag { ListItem::new("Tag").style(Style::default().fg(Color::Green)) } else { ListItem::new("Tag") } },
        { if current_config.layout_mode == LayoutMode::Workspace { ListItem::new("Workspace").style(Style::default().fg(Color::Green)) } else { ListItem::new("Workspace") } },
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
        panic!("popup state incorrectly set")
    }
    Ok(())
}

pub fn layouts(current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
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
                    layout_list.insert(0, ListItem::new("MainAndVertStack").style(Style::default().fg(Color::Green)));
                    layout_list.remove(1);
                }
                1 => {
                    layout_list.insert(1, ListItem::new("MainAndHorizontalStack").style(Style::default().fg(Color::Green)));
                    layout_list.remove(2);
                }
                2 => {
                    layout_list.insert(2, ListItem::new("MainAndDeck").style(Style::default().fg(Color::Green)));
                    layout_list.remove(3);
                }
                3 => {
                    layout_list.insert(3, ListItem::new("GridHorizontal").style(Style::default().fg(Color::Green)));
                    layout_list.remove(4);
                }
                4 => {
                    layout_list.insert(4, ListItem::new("EvenHorizontal").style(Style::default().fg(Color::Green)));
                    layout_list.remove(5);
                }
                5 => {
                    layout_list.insert(5, ListItem::new("EvenVertical").style(Style::default().fg(Color::Green)));
                    layout_list.remove(6);
                }
                6 => {
                    layout_list.insert(6, ListItem::new("Fibonacci").style(Style::default().fg(Color::Green)));
                    layout_list.remove(7);
                }
                7 => {
                    layout_list.insert(7, ListItem::new("LeftMain").style(Style::default().fg(Color::Green)));
                    layout_list.remove(8);
                }
                8 => {
                    layout_list.insert(8, ListItem::new("CenterMain").style(Style::default().fg(Color::Green)));
                    layout_list.remove(9);
                }
                9 => {
                    layout_list.insert(9, ListItem::new("CenterMainBalanced").style(Style::default().fg(Color::Green)));
                    layout_list.remove(10);
                }
                10 => {
                    layout_list.insert(10, ListItem::new("CenterMainFluid").style(Style::default().fg(Color::Green)));
                    layout_list.remove(11);
                }
                11 => {
                    layout_list.insert(11, ListItem::new("Monocle").style(Style::default().fg(Color::Green)));
                    layout_list.remove(12);
                }
                12 => {
                    layout_list.insert(12, ListItem::new("RightWiderLeftStack").style(Style::default().fg(Color::Green)));
                    layout_list.remove(13);
                }
                13 => {
                    layout_list.insert(13, ListItem::new("LeftWiderRightStack").style(Style::default().fg(Color::Green)));
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

    Ok(())
}

pub fn workspaces(current_config: &mut Config, current_popup_state: &mut PopupState, f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title({
            if let PopupState::MultistructState(s) = current_popup_state {
                if s.items_list_state.selected().is_some_and(|i| {*i == 0_usize}){
                    "Workspaces ->"
                } else if s.items_list_state.selected().is_some_and(|i| {*i == s.items - 1}) {
                    "<- Workspaces"
                } else {
                    "<- Workspaces ->"
                }
            } else {
              panic!("State is set incorrectly")
            }
        });
    let area = centered_rect(60, 20, f.size());

    let current_workspace = if let Some(w) = &mut current_config.workspaces {
        w.get(if let PopupState::MultistructState(s) = current_popup_state { s.items_list_state.selected().unwrap_or(0) } else { 0 })
    } else {
        None
    };

    let field_list = if let Some(c) = current_workspace {
        vec![
            ListItem::new(format!("X - {}", c.x)),
            ListItem::new(format!("Y - {}", c.y)),
            ListItem::new(format!("Widht - {}", c.width)),
            ListItem::new(format!("Height - {}", c.height)),
            ListItem::new(format!("Id - {:?}", c.id)),
            ListItem::new(format!("Max Window Width - {:?}", c.max_window_width)),
            ListItem::new(format!("Layouts - {:?}", c.layouts)),
        ]
    } else {
        vec![
            ListItem::new("X"),
            ListItem::new("Y"),
            ListItem::new("Widht"),
            ListItem::new("Height"),
            ListItem::new("Id"),
            ListItem::new("Max Window Width"),
            ListItem::new("Layouts"),
        ]
    };

    let list = List::new(field_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");


    f.render_widget(Clear, area);
    f.render_widget(block, area);
    if let PopupState::MultistructState(s) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(75, 70, area), &mut s.fields_list_state);
    } else {
        panic!("wrong state")
    }

    Ok(())
}

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
