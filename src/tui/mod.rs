use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BorderType, List, ListItem, ListState};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use crate::config::filehandler::load;
use crate::config::modifier::Modifier as KeyModifier;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode};
use crate::config::Config;
use crate::utils;
use crate::utils::AnyhowUnwrap;

mod key_handler;
mod popups;

#[derive(Clone)]
pub enum PopupState {
    None,
    List(ListState),
    MultiList(MultiselectListState),
    String(String),
    Int {
        current: isize,
        min: isize,
        max: isize,
    },
}

#[derive(Clone)]
pub struct MultiselectListState {
    pub(crate) liststate: ListState,
    pub(crate) selected: Vec<usize>,
}

#[allow(dead_code)]
pub enum Window {
    Home,
    Workspaces { index: usize, empty: bool },
    Tags { index: usize, empty: bool },
    WindowRules { index: usize, empty: bool },
    Scratchpads { index: usize, empty: bool },
    KeyBinds,
}

struct App<'a> {
    config_list: Vec<ListItem<'a>>,
    config_list_state: ListState,
    current_popup: Option<u8>,
    current_window: Window,
    current_popup_state: PopupState,
    current_config: Config,
    alive: Result<()>,
}

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    let app = App {
        config_list: vec![],
        config_list_state: state,
        current_popup: None,
        current_window: Window::Home,
        current_popup_state: PopupState::None,
        current_config: load(),
        alive: Ok(()),
    };

    let app_result = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    app_result
}

