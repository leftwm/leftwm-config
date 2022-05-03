use std::io;
use std::io::Stdout;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{BorderType, List, ListItem, ListState};

use crate::Config;
use crate::config::{load, save_to_file};
use crate::config::layout::Layout as WMLayout;
use crate::config::modifier::Modifier as KeyModifier;
use crate::config::structs::Workspace;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode, Size};
use crate::utils::popups_home;
use crate::utils::popups_workspaces;

pub enum PopupState {
    None,
    List(ListState),
    MultiList(MultiselectListState),
    MultistructState(MultistructState),
    String(String),
}

pub struct MultistructState {
    pub items: usize,
    pub items_list_state: ListState,
    pub fields: usize,
    pub fields_list_state: ListState,
}

#[derive(Clone)]
pub struct MultiselectListState {
    pub(crate) liststate: ListState,
    pub(crate) selected: Vec<usize>,
}

pub enum Window {
    Home,
    Workspaces {
        index: u8
    },
    Tags,
    WindowRules,
    Scratchpads,
    Keybinds,
}

struct App<'a> {
    config_list: Vec<ListItem<'a>>,
    config_list_state: ListState,
    current_popup: Option<u8>,
    // popups: [
    //     "Modkey",
    //     "MouseKey",
    //     "Max Window Width",
    //     "Disable Current Tag Swap",
    //     "Disable Tile Drag",
    //     "Focus New Windows",
    //     "Focus Behavior",
    //     "Insert Behavior",
    //     "Layout Mode",
    //     "Layouts",
    //     "Saved",
    // ]
    current_window: Window,
    current_popup_state: PopupState,
    current_config: Config,
}

pub fn run() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    let mut app = App {
        config_list: vec![],
        config_list_state: state,
        current_popup: None,
        current_window: Window::Home,
        current_popup_state: PopupState::None,
        current_config: load(),
    };

    app.run(&mut terminal)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

