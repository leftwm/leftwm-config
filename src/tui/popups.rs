use std::io::Stdout;
use std::mem;

use anyhow::{bail, Result};
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap};
use tui::Frame;

use crate::config::command::BaseCommand;
use crate::config::modifier::Modifier as KeyModifier;
use crate::config::modifier::Modifier::Single;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode};
use crate::config::Config;
use crate::tui::PopupState;
use crate::utils::xkeysym_lookup::into_keysym;
use crate::utils::{centered_rect, TryUnwrap};

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
                    layout_list.get_mut(0).try_unwrap()?,
                    ListItem::new("MainAndVertStack").style(Style::default().fg(Color::Green)),
                ),
                1 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(1).try_unwrap()?,
                    ListItem::new("MainAndHorizontalStack")
                        .style(Style::default().fg(Color::Green)),
                ),
                2 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(2).try_unwrap()?,
                    ListItem::new("MainAndDeck").style(Style::default().fg(Color::Green)),
                ),
                3 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(3).try_unwrap()?,
                    ListItem::new("GridHorizontal").style(Style::default().fg(Color::Green)),
                ),
                4 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(4).try_unwrap()?,
                    ListItem::new("EvenHorizontal").style(Style::default().fg(Color::Green)),
                ),
                5 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(5).try_unwrap()?,
                    ListItem::new("EvenVertical").style(Style::default().fg(Color::Green)),
                ),
                6 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(6).try_unwrap()?,
                    ListItem::new("Fibonacci").style(Style::default().fg(Color::Green)),
                ),
                7 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(7).try_unwrap()?,
                    ListItem::new("LeftMain").style(Style::default().fg(Color::Green)),
                ),
                8 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(8).try_unwrap()?,
                    ListItem::new("CenterMain").style(Style::default().fg(Color::Green)),
                ),
                9 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(9).try_unwrap()?,
                    ListItem::new("CenterMainBalanced").style(Style::default().fg(Color::Green)),
                ),
                10 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(10).try_unwrap()?,
                    ListItem::new("CenterMainFluid").style(Style::default().fg(Color::Green)),
                ),
                11 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(11).try_unwrap()?,
                    ListItem::new("Monocle").style(Style::default().fg(Color::Green)),
                ),
                12 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(12).try_unwrap()?,
                    ListItem::new("RightWiderLeftStack").style(Style::default().fg(Color::Green)),
                ),
                13 => mem::replace::<ListItem<'_>>(
                    layout_list.get_mut(13).try_unwrap()?,
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

//we allow this case if it saves having an explicit `Ok(())` after every call to this function
#[allow(clippy::unnecessary_wraps)]
pub fn saved(f: &mut Frame<CrosstermBackend<Stdout>>) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    let mut area = centered_rect(60, 4, f.size());
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

    let text_len = if string.len() % 2 == 0 {
        string.len()
    } else {
        string.len() + 1
    } as u16;

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));

    f.set_cursor(
        area.x + area.width / 2 + text_len / 2,
        area.y + area.height / 2,
    );

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