impl App<'_> {
    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        while self.alive.is_ok() {
            terminal.draw(|f| {
                match self.format_config_list() {
                    Err(e) => self.alive = Err(e),
                    Ok(l) => self.config_list = l,
                }
                let size = f.size();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
                    .split(size);

                let frame = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .border_type(BorderType::Rounded)
                    .style(Style::default().bg(Color::Black))
                    .title("LeftWM-Config");

                let list = List::new(self.config_list.clone())
                    .block(Block::default().borders(Borders::NONE))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(">>");

                let text = match self.current_window {
                    Window::Workspaces { .. }
                    | Window::WindowRules { .. }
                    | Window::Scratchpads { .. } => {
                        vec![Spans::from(vec![
                            Span::raw("Exit: q, "),
                            Span::raw("Save: s, "),
                            Span::raw("Delete Optional Value: Delete, "),
                            Span::raw("Back: Backspace"),
                        ])]
                    }
                    Window::Tags { .. } => {
                        vec![Spans::from(vec![
                            Span::raw("Exit: q, "),
                            Span::raw("Save: s, "),
                            Span::raw("Back: Backspace"),
                        ])]
                    }
                    _ => {
                        vec![Spans::from(vec![
                            Span::raw("Exit: q, "),
                            Span::raw("Save: s"),
                        ])]
                    }
                };

                let help = Paragraph::new(text)
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                f.render_widget(frame, size);
                f.render_stateful_widget(
                    list,
                    utils::centered_rect(50, 50, *chunks.get(0).unwrap_or(&size)),
                    &mut self.config_list_state,
                );
                f.render_widget(help, *chunks.get(1).unwrap_or(&size));

                if let Err(e) = match self.current_window {
                    Window::Home => {
                        if let Some(s) = self.current_popup {
                            match s {
                                0 => popups::modkey(
                                    &self.current_config,
                                    &mut self.current_popup_state,
                                    f,
                                    false,
                                ),
                                1 => popups::modkey(
                                    &self.current_config,
                                    &mut self.current_popup_state,
                                    f,
                                    true,
                                ),
                                2 => popups::max_window_width(&mut self.current_popup_state, f),
                                // 3, 4 and 5 dont need a popup
                                6 => popups::focus_behavior(
                                    &self.current_config,
                                    &mut self.current_popup_state,
                                    f,
                                ),
                                7 => popups::insert_behavior(
                                    &self.current_config,
                                    &mut self.current_popup_state,
                                    f,
                                ),
                                8 => popups::layout_mode(
                                    &self.current_config,
                                    &mut self.current_popup_state,
                                    f,
                                ),
                                9 => popups::layouts(&mut self.current_popup_state, f),
                                15 => popups::saved(f),
                                _ => Ok(()),
                            }
                        } else {
                            Ok(())
                        }
                    }
                    Window::Workspaces { .. } => {
                        if let Some(s) = self.current_popup {
                            match s {
                                0 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "X".to_string(),
                                    f,
                                ),
                                1 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "Y".to_string(),
                                    f,
                                ),
                                2 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "Widht".to_string(),
                                    f,
                                ),
                                3 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "Height".to_string(),
                                    f,
                                ),
                                4 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "Id".to_string(),
                                    f,
                                ),
                                5 => popups::text_input(
                                    &mut self.current_popup_state,
                                    "Max window width".to_string(),
                                    f,
                                ),
                                6 => popups::layouts(&mut self.current_popup_state, f),
                                15 => popups::saved(f),
                                _ => Ok(()),
                            }
                        } else {
                            Ok(())
                        }
                    }
                    Window::Tags { .. } => {
                        if let Some(15) = self.current_popup {
                            popups::saved(f)
                        } else if self.current_popup.is_some() {
                            popups::text_input(&mut self.current_popup_state, "Name".to_string(), f)
                        } else {
                            Ok(())
                        }
                    }
                    Window::WindowRules { .. } => match self.current_popup {
                        Some(0) => popups::text_input(
                            &mut self.current_popup_state,
                            "Title".to_string(),
                            f,
                        ),
                        Some(1) => popups::text_input(
                            &mut self.current_popup_state,
                            "Class".to_string(),
                            f,
                        ),
                        Some(2) => popups::counter(
                            &mut self.current_popup_state,
                            "Spawn on tag".to_string(),
                            f,
                        ),
                        Some(15) => popups::saved(f),
                        _ => Ok(()),
                    },
                    Window::Scratchpads { .. } => match self.current_popup {
                        Some(0) => {
                            popups::text_input(&mut self.current_popup_state, "Name".to_string(), f)
                        }
                        Some(1) => popups::text_input(
                            &mut self.current_popup_state,
                            "Value".to_string(),
                            f,
                        ),
                        Some(2) => {
                            popups::text_input(&mut self.current_popup_state, "X".to_string(), f)
                        }
                        Some(3) => {
                            popups::text_input(&mut self.current_popup_state, "Y".to_string(), f)
                        }
                        Some(4) => popups::text_input(
                            &mut self.current_popup_state,
                            "Width".to_string(),
                            f,
                        ),
                        Some(5) => popups::text_input(
                            &mut self.current_popup_state,
                            "Height".to_string(),
                            f,
                        ),
                        Some(15) => popups::saved(f),
                        _ => Ok(()),
                    },
                    _ => Ok(()),
                } {
                    self.alive = Err(e);
                }
            })?;

            if key_handler::handle_keys(&mut self)? {
                return Ok(());
            }
        }

        match self.alive {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn format_config_list<'a>(&mut self) -> Result<Vec<ListItem<'a>>> {
        Ok(match self.current_window {
            Window::Home => Vec::from([
                ListItem::new(format!(
                    "Modkey - {}",
                    format_modkey_name(self.current_config.modkey.clone())
                )),
                ListItem::new(format!(
                    "Mousekey - {}",
                    format_modkey_name(
                        self.current_config
                            .mousekey
                            .clone()
                            .unwrap_or_else(|| KeyModifier::Single("None".to_string()))
                            .to_string()
                    )
                )),
                ListItem::new(match &self.current_config.max_window_width {
                    Some(w) => format!("Max Window Width - {:?}", w),
                    None => "Max Window Width - not set".to_string(),
                }),
                ListItem::new(format!(
                    "Disable Current Tag Swap - {}",
                    self.current_config.disable_current_tag_swap
                )),
                ListItem::new(format!(
                    "Disable Tile Drag - {}",
                    self.current_config.disable_tile_drag
                )),
                ListItem::new(format!(
                    "Focus New Windows - {}",
                    self.current_config.focus_new_windows
                )),
                ListItem::new(format!(
                    "Focus Behavior - {}",
                    match self.current_config.focus_behaviour {
                        FocusBehaviour::Sloppy => "Sloppy".to_string(),
                        FocusBehaviour::ClickTo => "Click To".to_string(),
                        FocusBehaviour::Driven => "Driven".to_string(),
                    }
                )),
                ListItem::new(format!(
                    "Insert Behavior - {}",
                    match self.current_config.insert_behavior {
                        InsertBehavior::AfterCurrent => "Afer Current".to_string(),
                        InsertBehavior::BeforeCurrent => "Before Current".to_string(),
                        InsertBehavior::Bottom => "Bottom".to_string(),
                        InsertBehavior::Top => "Top".to_string(),
                    }
                )),
                ListItem::new(format!(
                    "Layout Mode - {}",
                    match self.current_config.layout_mode {
                        LayoutMode::Tag => "Tag".to_string(),
                        LayoutMode::Workspace => "Workspace".to_string(),
                    }
                )),
                ListItem::new(format!(
                    "Layouts - {} set",
                    self.current_config.layouts.len()
                )),
                ListItem::new(match &self.current_config.workspaces {
                    Some(v) => format!("Workspaces - {} set", v.len()),
                    None => "Workspaces".to_string(),
                }),
                ListItem::new(match &self.current_config.tags {
                    Some(v) => format!("Tags - {} set", v.len()),
                    None => "Tags".to_string(),
                }),
                ListItem::new(match &self.current_config.window_rules {
                    Some(v) => format!("Window Rules - {} set", v.len()),
                    None => "Window Rules".to_string(),
                }),
                ListItem::new(match &self.current_config.scratchpad {
                    Some(v) => format!("Scratchpads - {} set", v.len()),
                    None => "Scratchpads".to_string(),
                }),
                ListItem::new(format!(
                    "Keybinds - {} set",
                    self.current_config.keybind.len()
                )),
            ]),
            Window::Workspaces { index, .. } => {
                let current_workspace = if let Some(w) = &self.current_config.workspaces {
                    if self
                        .current_config
                        .workspaces
                        .as_ref()
                        .unwrap_anyhow()?
                        .is_empty()
                    {
                        None
                    } else {
                        w.get(index)
                    }
                } else {
                    None
                };

                if let Some(c) = current_workspace {
                    vec![
                        ListItem::new(format!(
                            "{} out of {}",
                            index + 1,
                            self.current_config
                                .workspaces
                                .as_ref()
                                .unwrap_anyhow()?
                                .len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new(format!("X - {}", c.x)),
                        ListItem::new(format!("Y - {}", c.y)),
                        ListItem::new(format!("Width - {}", c.width)),
                        ListItem::new(format!("Height - {}", c.height)),
                        ListItem::new(format!("Id - {:?}", c.id)),
                        ListItem::new(format!("Max Window Width - {:?}", c.max_window_width)),
                        ListItem::new(format!(
                            "Layouts - {}",
                            if c.layouts.is_some() {
                                "Some(Open to see more)"
                            } else {
                                "None"
                            }
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new workspace"),
                        ListItem::new("Delete this workspace"),
                    ]
                } else {
                    vec![
                        ListItem::new(format!(
                            "None out of {}",
                            self.current_config
                                .workspaces
                                .as_ref()
                                .unwrap_anyhow()?
                                .len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new workspace"),
                    ]
                }
            }
            Window::Tags { index, .. } => {
                let current_workspace = if let Some(w) = &self.current_config.tags {
                    if self
                        .current_config
                        .tags
                        .as_ref()
                        .unwrap_anyhow()?
                        .is_empty()
                    {
                        None
                    } else {
                        w.get(index)
                    }
                } else {
                    None
                };

                if let Some(c) = current_workspace {
                    vec![
                        ListItem::new(format!(
                            "{} out of {}",
                            index + 1,
                            self.current_config.tags.as_ref().unwrap_anyhow()?.len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new(format!("Name - {}", c)),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new tag"),
                        ListItem::new("Delete this tag"),
                    ]
                } else {
                    vec![
                        ListItem::new(format!(
                            "None out of {}",
                            self.current_config.tags.as_ref().unwrap_anyhow()?.len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new tag"),
                    ]
                }
            }
            Window::WindowRules { index, empty } => {
                if empty {
                    vec![
                        ListItem::new("None out of 0"),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new rule"),
                    ]
                } else {
                    let rule = self
                        .current_config
                        .window_rules
                        .as_ref()
                        .unwrap_anyhow()?
                        .get(index)
                        .unwrap_anyhow()?;

                    let mut vec = vec![
                        ListItem::new(format!(
                            "{} out of {}",
                            index + 1,
                            self.current_config
                                .window_rules
                                .as_ref()
                                .unwrap_anyhow()?
                                .len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new(format!("Title - {:?}", rule.window_title)),
                        ListItem::new(format!("Class - {:?}", rule.window_class)),
                        ListItem::new(format!("Spawn on tag - {:?}", rule.spawn_on_tag)),
                        ListItem::new(format!(
                            "Spawn floating - {}",
                            rule.spawn_floating.unwrap_or(false)
                        )),
                        ListItem::new("--------------------------"),
                    ];

                    if rule.window_class.is_none() && rule.window_title.is_none() {
                        vec.push(ListItem::new("WARNING:").style(Style::default().fg(Color::Red)));
                        vec.push(
                            ListItem::new("Neither title nor class are set")
                                .style(Style::default().fg(Color::Red)),
                        );
                        vec.push(
                            ListItem::new("This rule will be ignored")
                                .style(Style::default().fg(Color::Red)),
                        );
                        vec.push(ListItem::new("--------------------------"));
                    } else if rule.window_class.is_some() && rule.window_title.is_some() {
                        vec.push(ListItem::new("WARNING:").style(Style::default().fg(Color::Red)));
                        vec.push(
                            ListItem::new("Both the title and class are set")
                                .style(Style::default().fg(Color::Red)),
                        );
                        vec.push(
                            ListItem::new("Class will be ignored")
                                .style(Style::default().fg(Color::Red)),
                        );
                        vec.push(ListItem::new("--------------------------"));
                    }

                    vec.push(ListItem::new("Add new rule"));
                    vec.push(ListItem::new("Delete this rule"));

                    vec
                }
            }
            Window::Scratchpads { index, empty } => {
                if empty {
                    vec![
                        ListItem::new("None out of 0"),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new scratchpad"),
                    ]
                } else {
                    let scratchpad = self
                        .current_config
                        .scratchpad
                        .as_ref()
                        .unwrap_anyhow()?
                        .get(index)
                        .unwrap_anyhow()?;

                    vec![
                        ListItem::new(format!(
                            "{} out of {}",
                            index + 1,
                            self.current_config
                                .scratchpad
                                .as_ref()
                                .unwrap_anyhow()?
                                .len()
                        )),
                        ListItem::new("--------------------------"),
                        ListItem::new(format!("Name - {}", scratchpad.name)),
                        ListItem::new(format!("Value - {}", scratchpad.value)),
                        ListItem::new(format!("X - {:?}", scratchpad.x)),
                        ListItem::new(format!("Y - {:?}", scratchpad.y)),
                        ListItem::new(format!("Width - {:?}", scratchpad.width)),
                        ListItem::new(format!("Height - {:?}", scratchpad.height)),
                        ListItem::new("--------------------------"),
                        ListItem::new("Add new scratchpad"),
                        ListItem::new("Delete this scratchpad"),
                    ]
                }
            }
            _ => {
                vec![]
            }
        })
    }
}

fn next(state: &mut ListState, len: usize) {
    let i = match state.selected() {
        Some(i) => {
            if i >= len - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

fn previous(state: &mut ListState, len: usize) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                len - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.select(Some(i));
}

fn format_modkey_name(modkey: String) -> String {
    match modkey.as_str() {
        "Mod1" | "Alt" => "Alt".to_string(),
        "Mod4" | "Super" => "Super".to_string(),
        _ => modkey,
    }
}