impl App<'_> {
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| {
                self.config_list = self.format_config_list();
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

                let text = vec![Spans::from(
                    vec![
                        Span::raw("Exit: q, "),
                        Span::raw("Save: s"),
                    ])
                ];

                let help = Paragraph::new(text)
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: true });

                f.render_widget(frame, size);
                f.render_stateful_widget(list, centered_rect(50, 50, *chunks.get(0).unwrap_or(&size)), &mut self.config_list_state);
                f.render_widget(help, *chunks.get(1).unwrap_or(&size));

                #[allow(clippy::single_match)]
                match self.current_window {
                    Window::Home => {
                        if let Some(s) = self.current_popup {
                            if let Err(e) = match s {
                                0 => { popups_home::modkey(&self.current_config, &mut self.current_popup_state, f, false) }
                                1 => { popups_home::modkey(&self.current_config, &mut self.current_popup_state, f, true) }
                                2 => { popups_home::max_window_width(&mut self.current_popup_state, f) }
                                // 3, 4 and 5 dont need a popup
                                6 => { popups_home::focus_behavior(&self.current_config, &mut self.current_popup_state, f) }
                                7 => { popups_home::insert_behavior(&self.current_config, &mut self.current_popup_state, f) }
                                8 => { popups_home::layout_mode(&self.current_config, &mut self.current_popup_state, f) }
                                9 => { popups_home::layouts(&mut self.current_popup_state, f) }
                                // 10 => { popups::workspaces(&mut self.current_config, &mut self.current_popup_state, f) }
                                // 11 => { Ok(()) }
                                // 12 => { Ok(()) }
                                // 13 => { Ok(()) }
                                // 14 => { Ok(()) }
                                15 => { popups_home::saved(f) }
                                _ => { Ok(()) }
                            } {
                                panic!("{}", e);
                            }
                        }
                    }
                    Window::Workspaces {index} => {
                        if let Some(s) = self.current_popup {
                            if let Err(e) = match s {
                                0 => { popups_workspaces::text_input(&mut self.current_popup_state, "X".to_string(), f) }
                                _ => { Ok(()) }
                            } {
                                panic!("{}", e);
                            }
                        }
                    }
                    _ => {}
                }
            })?;

            if self.handle_events()? {
                return Ok(());
            }
        }
    }

    fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if self.current_popup.is_none() {
                        match self.config_list_state.selected() {
                            None => self.config_list_state.select(Some(0)),
                            Some(_) => { previous(&mut self.config_list_state, self.config_list.len()) }
                        }
                    } else if let Some(s) = self.current_popup {
                        match s {
                            0 => {
                                if let PopupState::List(s) = &mut self.current_popup_state {
                                    previous(s, 7);
                                }
                            }
                            1 => {
                                if let PopupState::List(s) = &mut self.current_popup_state {
                                    previous(s, 7);
                                }
                            }
                            2 => {}
                            3 => {}
                            4 => {}
                            5 => {}
                            6 => {
                                if let PopupState::List(s) = &mut self.current_popup_state {
                                    previous(s, 3);
                                }
                            }
                            7 => {
                                if let PopupState::List(s) = &mut self.current_popup_state {
                                    previous(s, 4);
                                }
                            }
                            8 => {
                                if let PopupState::List(s) = &mut self.current_popup_state {
                                    previous(s, 2);
                                }
                            }
                            9 => {
                                if let PopupState::MultiList(s) = &mut self.current_popup_state {
                                    previous(&mut s.liststate, 14);
                                }
                            }
                            10 => {
                                if let PopupState::MultistructState(s) = &mut self.current_popup_state {
                                    previous(&mut s.fields_list_state, s.fields);
                                } else {
                                    panic!("wrong state");
                                }
                            }
                            11 => {}
                            12 => {}
                            13 => {}
                            14 => {}
                            _ => {}
                        }
                    }
                }
                KeyCode::Down => {
                    if self.current_popup.is_none() {
                        match self.config_list_state.selected() {
                            None => self.config_list_state.select(Some(0)),
                            Some(_) => { next(&mut self.config_list_state, self.config_list.len()) }
                        }
                    } else if let Some(s) = self.current_popup {
                        match self.current_window {
                            Window::Home => {
                                match s {
                                    0 => {
                                        if let PopupState::List(s) = &mut self.current_popup_state {
                                            next(s, 7);
                                        }
                                    }
                                    1 => {
                                        if let PopupState::List(s) = &mut self.current_popup_state {
                                            next(s, 7);
                                        }
                                    }
                                    6 => {
                                        if let PopupState::List(s) = &mut self.current_popup_state {
                                            next(s, 3);
                                        }
                                    }
                                    7 => {
                                        if let PopupState::List(s) = &mut self.current_popup_state {
                                            next(s, 4);
                                        }
                                    }
                                    8 => {
                                        if let PopupState::List(s) = &mut self.current_popup_state {
                                            next(s, 2);
                                        }
                                    }
                                    9 => {
                                        if let PopupState::MultiList(s) = &mut self.current_popup_state {
                                            next(&mut s.liststate, 14);
                                        }
                                    }
                                    10 => {
                                        if let PopupState::MultistructState(s) = &mut self.current_popup_state {
                                            next(&mut s.fields_list_state, s.fields);
                                        } else {
                                            panic!("wrong state");
                                        }
                                    }
                                    11 => {}
                                    12 => {}
                                    13 => {}
                                    14 => {}
                                    _ => {}
                                }
                            }
                            Window::Workspaces { index } => {}
                            _ => {}
                        }
                    }
                }
                KeyCode::Right => {
                    // if let Some(s) = self.current_popup {
                    //     match s {
                    //         10 => {
                    //             if let PopupState::MultistructState(s) = &mut self.current_popup_state {
                    //                 next(&mut s.items_list_state, s.items);
                    //             } else {
                    //                 panic!("wrong state");
                    //             }
                    //         }
                    //         _ => {}
                    //     }
                    // }
                }
                KeyCode::Left => {
                    // if let Some(s) = self.current_popup {
                    //     match s {
                    //         10 => {
                    //             if let PopupState::MultistructState(s) = &mut self.current_popup_state {
                    //                 previous(&mut s.items_list_state, s.items);
                    //             } else {
                    //                 panic!("wrong state");
                    //             }
                    //         }
                    //         _ => {}
                    //     }
                    // }
                }
                KeyCode::Enter => {
                    match self.current_window {
                        Window::Home => {
                            if let Some(s) = self.config_list_state.selected() {
                                if self.current_popup.is_none() {
                                    match s {
                                        0 => {
                                            self.current_popup = Some(0);
                                            let mut state = ListState::default();
                                            match self.current_config.modkey.as_str() {
                                                "None" => state.select(Some(0)),
                                                "Shift" => state.select(Some(1)),
                                                "Control" => state.select(Some(2)),
                                                "Mod1" | "Alt" => state.select(Some(3)),
                                                //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                                //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                                "Mod3" => state.select(Some(4)),
                                                "Mod4" | "Super" => state.select(Some(5)),
                                                "Mod5" => state.select(Some(6)),
                                                _ => state.select(None),
                                            }
                                            self.current_popup_state = PopupState::List(state);
                                        }
                                        1 => {
                                            self.current_popup = Some(1);
                                            let mut state = ListState::default();
                                            match self.current_config.mousekey.clone().unwrap_or_else(|| KeyModifier::Single("None".to_string())).to_string().as_str() {
                                                "None" => state.select(Some(0)),
                                                "Shift" => state.select(Some(1)),
                                                "Control" => state.select(Some(2)),
                                                "Mod1" | "Alt" => state.select(Some(3)),
                                                //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                                //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                                "Mod3" => state.select(Some(4)),
                                                "Mod4" | "Super" => state.select(Some(5)),
                                                "Mod5" => state.select(Some(6)),
                                                _ => state.select(None),
                                            }
                                            self.current_popup_state = PopupState::List(state);
                                        }
                                        2 => {
                                            self.current_popup = Some(2);
                                            self.current_popup_state = PopupState::String(String::new())
                                        }
                                        3 => {
                                            self.current_config.disable_current_tag_swap = !self.current_config.disable_current_tag_swap;
                                        }
                                        4 => {
                                            self.current_config.disable_tile_drag = !self.current_config.disable_tile_drag;
                                        }
                                        5 => {
                                            self.current_config.focus_new_windows = !self.current_config.focus_new_windows;
                                        }
                                        6 => {
                                            self.current_popup = Some(6);
                                            let index = match self.current_config.focus_behaviour {
                                                FocusBehaviour::Sloppy => Some(0),
                                                FocusBehaviour::ClickTo => Some(1),
                                                FocusBehaviour::Driven => Some(2)
                                            };
                                            let mut state = ListState::default();
                                            state.select(index);
                                            self.current_popup_state = PopupState::List(state)
                                        }
                                        7 => {
                                            self.current_popup = Some(7);
                                            let index = match self.current_config.insert_behavior {
                                                InsertBehavior::Top => Some(0),
                                                InsertBehavior::Bottom => Some(1),
                                                InsertBehavior::BeforeCurrent => Some(2),
                                                InsertBehavior::AfterCurrent => Some(3),
                                            };
                                            let mut state = ListState::default();
                                            state.select(index);
                                            self.current_popup_state = PopupState::List(state)
                                        }
                                        8 => {
                                            self.current_popup = Some(8);
                                            let index = match self.current_config.layout_mode {
                                                LayoutMode::Tag => Some(0),
                                                LayoutMode::Workspace => Some(1),
                                            };
                                            let mut state = ListState::default();
                                            state.select(index);
                                            self.current_popup_state = PopupState::List(state)
                                        }
                                        9 => {
                                            self.current_popup = Some(9);
                                            let mut selected: Vec<usize> = vec![];
                                            for l in &self.current_config.layouts {
                                                match l {
                                                    WMLayout::MainAndVertStack => selected.push(0),
                                                    WMLayout::MainAndHorizontalStack => selected.push(1),
                                                    WMLayout::MainAndDeck => selected.push(2),
                                                    WMLayout::GridHorizontal => selected.push(3),
                                                    WMLayout::EvenHorizontal => selected.push(4),
                                                    WMLayout::EvenVertical => selected.push(5),
                                                    WMLayout::Fibonacci => selected.push(6),
                                                    WMLayout::LeftMain => selected.push(7),
                                                    WMLayout::CenterMain => selected.push(8),
                                                    WMLayout::CenterMainBalanced => selected.push(9),
                                                    WMLayout::CenterMainFluid => selected.push(10),
                                                    WMLayout::Monocle => selected.push(11),
                                                    WMLayout::RightWiderLeftStack => selected.push(12),
                                                    WMLayout::LeftWiderRightStack => selected.push(13),
                                                }
                                            }
                                            let mut liststate = ListState::default();
                                            liststate.select(Some(0));
                                            self.current_popup_state = PopupState::MultiList(MultiselectListState {
                                                selected,
                                                liststate,
                                            })
                                        }
                                        10 => {
                                            self.current_window = Window::Workspaces {
                                                index: 0,
                                            };
                                        }
                                        11 => {}
                                        12 => {}
                                        13 => {}
                                        14 => {}
                                        _ => {}
                                    }
                                } else if let Some(s) = self.current_popup {
                                    match s {
                                        0 => {
                                            if let PopupState::List(s) = &self.current_popup_state {
                                                if let Some(s) = s.selected() {
                                                    match s {
                                                        0 => {
                                                            self.current_config.modkey = "None".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        1 => {
                                                            self.current_config.modkey = "Shift".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        2 => {
                                                            self.current_config.modkey = "Control".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        3 => {
                                                            self.current_config.modkey = "Mod1".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        4 => {
                                                            self.current_config.modkey = "Mod3".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        5 => {
                                                            self.current_config.modkey = "Mod4".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        6 => {
                                                            self.current_config.modkey = "Mod5".to_string();
                                                            self.current_popup = None;
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                            }
                                        }
                                        1 => {
                                            if let PopupState::List(s) = &self.current_popup_state {
                                                if let Some(s) = s.selected() {
                                                    match s {
                                                        0 => self.current_config.mousekey = None,
                                                        1 => self.current_config.mousekey = Some(KeyModifier::Single("Shift".to_string())),
                                                        2 => self.current_config.mousekey = Some(KeyModifier::Single("Control".to_string())),
                                                        3 => self.current_config.mousekey = Some(KeyModifier::Single("Mod1".to_string())),
                                                        4 => self.current_config.mousekey = Some(KeyModifier::Single("Mod3".to_string())),
                                                        5 => self.current_config.mousekey = Some(KeyModifier::Single("Mod4".to_string())),
                                                        6 => self.current_config.mousekey = Some(KeyModifier::Single("Mod5".to_string())),
                                                        _ => {}
                                                    }
                                                }
                                                self.current_popup = None;
                                            } else {
                                                panic!("popup state incorrectly set")
                                            };
                                        }
                                        2 => {
                                            self.current_config.max_window_width = if let PopupState::String(s) = &self.current_popup_state {
                                                if s.contains('.') {
                                                    Some(Size::Ratio(s.parse().unwrap_or(0.0)))
                                                } else {
                                                    Some(Size::Pixel(s.parse().unwrap_or(0)))
                                                }
                                            } else {
                                                panic!("popup state incorrectly set")
                                            };
                                            self.current_popup = None;
                                        }
                                        3 => {}
                                        4 => {}
                                        5 => {}
                                        6 => {
                                            if let PopupState::List(l) = &self.current_popup_state {
                                                match l.selected() {
                                                    Some(0) => self.current_config.focus_behaviour = FocusBehaviour::Sloppy,
                                                    Some(1) => self.current_config.focus_behaviour = FocusBehaviour::ClickTo,
                                                    Some(2) => self.current_config.focus_behaviour = FocusBehaviour::Driven,
                                                    Some(i) => panic!("index out of bounds {i}"),
                                                    None => {}
                                                }
                                            }
                                            self.current_popup = None;
                                        }
                                        7 => {
                                            if let PopupState::List(l) = &self.current_popup_state {
                                                match l.selected() {
                                                    Some(0) => self.current_config.insert_behavior = InsertBehavior::Top,
                                                    Some(1) => self.current_config.insert_behavior = InsertBehavior::Bottom,
                                                    Some(2) => self.current_config.insert_behavior = InsertBehavior::BeforeCurrent,
                                                    Some(3) => self.current_config.insert_behavior = InsertBehavior::AfterCurrent,
                                                    Some(i) => panic!("index out of bounds {i}"),
                                                    None => {}
                                                }
                                            }
                                            self.current_popup = None;
                                        }
                                        8 => {
                                            if let PopupState::List(l) = &self.current_popup_state {
                                                match l.selected() {
                                                    Some(0) => self.current_config.layout_mode = LayoutMode::Tag,
                                                    Some(1) => self.current_config.layout_mode = LayoutMode::Workspace,
                                                    Some(i) => panic!("index out of bounds {i}"),
                                                    None => {}
                                                }
                                            }
                                            self.current_popup = None;
                                        }
                                        9 => {
                                            if let PopupState::MultiList(l) = &self.current_popup_state {
                                                let mut layouts: Vec<WMLayout> = vec![];
                                                for s in &l.selected {
                                                    match s {
                                                        0 => layouts.push(WMLayout::MainAndVertStack),
                                                        1 => layouts.push(WMLayout::MainAndHorizontalStack),
                                                        2 => layouts.push(WMLayout::MainAndDeck),
                                                        3 => layouts.push(WMLayout::GridHorizontal),
                                                        4 => layouts.push(WMLayout::EvenHorizontal),
                                                        5 => layouts.push(WMLayout::EvenVertical),
                                                        6 => layouts.push(WMLayout::Fibonacci),
                                                        7 => layouts.push(WMLayout::LeftMain),
                                                        8 => layouts.push(WMLayout::CenterMain),
                                                        9 => layouts.push(WMLayout::CenterMainBalanced),
                                                        10 => layouts.push(WMLayout::CenterMainFluid),
                                                        11 => layouts.push(WMLayout::Monocle),
                                                        12 => layouts.push(WMLayout::RightWiderLeftStack),
                                                        13 => layouts.push(WMLayout::LeftWiderRightStack),
                                                        _ => {}
                                                    }
                                                }
                                                self.current_config.layouts = layouts;
                                                self.current_popup = None
                                            }
                                        }
                                        10 => {}
                                        11 => {}
                                        12 => {}
                                        13 => {}
                                        14 => {}
                                        _ => {}
                                    }
                                }
                            }
                        }
                        Window::Workspaces { index } => {
                            if let Some(s) = self.current_popup {
                                match s {
                                    0 => {
                                        self.current_popup = None;
                                        if let Some(w) = &mut self.current_config.workspaces{
                                            let ws = w.get_mut(index as usize).unwrap();
                                            if let PopupState::String(s) = &self.current_popup_state {
                                                ws.x = s.parse().unwrap();
                                            }
                                        }
                                    }
                                    1 => {}
                                    _ => {}
                                }
                            } else if let Some(s) = self.config_list_state.selected() {
                                match s {
                                    0 => {
                                        self.current_popup = Some(0);
                                        self.current_popup_state = PopupState::String(String::new());
                                    }
                                    1 => {}
                                    2 => {}
                                    3 => {}
                                    4 => {}
                                    5 => {}
                                    6 => {}
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Esc => {
                    self.current_popup = None;
                }
                //space
                KeyCode::Char(' ') => {
                    match self.current_window {
                        Window::Home => {
                            if let Some(9) = self.current_popup {
                                if let PopupState::MultiList(l) = &mut self.current_popup_state {
                                    // l.selected.push(l.liststate.selected().unwrap_or(14));
                                    if !l.selected.contains(&l.liststate.selected().unwrap_or(14)) {
                                        l.selected.push(l.liststate.selected().unwrap_or(14))
                                    } else {
                                        let index = l.selected.iter().position(|x| *x == l.liststate.selected().unwrap_or(14)).unwrap();
                                        l.selected.remove(index);
                                    }
                                }
                            }
                        }
                        Window::Workspaces { index } => {}
                        _ => {}
                    }
                }
                KeyCode::Char(c) => {
                    match self.current_window {
                        Window::Home => {
                            match self.current_popup {
                                Some(2) => {
                                    if let PopupState::String(s) = &mut self.current_popup_state {
                                        if "1234567890,.".contains(c) {
                                            s.push(c);
                                        }
                                    }
                                }
                                Some(_) => {}
                                None => {
                                    match c {
                                        'q' => {
                                            return Ok(true);
                                        }
                                        's' => {
                                            save_to_file(&self.current_config)?;
                                            self.current_popup = Some(15);
                                            self.current_popup_state = PopupState::None;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        Window::Workspaces { index } => {
                            match self.current_popup {
                                Some(0..=3) => {
                                    if let PopupState::String(s) = &mut self.current_popup_state {
                                        if "1234567890,.".contains(c) {
                                            s.push(c);
                                        }
                                    }
                                }
                                Some(_) => {}
                                None => {
                                    match c {
                                        'q' => {
                                            return Ok(true);
                                        }
                                        's' => {
                                            save_to_file(&self.current_config)?;
                                            self.current_popup = Some(15);
                                            self.current_popup_state = PopupState::None;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Backspace => {
                    if let Some(p) = self.current_popup {
                        match p {
                            2 => {
                                if let PopupState::String(s) = &mut self.current_popup_state {
                                    s.pop();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn format_config_list<'a>(&mut self) -> Vec<ListItem<'a>> {
        match self.current_window {
            Window::Home => {
                Vec::from([
                    ListItem::new(format!("Modkey - {}", format_modkey_name(self.current_config.modkey.clone()))),
                    ListItem::new(format!("Mousekey - {}", format_modkey_name(self.current_config.mousekey.clone().unwrap_or_else(|| KeyModifier::Single("None".to_string())).to_string()))),
                    ListItem::new(match &self.current_config.max_window_width {
                        Some(w) => {
                            format!("Max Window Width - {}", match w {
                                Size::Pixel(s) => { format!("{s}") }
                                Size::Ratio(s) => { format!("{s}") }
                            })
                        }
                        None => "Max Window Width - not set".to_string()
                    }),
                    ListItem::new(format!("Disable Current Tag Swap - {}", self.current_config.disable_current_tag_swap)),
                    ListItem::new(format!("Disable Tile Drag - {}", self.current_config.disable_tile_drag)),
                    ListItem::new(format!("Focus New Windows - {}", self.current_config.focus_new_windows)),
                    ListItem::new(format!("Focus Behavior - {}", match self.current_config.focus_behaviour {
                        FocusBehaviour::Sloppy => { "Sloppy".to_string() }
                        FocusBehaviour::ClickTo => { "Click To".to_string() }
                        FocusBehaviour::Driven => { "Driven".to_string() }
                    })),
                    ListItem::new(format!("Insert Behavior - {}", match self.current_config.insert_behavior {
                        InsertBehavior::AfterCurrent => { "Afer Current".to_string() }
                        InsertBehavior::BeforeCurrent => { "Before Current".to_string() }
                        InsertBehavior::Bottom => { "Bottom".to_string() }
                        InsertBehavior::Top => { "Top".to_string() }
                    })),
                    ListItem::new(format!("Layout Mode - {}", match self.current_config.layout_mode {
                        LayoutMode::Tag => { "Tag".to_string() }
                        LayoutMode::Workspace => { "Workspace".to_string() }
                    })),
                    ListItem::new(format!("Layouts - {} set", self.current_config.layouts.len())),
                    ListItem::new(match &self.current_config.workspaces {
                        Some(v) => { format!("Workspaces - {} set", v.len()) }
                        None => "Workspaces".to_string()
                    }),
                    ListItem::new(match &self.current_config.tags {
                        Some(v) => { format!("Tags - {} set", v.len()) }
                        None => "Tags".to_string()
                    }),
                    ListItem::new(match &self.current_config.window_rules {
                        Some(v) => { format!("Window Rules - {} set", v.len()) }
                        None => "Window Rules".to_string()
                    }),
                    ListItem::new(match &self.current_config.scratchpad {
                        Some(v) => { format!("Scratchpads - {} set", v.len()) }
                        None => "Scratchpads".to_string()
                    }),
                    ListItem::new(format!("Keybinds - {} set", self.current_config.keybind.len())),
                ])
            }
            Window::Workspaces { index } => {
                let current_workspace = if let Some(w) = &self.current_config.workspaces {
                    w.get(index as usize)
                } else {
                    None
                };

                if let Some(c) = current_workspace {
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
                }
            }
            _ => {
                vec![]
            }
        }
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Min(3),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn format_modkey_name(modkey: String) -> String {
    match modkey.as_str() {
        "Mod1" | "Alt" => "Alt".to_string(),
        "Mod4" | "Super" => "Super".to_string(),
        _ => modkey,
    }
}