pub fn keybind_command(
    current_config: &Config,
    index: usize,
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Command");
    let area = centered_rect(60, 20, f.size());
    let mut command_list = [
        ListItem::new("Execute"),
        ListItem::new("CloseWindow"),
        ListItem::new("SwapTags"),
        ListItem::new("SoftReload"),
        ListItem::new("HardReload"),
        ListItem::new("ToggleScratchPad"),
        ListItem::new("ToggleFullScreen"),
        ListItem::new("ToggleSticky"),
        ListItem::new("GotoTag"),
        ListItem::new("ReturnToLastTag"),
        ListItem::new("FloatingToTile"),
        ListItem::new("TileToFloating"),
        ListItem::new("ToggleFloating"),
        ListItem::new("MoveWindowUp"),
        ListItem::new("MoveWindowDown"),
        ListItem::new("MoveWindowTop"),
        ListItem::new("FocusNextTag"),
        ListItem::new("FocusPreviousTag"),
        ListItem::new("FocusWindow"),
        ListItem::new("FocusWindowUp"),
        ListItem::new("FocusWindowDown"),
        ListItem::new("FocusWindowTop"),
        ListItem::new("FocusWorkspaceNext"),
        ListItem::new("FocusWorkspacePrevious"),
        ListItem::new("MoveToTag"),
        ListItem::new("MoveToLastWorkspace"),
        ListItem::new("MoveWindowToNextWorkspace"),
        ListItem::new("MoveWindowToPreviousWorkspace"),
        ListItem::new("MouseMoveWindow"),
        ListItem::new("NextLayout"),
        ListItem::new("PreviousLayout"),
        ListItem::new("SetLayout"),
        ListItem::new("RotateTag"),
        ListItem::new("IncreaseMainWidth"),
        ListItem::new("DecreaseMainWidth"),
        ListItem::new("SetMarginMultiplier"),
        ListItem::new("UnloadTheme"),
        ListItem::new("LoadTheme"),
        ListItem::new("CloseAllOtherWindows"),
    ];

    match current_config.keybind.get(index).try_unwrap()?.command {
        BaseCommand::Execute => mem::replace::<ListItem>(
            &mut command_list[0],
            ListItem::new("Execute").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::CloseWindow => mem::replace::<ListItem>(
            &mut command_list[1],
            ListItem::new("CloseWindow").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::SwapTags => mem::replace::<ListItem>(
            &mut command_list[2],
            ListItem::new("SwapTags").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::SoftReload => mem::replace::<ListItem>(
            &mut command_list[3],
            ListItem::new("SoftReload").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::HardReload => mem::replace::<ListItem>(
            &mut command_list[4],
            ListItem::new("HardReload").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::ToggleScratchPad => mem::replace::<ListItem>(
            &mut command_list[5],
            ListItem::new("ToggleScratchPad").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::ToggleFullScreen => mem::replace::<ListItem>(
            &mut command_list[6],
            ListItem::new("ToggleFullScreen").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::ToggleSticky => mem::replace::<ListItem>(
            &mut command_list[7],
            ListItem::new("ToggleSticky").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::GotoTag => mem::replace::<ListItem>(
            &mut command_list[8],
            ListItem::new("GotoTag").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::ReturnToLastTag => mem::replace::<ListItem>(
            &mut command_list[9],
            ListItem::new("ReturnToLastTag").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FloatingToTile => mem::replace::<ListItem>(
            &mut command_list[10],
            ListItem::new("FloatingToTile").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::TileToFloating => mem::replace::<ListItem>(
            &mut command_list[11],
            ListItem::new("TileToFloating").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::ToggleFloating => mem::replace::<ListItem>(
            &mut command_list[12],
            ListItem::new("ToggleFloating").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveWindowUp => mem::replace::<ListItem>(
            &mut command_list[13],
            ListItem::new("MoveWindowUp").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveWindowDown => mem::replace::<ListItem>(
            &mut command_list[14],
            ListItem::new("MoveWindowDown").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveWindowTop => mem::replace::<ListItem>(
            &mut command_list[15],
            ListItem::new("MoveWindowTop ").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusNextTag => mem::replace::<ListItem>(
            &mut command_list[16],
            ListItem::new("FocusNextTag ").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusPreviousTag => mem::replace::<ListItem>(
            &mut command_list[17],
            ListItem::new("FocusPreviousTag").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWindow => mem::replace::<ListItem>(
            &mut command_list[18],
            ListItem::new("FocusWindow").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWindowUp => mem::replace::<ListItem>(
            &mut command_list[19],
            ListItem::new("FocusWindowUp").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWindowDown => mem::replace::<ListItem>(
            &mut command_list[20],
            ListItem::new("FocusWindowDown").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWindowTop => mem::replace::<ListItem>(
            &mut command_list[21],
            ListItem::new("FocusWindowTop ").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWorkspaceNext => mem::replace::<ListItem>(
            &mut command_list[22],
            ListItem::new("FocusWorkspaceNext").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::FocusWorkspacePrevious => mem::replace::<ListItem>(
            &mut command_list[23],
            ListItem::new("FocusWorkspacePrevious").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveToTag => mem::replace::<ListItem>(
            &mut command_list[24],
            ListItem::new("MoveToTag").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveToLastWorkspace => mem::replace::<ListItem>(
            &mut command_list[25],
            ListItem::new("MoveToLastWorkspace").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveWindowToNextWorkspace => mem::replace::<ListItem>(
            &mut command_list[26],
            ListItem::new("MoveWindowToNextWorkspace ").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MoveWindowToPreviousWorkspace => mem::replace::<ListItem>(
            &mut command_list[27],
            ListItem::new("MoveWindowToPreviousWorkspace").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::MouseMoveWindow => mem::replace::<ListItem>(
            &mut command_list[28],
            ListItem::new("MouseMoveWindow").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::NextLayout => mem::replace::<ListItem>(
            &mut command_list[29],
            ListItem::new("NextLayout").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::PreviousLayout => mem::replace::<ListItem>(
            &mut command_list[30],
            ListItem::new("PreviousLayout").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::SetLayout => mem::replace::<ListItem>(
            &mut command_list[31],
            ListItem::new("SetLayout").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::RotateTag => mem::replace::<ListItem>(
            &mut command_list[32],
            ListItem::new("RotateTag").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::IncreaseMainWidth => mem::replace::<ListItem>(
            &mut command_list[33],
            ListItem::new("IncreaseMainWidth").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::DecreaseMainWidth => mem::replace::<ListItem>(
            &mut command_list[34],
            ListItem::new("DecreaseMainWidth").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::SetMarginMultiplier => mem::replace::<ListItem>(
            &mut command_list[35],
            ListItem::new("SetMarginMultiplier").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::UnloadTheme => mem::replace::<ListItem>(
            &mut command_list[36],
            ListItem::new("UnloadTheme").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::LoadTheme => mem::replace::<ListItem>(
            &mut command_list[37],
            ListItem::new("LoadTheme").style(Style::default().fg(Color::Green)),
        ),
        BaseCommand::CloseAllOtherWindows => mem::replace::<ListItem>(
            &mut command_list[38],
            ListItem::new("CloseAllOtherWindows").style(Style::default().fg(Color::Green)),
        ),
    };

    let list = List::new(command_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);

    if let PopupState::List(e) = current_popup_state {
        f.render_stateful_widget(list, centered_rect(60, 50, area), e);
    } else {
        bail!("Invalid popup state");
    }

    Ok(())
}

pub fn keybind_modkey(
    current_popup_state: &mut PopupState,
    f: &mut Frame<CrosstermBackend<Stdout>>,
) -> Result<()> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black))
        .title("Modifier");
    let area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    let state = if let PopupState::MultiList(s) = current_popup_state {
        s
    } else {
        bail!("Invalid popup state")
    };
    let modkey_list = [
        if state.selected.contains(&0) {
            ListItem::new("None").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("None")
        },
        if state.selected.contains(&1) {
            ListItem::new("Shift").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Shift")
        },
        if state.selected.contains(&2) {
            ListItem::new("Control").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Control")
        },
        if state.selected.contains(&3) {
            ListItem::new("Alt").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Alt")
        },
        if state.selected.contains(&4) {
            ListItem::new("Mod3").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Mod3")
        },
        if state.selected.contains(&5) {
            ListItem::new("Super").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Super")
        },
        if state.selected.contains(&6) {
            ListItem::new("Mod5").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("Mod5")
        },
        if state.selected.contains(&7) {
            ListItem::new("modkey").style(Style::default().fg(Color::Green))
        } else {
            ListItem::new("modkey")
        },
    ];
    let list = List::new(modkey_list)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, centered_rect(30, 50, area), &mut state.liststate);

    Ok(())
}

pub fn keybind_key(
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

    let mut area = centered_rect(60, 4, f.size());

    area.y -= area.height / 2;

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

    let text_len = if string.len() % 2 == 0 {
        string.len()
    } else {
        string.len() + 1
    } as u16;

    let text = vec![Spans::from(vec![Span::raw(string)])];

    let text = Paragraph::new(text)
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area); //this clears out the background
    f.render_widget(block, area);
    f.render_widget(text, *chunks.get(1).unwrap_or(&area));

    f.set_cursor(
        area.x + area.width / 2 + text_len / 2,
        area.y + area.height / 2,
    );

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));

    let mut indicator_area = centered_rect(60, 4, f.size());
    indicator_area.y += area.height;

    let chunks_indicator = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ]
            .as_ref(),
        )
        .split(indicator_area);

    let indicator = if let PopupState::String(s) = current_popup_state {
        if into_keysym(s.as_str()).is_some() {
            let text = vec![Spans::from(vec![Span::raw("Key Ok")])];
            block = block.border_style(Style::default().fg(Color::Green));
            Paragraph::new(text)
                .style(Style::default().fg(Color::Green).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
        } else {
            let text = vec![Spans::from(vec![Span::raw("Key Doesn't Exist!")])];
            block = block.border_style(Style::default().fg(Color::Red));
            Paragraph::new(text)
                .style(Style::default().fg(Color::Red).bg(Color::Black))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
        }
    } else {
        bail!("Invalid popup state")
    };

    f.render_widget(Clear, indicator_area); //this clears out the background
    f.render_widget(block, indicator_area);
    f.render_widget(
        indicator,
        *chunks_indicator.get(1).unwrap_or(&indicator_area),
    );

    Ok(())
}
