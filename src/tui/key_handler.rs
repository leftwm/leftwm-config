use std::mem;

use anyhow::{bail, Result};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use tui::widgets::ListState;

use crate::config::command::BaseCommand;
use crate::config::filehandler::save_to_file;
use crate::config::keybind::Keybind;
use crate::config::layout::Layout as WMLayout;
use crate::config::modifier::{Modifier as KeyModifier, Modifier};
use crate::config::structs::{ScratchPad, WindowHook, Workspace};
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode, Size};
use crate::tui::{next, previous, App, MultiselectListState, PopupState, Window};
use crate::utils::xkeysym_lookup::into_keysym;
use crate::utils::{TryRemove, TryUnwrap};

pub(super) fn handle_keys(app: &mut App) -> Result<bool> {
    if let Event::Key(key) = event::read()? {
        if let Some(15) = app.current_popup {
            app.current_popup = None;
        } else {
            return match key.code {
                KeyCode::Up => up(app),
                KeyCode::Down => down(app),
                KeyCode::Right => right(app),
                KeyCode::Left => left(app),
                KeyCode::Enter => match app.current_window {
                    Window::Home => enter_home(app),
                    Window::Workspaces { index, empty } => enter_workspaces(app, index, empty),
                    Window::Tags { index, empty } => enter_tags(app, index, empty),
                    Window::WindowRules { index, empty } => enter_window_rules(app, index, empty),
                    Window::Scratchpads { index, empty } => enter_scratchpads(app, index, empty),
                    Window::KeyBinds { index, empty } => enter_keybinds(app, index, empty),
                },
                KeyCode::Esc => {
                    app.current_popup = None;
                    Ok(false)
                }
                //space
                KeyCode::Char(' ') => space(app),
                KeyCode::Char(c) => char(app, c),
                KeyCode::Backspace => backspace(app),
                KeyCode::Delete => delete(app),
                _ => Ok(false),
            };
        }
    }

    Ok(false)
}

fn up(app: &mut App) -> Result<bool> {
    if app.current_popup.is_none() {
        match app.config_list_state.selected() {
            None => app.config_list_state.select(Some(0)),
            Some(_) => {
                previous(&mut app.config_list_state, app.config_list.len());
            }
        }
    } else if let Some(s) = app.current_popup {
        match app.current_window {
            Window::Home => match s {
                0 | 1 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        previous(s, 7);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                2 | 3 | 4 | 5 => {}
                6 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        previous(s, 3);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                7 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        previous(s, 4);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                8 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        previous(s, 2);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                9 => {
                    if let PopupState::MultiList(s) = &mut app.current_popup_state {
                        previous(&mut s.liststate, 14);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                10 => {}
                11 => {}
                12 => {}
                13 => {}
                14 => {}
                _ => {}
            },
            Window::Workspaces { .. } => {
                if s == 6 {
                    if let PopupState::MultiList(s) = &mut app.current_popup_state {
                        previous(&mut s.liststate, 14);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
            }
            Window::KeyBinds { .. } => match s {
                0 => {
                    if let PopupState::List(l) = &mut app.current_popup_state {
                        previous(l, 39);
                    } else {
                        bail!("Invalid popup state")
                    }
                }
                2 => {
                    if let PopupState::MultiList(l) = &mut app.current_popup_state {
                        previous(&mut l.liststate, 8);
                    } else {
                        bail!("Invalid popup state")
                    }
                }
                3 => {}
                _ => {}
            },
            _ => {}
        }
    }

    Ok(false)
}

fn down(app: &mut App) -> Result<bool> {
    if app.current_popup.is_none() {
        match app.config_list_state.selected() {
            None => app.config_list_state.select(Some(0)),
            Some(_) => {
                next(&mut app.config_list_state, app.config_list.len());
            }
        }
    } else if let Some(s) = app.current_popup {
        match app.current_window {
            Window::Home => match s {
                0 | 1 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        next(s, 7);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                6 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        next(s, 3);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                7 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        next(s, 4);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                8 => {
                    if let PopupState::List(s) = &mut app.current_popup_state {
                        next(s, 2);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                9 => {
                    if let PopupState::MultiList(s) = &mut app.current_popup_state {
                        next(&mut s.liststate, 14);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                10 => {}
                11 => {}
                12 => {}
                13 => {}
                14 => {}
                _ => {}
            },
            Window::Workspaces { .. } => {
                if s == 6 {
                    if let PopupState::MultiList(s) = &mut app.current_popup_state {
                        next(&mut s.liststate, 14);
                    } else {
                        bail!("Invalid popup state");
                    }
                }
            }
            Window::KeyBinds { .. } => match s {
                0 => {
                    if let PopupState::List(l) = &mut app.current_popup_state {
                        next(l, 39);
                    } else {
                        bail!("Invalid popup state")
                    }
                }
                2 => {
                    if let PopupState::MultiList(l) = &mut app.current_popup_state {
                        next(&mut l.liststate, 8);
                    } else {
                        bail!("Invalid popup state")
                    }
                }
                3 => {}
                _ => {}
            },
            _ => {}
        }
    }

    Ok(false)
}

fn right(app: &mut App) -> Result<bool> {
    match app.current_window {
        Window::Workspaces { index, empty } => {
            if !empty {
                if index >= app.current_config.workspaces.as_ref().try_unwrap()?.len() - 1 {
                    app.current_window.try_set_index(0);
                } else {
                    app.current_window.try_increment_index();
                }
            }
        }
        Window::Tags { index, empty } => {
            if !empty {
                if index >= app.current_config.tags.as_ref().try_unwrap()?.len() - 1 {
                    app.current_window.try_set_index(0);
                } else {
                    app.current_window.try_increment_index();
                }
            }
        }
        Window::WindowRules { index, empty } => {
            if app.current_popup.is_some_and(|i| *i == 2) {
                if let PopupState::Int { current, min, max } = app.current_popup_state {
                    if current < max {
                        app.current_popup_state = PopupState::Int {
                            current: current + 1,
                            min,
                            max,
                        }
                    }
                } else {
                    bail!("Invalid popup state");
                }
            } else if !empty {
                if index >= app.current_config.window_rules.as_ref().try_unwrap()?.len() - 1 {
                    app.current_window.try_set_index(0);
                } else {
                    app.current_window.try_increment_index();
                }
            }
        }
        Window::Scratchpads { index, empty } => {
            if !empty {
                if index >= app.current_config.scratchpad.as_ref().try_unwrap()?.len() - 1 {
                    app.current_window.try_set_index(0);
                } else {
                    app.current_window.try_increment_index();
                }
            }
        }
        Window::KeyBinds { index, empty } => {
            if !empty {
                if index >= app.current_config.keybind.len() - 1 {
                    app.current_window.try_set_index(0);
                } else {
                    app.current_window.try_increment_index();
                }
            }
        }
        Window::Home => {}
    }

    Ok(false)
}

fn left(app: &mut App) -> Result<bool> {
    match app.current_window {
        Window::Workspaces { index, empty } => {
            if !empty {
                if index == 0 {
                    app.current_window.try_set_index(
                        app.current_config.workspaces.as_ref().try_unwrap()?.len() - 1,
                    );
                } else {
                    app.current_window.try_decrement_index();
                }
            }
        }
        Window::Tags { index, empty } => {
            if !empty {
                if index == 0 {
                    app.current_window
                        .try_set_index(app.current_config.tags.as_ref().try_unwrap()?.len() - 1);
                } else {
                    app.current_window.try_decrement_index();
                }
            }
        }
        Window::WindowRules { index, empty } => {
            if app.current_popup.is_some_and(|i| *i == 2) {
                if let PopupState::Int { current, min, max } = app.current_popup_state {
                    if current > min {
                        app.current_popup_state = PopupState::Int {
                            current: current - 1,
                            min,
                            max,
                        }
                    }
                } else {
                    bail!("Invalid popup state");
                }
            } else if !empty {
                if index == 0 {
                    app.current_window.try_set_index(
                        app.current_config.window_rules.as_ref().try_unwrap()?.len() - 1,
                    );
                } else {
                    app.current_window.try_decrement_index();
                }
            }
        }
        Window::Scratchpads { index, empty } => {
            if !empty {
                if index == 0 {
                    app.current_window.try_set_index(
                        app.current_config.scratchpad.as_ref().try_unwrap()?.len() - 1,
                    );
                } else {
                    app.current_window.try_decrement_index();
                }
            }
        }
        Window::KeyBinds { index, empty } => {
            if !empty {
                if index == 0 {
                    app.current_window
                        .try_set_index(app.current_config.keybind.len() - 1);
                } else {
                    app.current_window.try_decrement_index();
                }
            }
        }
        Window::Home => {}
    }

    Ok(false)
}

fn enter_home(app: &mut App) -> Result<bool> {
    if let Some(s) = app.config_list_state.selected() {
        if app.current_popup.is_none() {
            match s {
                0 => {
                    app.current_popup = Some(0);
                    let mut state = ListState::default();
                    match app.current_config.modkey.as_str() {
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
                    app.current_popup_state = PopupState::List(state);
                }
                1 => {
                    app.current_popup = Some(1);
                    let mut state = ListState::default();
                    match app
                        .current_config
                        .mousekey
                        .clone()
                        .unwrap_or_else(|| KeyModifier::Single("None".to_string()))
                        .to_string()
                        .as_str()
                    {
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
                    app.current_popup_state = PopupState::List(state);
                }
                2 => {
                    app.current_popup = Some(2);
                    app.current_popup_state = PopupState::String(String::new());
                }
                3 => {
                    app.current_config.disable_current_tag_swap =
                        !app.current_config.disable_current_tag_swap;
                }
                4 => {
                    app.current_config.disable_tile_drag = !app.current_config.disable_tile_drag;
                }
                5 => {
                    app.current_config.focus_new_windows = !app.current_config.focus_new_windows;
                }
                6 => {
                    app.current_popup = Some(6);
                    let index = match app.current_config.focus_behaviour {
                        FocusBehaviour::Sloppy => Some(0),
                        FocusBehaviour::ClickTo => Some(1),
                        FocusBehaviour::Driven => Some(2),
                    };
                    let mut state = ListState::default();
                    state.select(index);
                    app.current_popup_state = PopupState::List(state);
                }
                7 => {
                    app.current_popup = Some(7);
                    let index = match app.current_config.insert_behavior {
                        InsertBehavior::Top => Some(0),
                        InsertBehavior::Bottom => Some(1),
                        InsertBehavior::BeforeCurrent => Some(2),
                        InsertBehavior::AfterCurrent => Some(3),
                    };
                    let mut state = ListState::default();
                    state.select(index);
                    app.current_popup_state = PopupState::List(state);
                }
                8 => {
                    app.current_popup = Some(8);
                    let index = match app.current_config.layout_mode {
                        LayoutMode::Tag => Some(0),
                        LayoutMode::Workspace => Some(1),
                    };
                    let mut state = ListState::default();
                    state.select(index);
                    app.current_popup_state = PopupState::List(state);
                }
                9 => {
                    app.current_popup = Some(9);
                    let mut selected: Vec<usize> = vec![];
                    for l in &app.current_config.layouts {
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
                    app.current_popup_state = PopupState::MultiList(MultiselectListState {
                        liststate,
                        selected,
                    });
                }
                10 => {
                    app.current_window = Window::Workspaces {
                        index: 0,
                        empty: app
                            .current_config
                            .workspaces
                            .as_ref()
                            .is_some_and(|v| v.is_empty())
                            || app.current_config.workspaces.as_ref().is_none(),
                    };
                }
                11 => {
                    app.current_window = Window::Tags {
                        index: 0,
                        empty: app
                            .current_config
                            .tags
                            .as_ref()
                            .is_some_and(|v| v.is_empty())
                            || app.current_config.tags.as_ref().is_none(),
                    }
                }
                12 => {
                    app.current_window = Window::WindowRules {
                        index: 0,
                        empty: app
                            .current_config
                            .window_rules
                            .as_ref()
                            .is_some_and(|v| v.is_empty())
                            || app.current_config.window_rules.as_ref().is_none(),
                    }
                }
                13 => {
                    app.current_window = Window::Scratchpads {
                        index: 0,
                        empty: app
                            .current_config
                            .scratchpad
                            .as_ref()
                            .is_some_and(|v| v.is_empty())
                            || app.current_config.scratchpad.as_ref().is_none(),
                    }
                }
                14 => {
                    app.current_window = Window::KeyBinds {
                        index: 0,
                        empty: app.current_config.keybind.is_empty(),
                    }
                }
                _ => {}
            }
        } else if let Some(s) = app.current_popup {
            match s {
                0 => {
                    if let PopupState::List(s) = &app.current_popup_state {
                        if let Some(s) = s.selected() {
                            match s {
                                0 => {
                                    app.current_config.modkey = "None".to_string();
                                    app.current_popup = None;
                                }
                                1 => {
                                    app.current_config.modkey = "Shift".to_string();
                                    app.current_popup = None;
                                }
                                2 => {
                                    app.current_config.modkey = "Control".to_string();
                                    app.current_popup = None;
                                }
                                3 => {
                                    app.current_config.modkey = "Mod1".to_string();
                                    app.current_popup = None;
                                }
                                4 => {
                                    app.current_config.modkey = "Mod3".to_string();
                                    app.current_popup = None;
                                }
                                5 => {
                                    app.current_config.modkey = "Mod4".to_string();
                                    app.current_popup = None;
                                }
                                6 => {
                                    app.current_config.modkey = "Mod5".to_string();
                                    app.current_popup = None;
                                }
                                _ => {}
                            }
                        }
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                1 => {
                    if let PopupState::List(s) = &app.current_popup_state {
                        if let Some(s) = s.selected() {
                            match s {
                                0 => {
                                    app.current_config.mousekey = None;
                                }
                                1 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Shift".to_string()));
                                }
                                2 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Control".to_string()));
                                }
                                3 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Mod1".to_string()));
                                }
                                4 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Mod3".to_string()));
                                }
                                5 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Mod4".to_string()));
                                }
                                6 => {
                                    app.current_config.mousekey =
                                        Some(KeyModifier::Single("Mod5".to_string()));
                                }
                                _ => {}
                            }
                        }
                        app.current_popup = None;
                    } else {
                        bail!("Invalid popup state");
                    };
                }
                2 => {
                    app.current_config.max_window_width =
                        if let PopupState::String(s) = &app.current_popup_state {
                            if s.contains('.') || s.contains(',') {
                                Some(Size::Ratio(s.parse().unwrap_or(0.0)))
                            } else {
                                Some(Size::Pixel(s.parse().unwrap_or(0)))
                            }
                        } else {
                            bail!("Invalid popup state");
                        };
                    app.current_popup = None;
                }
                6 => {
                    if let PopupState::List(l) = &app.current_popup_state {
                        match l.selected() {
                            Some(0) => {
                                app.current_config.focus_behaviour = FocusBehaviour::Sloppy;
                            }
                            Some(1) => {
                                app.current_config.focus_behaviour = FocusBehaviour::ClickTo;
                            }
                            Some(2) => {
                                app.current_config.focus_behaviour = FocusBehaviour::Driven;
                            }
                            Some(i) => {
                                bail!("index out of bounds {i}")
                            }
                            None => {}
                        }
                    } else {
                        bail!("Invalid popup state");
                    }
                    app.current_popup = None;
                }
                7 => {
                    if let PopupState::List(l) = &app.current_popup_state {
                        match l.selected() {
                            Some(0) => {
                                app.current_config.insert_behavior = InsertBehavior::Top;
                            }
                            Some(1) => {
                                app.current_config.insert_behavior = InsertBehavior::Bottom;
                            }
                            Some(2) => {
                                app.current_config.insert_behavior = InsertBehavior::BeforeCurrent;
                            }
                            Some(3) => {
                                app.current_config.insert_behavior = InsertBehavior::AfterCurrent;
                            }
                            Some(i) => {
                                bail!("index out of bounds {i}")
                            }
                            None => {}
                        }
                    } else {
                        bail!("Invalid popup state");
                    }
                    app.current_popup = None;
                }
                8 => {
                    if let PopupState::List(l) = &app.current_popup_state {
                        match l.selected() {
                            Some(0) => {
                                app.current_config.layout_mode = LayoutMode::Tag;
                            }
                            Some(1) => {
                                app.current_config.layout_mode = LayoutMode::Workspace;
                            }
                            Some(i) => {
                                bail!("index out of bounds {i}")
                            }
                            None => {}
                        }
                    } else {
                        bail!("Invalid popup state");
                    }
                    app.current_popup = None;
                }
                9 => {
                    if let PopupState::MultiList(l) = &app.current_popup_state {
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
                        app.current_config.layouts = layouts;
                        app.current_popup = None;
                    } else {
                        bail!("Invalid popup state");
                    }
                }
                _ => {}
            }
        }
    }

    Ok(false)
}

fn enter_workspaces(app: &mut App, index: usize, empty: bool) -> Result<bool> {
    if empty {
        if let Some(2) = app.config_list_state.selected() {
            app.current_window = Window::Workspaces {
                index,
                empty: false,
            };
            app.current_config
                .workspaces
                .as_mut()
                .try_unwrap()?
                .push(Workspace::default());
        }
    } else if let Some(s) = app.current_popup {
        match s {
            0 => {
                app.current_popup = None;
                if let PopupState::String(s) = &app.current_popup_state {
                    app.current_config
                        .workspaces
                        .as_mut()
                        .try_unwrap()?
                        .get_mut(index)
                        .try_unwrap()?
                        .x = s.parse().unwrap_or(0);
                } else {
                    bail!("Invalid popup state");
                }
            }
            1 => {
                app.current_popup = None;
                if let PopupState::String(s) = &app.current_popup_state {
                    app.current_config
                        .workspaces
                        .as_mut()
                        .try_unwrap()?
                        .get_mut(index)
                        .try_unwrap()?
                        .y = s.parse().unwrap_or(0);
                } else {
                    bail!("Invalid popup state");
                }
            }
            2 => {
                app.current_popup = None;
                if let PopupState::String(s) = &app.current_popup_state {
                    app.current_config
                        .workspaces
                        .as_mut()
                        .try_unwrap()?
                        .get_mut(index)
                        .try_unwrap()?
                        .width = s.parse().unwrap_or(0);
                } else {
                    bail!("Invalid popup state");
                }
            }
            3 => {
                app.current_popup = None;
                if let PopupState::String(s) = &app.current_popup_state {
                    app.current_config
                        .workspaces
                        .as_mut()
                        .try_unwrap()?
                        .get_mut(index)
                        .try_unwrap()?
                        .height = s.parse().unwrap_or(0);
                } else {
                    bail!("Invalid popup state");
                }
            }
            4 => {
                app.current_popup = None;
                if let PopupState::String(s) = &app.current_popup_state {
                    app.current_config
                        .workspaces
                        .as_mut()
                        .try_unwrap()?
                        .get_mut(index)
                        .try_unwrap()?
                        .id = Some(s.parse().unwrap_or(0));
                } else {
                    bail!("Invalid popup state");
                }
            }
            5 => {
                app.current_popup = None;
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .max_window_width = if let PopupState::String(s) = &app.current_popup_state {
                    if s.contains('.') || s.contains(',') {
                        Some(Size::Ratio(s.parse().unwrap_or(0.0)))
                    } else {
                        Some(Size::Pixel(s.parse().unwrap_or(0)))
                    }
                } else {
                    bail!("Invalid popup state");
                };
            }
            6 => {
                if let PopupState::MultiList(l) = &app.current_popup_state {
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
                    let mut workspace = app
                        .current_config
                        .workspaces
                        .as_deref_mut()
                        .try_unwrap()?
                        .get(index)
                        .cloned()
                        .try_unwrap()?;
                    workspace.layouts = Some(layouts);
                    //we are just getting rid of the thing mem::replace returns
                    #[allow(clippy::let_underscore_drop)]
                    let _ = mem::replace(
                        app.current_config
                            .workspaces
                            .as_mut()
                            .try_unwrap()?
                            .get_mut(index)
                            .try_unwrap()?,
                        workspace,
                    );
                    app.current_popup = None;
                } else {
                    bail!("Invalid popup state");
                }
            }
            _ => {}
        }
    } else if let Some(s) = app.config_list_state.selected() {
        match s {
            2 => {
                app.current_popup = Some(0);
                app.current_popup_state = PopupState::String(String::new());
            }
            3 => {
                app.current_popup = Some(1);
                app.current_popup_state = PopupState::String(String::new());
            }
            4 => {
                app.current_popup = Some(2);
                app.current_popup_state = PopupState::String(String::new());
            }
            5 => {
                app.current_popup = Some(3);
                app.current_popup_state = PopupState::String(String::new());
            }
            6 => {
                app.current_popup = Some(4);
                app.current_popup_state = PopupState::String(String::new());
            }
            7 => {
                app.current_popup = Some(5);
                app.current_popup_state = PopupState::String(String::new());
            }
            8 => {
                app.current_popup = Some(6);
                let mut selected: Vec<usize> = vec![];
                for l in app
                    .current_config
                    .workspaces
                    .as_ref()
                    .try_unwrap()?
                    .get(index)
                    .try_unwrap()?
                    .layouts
                    .as_ref()
                    .unwrap_or(&vec![])
                {
                    match l {
                        WMLayout::MainAndVertStack => selected.push(0),
                        WMLayout::MainAndHorizontalStack => {
                            selected.push(1);
                        }
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
                app.current_popup_state = PopupState::MultiList(MultiselectListState {
                    liststate,
                    selected,
                });
            }
            10 => {
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .push(Workspace::default());
            }
            11 => {
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .try_remove(index)?;
                if app
                    .current_config
                    .workspaces
                    .as_ref()
                    .try_unwrap()?
                    .is_empty()
                {
                    app.current_window = Window::Workspaces { index, empty: true };
                }
            }
            _ => {}
        }
    }

    Ok(false)
}

fn enter_tags(app: &mut App, index: usize, empty: bool) -> Result<bool> {
    match app.current_popup {
        Some(_) => {
            *app.current_config
                .tags
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()? = {
                if let PopupState::String(s) = app.current_popup_state.clone() {
                    s
                } else {
                    bail!("Invalid popup state");
                }
            };
            app.current_popup = None;
            app.current_popup_state = PopupState::None;
        }
        None => match app.config_list_state.selected().unwrap_or(0) {
            2 => {
                if empty {
                    app.current_config
                        .tags
                        .as_mut()
                        .try_unwrap()?
                        .push(String::default());
                    app.current_window = Window::Tags {
                        index,
                        empty: false,
                    };
                } else {
                    app.current_popup = Some(0);
                    app.current_popup_state = PopupState::String(String::default());
                }
            }
            4 => {
                app.current_config
                    .tags
                    .as_mut()
                    .try_unwrap()?
                    .push(String::default());
            }
            5 => {
                app.current_config
                    .tags
                    .as_mut()
                    .try_unwrap()?
                    .try_remove(index)?;
                if app.current_config.tags.as_ref().try_unwrap()?.is_empty() {
                    app.current_window = Window::Workspaces { index, empty: true };
                }
                if index >= app.current_config.tags.as_ref().try_unwrap()?.len() && index > 0 {
                    app.current_window = Window::Tags {
                        index: index - 1,
                        empty,
                    };
                } else if app.current_config.tags.as_ref().try_unwrap()?.is_empty() {
                    app.current_window = Window::Tags {
                        index: 0,
                        empty: true,
                    };
                    app.config_list_state.select(None);
                }
            }
            _ => {}
        },
    }

    Ok(false)
}

fn enter_window_rules(app: &mut App, index: usize, empty: bool) -> Result<bool> {
    match app.current_popup {
        Some(0) => {
            app.current_config
                .window_rules
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .window_title = {
                if let PopupState::String(s) = app.current_popup_state.clone() {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                } else {
                    bail!("Invalid popup state");
                }
            };
            app.current_popup = None;
            app.current_popup_state = PopupState::None;
        }
        Some(1) => {
            app.current_config
                .window_rules
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .window_class = {
                if let PopupState::String(s) = app.current_popup_state.clone() {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s)
                    }
                } else {
                    bail!("Invalid popup state");
                }
            };
            app.current_popup = None;
            app.current_popup_state = PopupState::None;
        }
        Some(2) => {
            app.current_config
                .window_rules
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .spawn_on_tag = {
                if let PopupState::Int { current, .. } = app.current_popup_state {
                    Some(current as usize)
                } else {
                    bail!("Invalid popup state")
                }
            };
            app.current_popup = None;
            app.current_popup_state = PopupState::None;
        }
        Some(_) => {}
        None => match app.config_list_state.selected() {
            Some(2) => {
                if empty {
                    if app
                        .current_config
                        .window_rules
                        .as_ref()
                        .try_unwrap()?
                        .is_empty()
                    {
                        app.current_window = Window::WindowRules {
                            index,
                            empty: false,
                        }
                    }
                    app.current_config
                        .window_rules
                        .as_mut()
                        .try_unwrap()?
                        .push(WindowHook::default());
                } else {
                    app.current_popup = Some(0);
                    app.current_popup_state = PopupState::String(
                        app.current_config
                            .window_rules
                            .as_ref()
                            .try_unwrap()?
                            .get(index)
                            .try_unwrap()?
                            .window_title
                            .clone()
                            .unwrap_or_default(),
                    );
                }
            }
            Some(3) => {
                app.current_popup = Some(1);
                app.current_popup_state = PopupState::String(
                    app.current_config
                        .window_rules
                        .as_ref()
                        .try_unwrap()?
                        .get(index)
                        .try_unwrap()?
                        .window_class
                        .clone()
                        .unwrap_or_default(),
                );
            }
            Some(4) => {
                app.current_popup = Some(2);
                app.current_popup_state = PopupState::Int {
                    current: app
                        .current_config
                        .window_rules
                        .as_ref()
                        .try_unwrap()?
                        .get(index)
                        .try_unwrap()?
                        .spawn_on_tag
                        .unwrap_or_default() as isize,
                    min: 1,
                    max: app.current_config.tags.as_ref().try_unwrap()?.len() as isize,
                }
            }
            Some(5) => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .spawn_floating = Some(
                    !app.current_config
                        .window_rules
                        .as_ref()
                        .try_unwrap()?
                        .get(index)
                        .try_unwrap()?
                        .spawn_floating
                        .unwrap_or(false),
                );
            }
            Some(7) => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .push(WindowHook::default());
            }
            Some(8) => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .try_remove(index)?;
                if index == app.current_config.window_rules.as_ref().try_unwrap()?.len()
                    && !app
                        .current_config
                        .window_rules
                        .as_ref()
                        .try_unwrap()?
                        .is_empty()
                {
                    app.current_window = Window::WindowRules {
                        index: index - 1,
                        empty,
                    };
                } else if app
                    .current_config
                    .window_rules
                    .as_ref()
                    .try_unwrap()?
                    .is_empty()
                {
                    app.current_window = Window::WindowRules {
                        index: 0,
                        empty: true,
                    };
                    app.config_list_state.select(None);
                }
            }
            _ => {}
        },
    }

    Ok(false)
}

fn enter_scratchpads(app: &mut App, index: usize, empty: bool) -> Result<bool> {
    match app.current_popup {
        Some(0) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .name = {
                if let PopupState::String(s) = app.current_popup_state.clone() {
                    s
                } else {
                    bail!("Invalid popup state")
                }
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }
        Some(1) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .value = {
                if let PopupState::String(s) = app.current_popup_state.clone() {
                    s
                } else {
                    bail!("Invalid popup state")
                }
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }
        Some(2) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .x = if let PopupState::String(s) = &app.current_popup_state {
                if s.contains('.') || s.contains(',') {
                    Some(Size::Ratio(s.parse()?))
                } else {
                    Some(Size::Pixel(s.parse()?))
                }
            } else {
                bail!("Invalid popup state")
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }
        Some(3) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .y = if let PopupState::String(s) = &app.current_popup_state {
                if s.contains('.') || s.contains(',') {
                    Some(Size::Ratio(s.parse()?))
                } else {
                    Some(Size::Pixel(s.parse()?))
                }
            } else {
                bail!("Invalid popup state")
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }
        Some(4) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .width = if let PopupState::String(s) = &app.current_popup_state {
                if s.contains('.') || s.contains(',') {
                    Some(Size::Ratio(s.parse()?))
                } else {
                    Some(Size::Pixel(s.parse()?))
                }
            } else {
                bail!("Invalid popup state")
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }
        Some(5) => {
            app.current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .get_mut(index)
                .try_unwrap()?
                .height = if let PopupState::String(s) = &app.current_popup_state {
                if s.contains('.') || s.contains(',') {
                    Some(Size::Ratio(s.parse()?))
                } else {
                    Some(Size::Pixel(s.parse()?))
                }
            } else {
                bail!("Invalid popup state")
            };
            app.current_popup_state = PopupState::None;
            app.current_popup = None;
        }

        None => match app.config_list_state.selected() {
            Some(2) => {
                if empty {
                    app.current_config
                        .scratchpad
                        .as_mut()
                        .try_unwrap()?
                        .push(ScratchPad::default());
                    app.current_window = Window::Scratchpads {
                        index,
                        empty: false,
                    };
                } else {
                    app.current_popup = Some(0);
                    app.current_popup_state = PopupState::String(
                        app.current_config
                            .scratchpad
                            .as_ref()
                            .try_unwrap()?
                            .get(index)
                            .try_unwrap()?
                            .name
                            .clone(),
                    );
                }
            }
            Some(3) => {
                app.current_popup = Some(1);
                app.current_popup_state = PopupState::String(
                    app.current_config
                        .scratchpad
                        .as_ref()
                        .try_unwrap()?
                        .get(index)
                        .try_unwrap()?
                        .value
                        .clone(),
                );
            }
            Some(i @ 4..=7) => {
                app.current_popup = Some(i as u8 - 2);
                app.current_popup_state = PopupState::String(String::new());
            }
            Some(9) => app
                .current_config
                .scratchpad
                .as_mut()
                .try_unwrap()?
                .push(ScratchPad::default()),

            Some(10) => {
                app.current_config
                    .scratchpad
                    .as_mut()
                    .try_unwrap()?
                    .try_remove(index)?;
                if index == app.current_config.scratchpad.as_ref().try_unwrap()?.len()
                    && !app
                        .current_config
                        .scratchpad
                        .as_ref()
                        .try_unwrap()?
                        .is_empty()
                {
                    app.current_window = Window::Scratchpads {
                        index: index - 1,
                        empty,
                    };
                } else if app
                    .current_config
                    .scratchpad
                    .as_ref()
                    .try_unwrap()?
                    .is_empty()
                {
                    app.current_window = Window::Scratchpads {
                        index: 0,
                        empty: true,
                    };
                    app.config_list_state.select(None);
                }
            }
            _ => {}
        },
        _ => {}
    }

    Ok(false)
}

fn enter_keybinds(app: &mut App, index: usize, empty: bool) -> Result<bool> {
    if empty && app.config_list_state.selected().is_some_and(|i| *i == 2) {
        app.current_config.keybind.push(Keybind::default());
    } else {
        match app.current_popup {
            Some(0) => {
                app.current_config
                    .keybind
                    .get_mut(index)
                    .try_unwrap()?
                    .command = if let PopupState::List(l) = &app.current_popup_state {
                    match l.selected() {
                        Some(0) => BaseCommand::Execute,
                        Some(1) => BaseCommand::CloseWindow,
                        Some(2) => BaseCommand::SwapTags,
                        Some(3) => BaseCommand::SoftReload,
                        Some(4) => BaseCommand::HardReload,
                        Some(5) => BaseCommand::ToggleScratchPad,
                        Some(6) => BaseCommand::ToggleFullScreen,
                        Some(7) => BaseCommand::ToggleSticky,
                        Some(8) => BaseCommand::GotoTag,
                        Some(9) => BaseCommand::ReturnToLastTag,
                        Some(10) => BaseCommand::FloatingToTile,
                        Some(11) => BaseCommand::TileToFloating,
                        Some(12) => BaseCommand::ToggleFloating,
                        Some(13) => BaseCommand::MoveWindowUp,
                        Some(14) => BaseCommand::MoveWindowDown,
                        Some(15) => BaseCommand::MoveWindowTop,
                        Some(16) => BaseCommand::FocusNextTag,
                        Some(17) => BaseCommand::FocusPreviousTag,
                        Some(18) => BaseCommand::FocusWindow,
                        Some(19) => BaseCommand::FocusWindowUp,
                        Some(20) => BaseCommand::FocusWindowDown,
                        Some(21) => BaseCommand::FocusWindowTop,
                        Some(22) => BaseCommand::FocusWorkspaceNext,
                        Some(23) => BaseCommand::FocusWorkspacePrevious,
                        Some(24) => BaseCommand::MoveToTag,
                        Some(25) => BaseCommand::MoveToLastWorkspace,
                        Some(26) => BaseCommand::MoveWindowToNextWorkspace,
                        Some(27) => BaseCommand::MoveWindowToPreviousWorkspace,
                        Some(28) => BaseCommand::MouseMoveWindow,
                        Some(29) => BaseCommand::NextLayout,
                        Some(30) => BaseCommand::PreviousLayout,
                        Some(31) => BaseCommand::SetLayout,
                        Some(32) => BaseCommand::RotateTag,
                        Some(33) => BaseCommand::IncreaseMainWidth,
                        Some(34) => BaseCommand::DecreaseMainWidth,
                        Some(35) => BaseCommand::SetMarginMultiplier,
                        Some(36) => BaseCommand::UnloadTheme,
                        Some(37) => BaseCommand::LoadTheme,
                        Some(38) => BaseCommand::CloseAllOtherWindows,
                        _ => {
                            bail!("Unexpected value")
                        }
                    }
                } else {
                    bail!("Invalid popup _state")
                };
                app.current_popup = None;
                app.current_popup_state = PopupState::None;
            }
            Some(1) => {
                app.current_config
                    .keybind
                    .get_mut(index)
                    .try_unwrap()?
                    .value = if let PopupState::String(s) = app.current_popup_state.clone() {
                    s
                } else {
                    bail!("Invalid popup state")
                };
                app.current_popup = None;
                app.current_popup_state = PopupState::None;
            }
            Some(2) => {
                if let PopupState::MultiList(m) = &mut app.current_popup_state {
                    let modkey_index = match app.current_config.modkey.as_str() {
                        "Shift" => 1,
                        "Control" => 2,
                        "Mod1" | "Alt" => 3,
                        "Mod3" => 4,
                        "Mod4" | "Super" => 5,
                        "Mod5" => 6,
                        _ => bail!("unexpected value"),
                    };

                    if m.selected.len() == 1 {
                        let modifier = match m.selected.first().try_unwrap()? {
                            i if *i == modkey_index => Some("modkey".to_string()),
                            0 => None,
                            1 => Some("Shift".to_string()),
                            2 => Some("Control".to_string()),
                            3 => Some("Mod1".to_string()),
                            4 => Some("Mod4".to_string()),
                            5 => Some("Mod5".to_string()),
                            6 => Some("Mod5".to_string()),
                            7 => Some("modkey".to_string()),
                            _ => bail!("Unexpected value"),
                        };

                        app.current_config
                            .keybind
                            .get_mut(index)
                            .try_unwrap()?
                            .modifier = modifier.map(Modifier::Single);
                    } else {
                        let mut keys = vec![];

                        for i in &m.selected {
                            if *i == modkey_index {
                                continue;
                            }

                            match i {
                                1 => keys.push("Shift".to_string()),
                                2 => keys.push("Control".to_string()),
                                3 => keys.push("Mod1".to_string()),
                                4 => keys.push("Mod4".to_string()),
                                5 => keys.push("Mod5".to_string()),
                                6 => keys.push("Mod5".to_string()),
                                7 => keys.push("modkey".to_string()),
                                _ => bail!("Unexpected value"),
                            };
                        }
                        app.current_config
                            .keybind
                            .get_mut(index)
                            .try_unwrap()?
                            .modifier = if keys.is_empty() {
                            None
                        } else {
                            Some(Modifier::List(keys))
                        };
                    }
                    app.current_popup = None;
                    app.current_popup_state = PopupState::None;
                }
            }
            Some(3) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    if into_keysym(s).is_some() {
                        app.current_config.keybind.get_mut(index).try_unwrap()?.key = s.clone();
                        app.current_popup = None;
                        app.current_popup_state = PopupState::None;
                    }
                }
            }
            None => {
                if app
                    .current_config
                    .keybind
                    .get(index)
                    .try_unwrap()?
                    .command
                    .needs_value()
                {
                    match app.config_list_state.selected() {
                        Some(2) => {
                            app.current_popup = Some(0);
                            app.current_popup_state = PopupState::List(ListState::default());
                        }
                        Some(3) => {
                            app.current_popup = Some(1);
                            app.current_popup_state = PopupState::String(
                                app.current_config
                                    .keybind
                                    .get(index)
                                    .try_unwrap()?
                                    .value
                                    .clone(),
                            );
                        }
                        Some(4) => {
                            let selected = if app
                                .current_config
                                .keybind
                                .get(index)
                                .try_unwrap()?
                                .modifier
                                .is_none()
                            {
                                vec![]
                            } else {
                                let mut modifier = match app
                                    .current_config
                                    .keybind
                                    .get(index)
                                    .try_unwrap()?
                                    .modifier
                                    .as_ref()
                                    .unwrap()
                                {
                                    Modifier::Single(s) => {
                                        match s.as_str() {
                                            "None" => vec![0],
                                            "Shift" => vec![1],
                                            "Control" => vec![2],
                                            "Mod1" | "Alt" => vec![3],
                                            //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                            //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                            "Mod3" => vec![4],
                                            "Mod4" | "Super" => vec![5],
                                            "Mod5" => vec![6],
                                            "modkey" => vec![7],
                                            _ => vec![],
                                        }
                                    }
                                    Modifier::List(s) => {
                                        let mut vec = vec![];
                                        for s in s {
                                            match s.as_str() {
                                                "None" => vec.push(0),
                                                "Shift" => vec.push(1),
                                                "Control" => vec.push(2),
                                                "Mod1" | "Alt" => vec.push(3),
                                                //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                                //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                                "Mod3" => vec.push(4),
                                                "Mod4" | "Super" => vec.push(5),
                                                "Mod5" => vec.push(6),
                                                "modkey" => vec.push(7),
                                                _ => (),
                                            }
                                        }

                                        vec
                                    }
                                };

                                let modkey_index = match app.current_config.modkey.as_str() {
                                    "Shift" => 1,
                                    "Control" => 2,
                                    "Mod1" | "Alt" => 3,
                                    "Mod3" => 4,
                                    "Mod4" | "Super" => 5,
                                    "Mod5" => 6,
                                    _ => bail!("unexpected value"),
                                };

                                if modifier.contains(&modkey_index) {
                                    modifier.push(7);
                                } else if modifier.contains(&7) {
                                    modifier.push(modkey_index);
                                }

                                modifier
                            };

                            app.current_popup = Some(2);
                            app.current_popup_state = PopupState::MultiList(MultiselectListState {
                                liststate: ListState::default(),
                                selected,
                            });
                        }
                        Some(5) => {
                            app.current_popup = Some(3);
                            app.current_popup_state = PopupState::String(
                                app.current_config
                                    .keybind
                                    .get(index)
                                    .try_unwrap()?
                                    .key
                                    .clone(),
                            );
                        }
                        Some(7) => {
                            app.current_config.keybind.push(Keybind::default());
                        }
                        Some(8) => {
                            app.current_config.keybind.try_remove(index)?;
                            app.current_window.try_decrement_index();
                        }
                        _ => {}
                    }
                } else {
                    match app.config_list_state.selected() {
                        Some(2) => {
                            app.current_popup = Some(0);
                            app.current_popup_state = PopupState::List(ListState::default());
                        }
                        Some(3) => {
                            let selected = if app
                                .current_config
                                .keybind
                                .get(index)
                                .try_unwrap()?
                                .modifier
                                .is_none()
                            {
                                vec![]
                            } else {
                                let mut modifier = match app
                                    .current_config
                                    .keybind
                                    .get(index)
                                    .try_unwrap()?
                                    .modifier
                                    .as_ref()
                                    .unwrap()
                                {
                                    Modifier::Single(s) => {
                                        match s.as_str() {
                                            "None" => vec![0],
                                            "Shift" => vec![1],
                                            "Control" => vec![2],
                                            "Mod1" | "Alt" => vec![3],
                                            //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                            //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                            "Mod3" => vec![4],
                                            "Mod4" | "Super" => vec![5],
                                            "Mod5" => vec![6],
                                            "modkey" => vec![7],
                                            _ => vec![],
                                        }
                                    }
                                    Modifier::List(s) => {
                                        let mut vec = vec![];
                                        for s in s {
                                            match s.as_str() {
                                                "None" => vec.push(0),
                                                "Shift" => vec.push(1),
                                                "Control" => vec.push(2),
                                                "Mod1" | "Alt" => vec.push(3),
                                                //"Mod2" => xlib::Mod2Mask,     // NOTE: we are ignoring the state of Numlock
                                                //"NumLock" => xlib::Mod2Mask,  // this is left here as a reminder
                                                "Mod3" => vec.push(4),
                                                "Mod4" | "Super" => vec.push(5),
                                                "Mod5" => vec.push(6),
                                                "modkey" => vec.push(7),
                                                _ => (),
                                            }
                                        }

                                        vec
                                    }
                                };

                                let modkey_index = match app.current_config.modkey.as_str() {
                                    "Shift" => 1,
                                    "Control" => 2,
                                    "Mod1" | "Alt" => 3,
                                    "Mod3" => 4,
                                    "Mod4" | "Super" => 5,
                                    "Mod5" => 6,
                                    _ => bail!("unexpected value"),
                                };

                                if modifier.contains(&modkey_index) {
                                    modifier.push(7);
                                } else if modifier.contains(&7) {
                                    modifier.push(modkey_index);
                                }

                                modifier
                            };

                            app.current_popup = Some(2);
                            app.current_popup_state = PopupState::MultiList(MultiselectListState {
                                liststate: ListState::default(),
                                selected,
                            });
                        }
                        Some(4) => {
                            app.current_popup = Some(3);
                            app.current_popup_state = PopupState::String(
                                app.current_config
                                    .keybind
                                    .get(index)
                                    .try_unwrap()?
                                    .key
                                    .clone(),
                            );
                        }
                        Some(6) => {
                            app.current_config.keybind.push(Keybind::default());
                        }
                        Some(7) => {
                            app.current_config.keybind.try_remove(index)?;
                            app.current_window.try_decrement_index();
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

fn space(app: &mut App) -> Result<bool> {
    match app.current_window {
        Window::Home => {
            if let Some(9) = app.current_popup {
                if let PopupState::MultiList(l) = &mut app.current_popup_state {
                    if l.selected.contains(&l.liststate.selected().unwrap_or(14)) {
                        let index = l
                            .selected
                            .iter()
                            .position(|x| *x == l.liststate.selected().unwrap_or(14))
                            .try_unwrap()?;
                        l.selected.try_remove(index)?;
                    } else {
                        l.selected.push(l.liststate.selected().unwrap_or(14));
                    }
                } else {
                    bail!("Invalid popup state");
                }
            }
        }
        Window::Workspaces { .. } => {
            if let Some(6) = app.current_popup {
                if let PopupState::MultiList(l) = &mut app.current_popup_state {
                    if l.selected.contains(&l.liststate.selected().unwrap_or(14)) {
                        let index = l
                            .selected
                            .iter()
                            .position(|x| *x == l.liststate.selected().unwrap_or(14))
                            .try_unwrap()?;
                        l.selected.try_remove(index)?;
                    } else {
                        l.selected.push(l.liststate.selected().unwrap_or(14));
                    }
                } else {
                    bail!("Invalid popup state");
                }
            }
        }
        Window::Scratchpads { .. } => {
            if let Some(1) = app.current_popup {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(' ');
                } else {
                    bail!("Invalid popup state")
                }
            }
        }
        Window::KeyBinds { .. } => match app.current_popup {
            Some(1) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(' ');
                } else {
                    bail!("Invalid popup state")
                }
            }
            Some(2) => {
                if let PopupState::MultiList(s) = &mut app.current_popup_state {
                    if let Some(i) = s.liststate.selected() {
                        let modkey_index = match app.current_config.modkey.as_str() {
                            "Shift" => 1,
                            "Control" => 2,
                            "Mod1" | "Alt" => 3,
                            "Mod3" => 4,
                            "Mod4" | "Super" => 5,
                            "Mod5" => 6,
                            _ => bail!("unexpected value"),
                        };

                        if i == 0 {
                            s.selected.clear();
                            s.selected.push(0);
                        } else {
                            if s.selected.contains(&0) {
                                let index = s.selected.iter().position(|x| *x == 0).try_unwrap()?;
                                s.selected.remove(index);
                            }
                            if s.selected.contains(&i) {
                                if i == modkey_index {
                                    let index =
                                        s.selected.iter().position(|x| *x == 7).try_unwrap()?;
                                    s.selected.remove(index);
                                } else if i == 7 {
                                    let index = s
                                        .selected
                                        .iter()
                                        .position(|x| *x == modkey_index)
                                        .try_unwrap()?;
                                    s.selected.remove(index);
                                }
                                let index = s.selected.iter().position(|x| *x == i).try_unwrap()?;
                                s.selected.remove(index);
                            } else {
                                if i == modkey_index {
                                    s.selected.push(7);
                                } else if i == 7 {
                                    s.selected.push(modkey_index);
                                }
                                s.selected.push(i);
                            }
                        }
                    }
                } else {
                    bail!("Invalid popup state")
                }
            }
            _ => {}
        },
        _ => {}
    }

    Ok(false)
}

fn char(app: &mut App, c: char) -> Result<bool> {
    match app.current_window {
        Window::Home => match app.current_popup {
            Some(2) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    if "1234567890,.".contains(c) {
                        s.push(c);
                    }
                } else {
                    bail!("Invalid popup state");
                }
            }
            Some(_) => {}
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
        },
        Window::Workspaces { .. } => match app.current_popup {
            Some(0..=5) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    if "1234567890,.".contains(c) {
                        s.push(c);
                    }
                } else {
                    bail!("Invalid popup state");
                }
            }
            Some(_) => {}
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
        },
        Window::Tags { .. } => match app.current_popup {
            Some(0) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(c);
                } else {
                    bail!("Invalid popup state");
                }
            }
            Some(_) => {}
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
        },
        Window::WindowRules { .. } => match app.current_popup {
            Some(0 | 1) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(c);
                } else {
                    bail!("Invalid popup state");
                }
            }
            Some(_) => {}
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
        },
        Window::Scratchpads { .. } => match app.current_popup {
            Some(0 | 1) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(c);
                } else {
                    bail!("Invalid popup state")
                }
            }
            Some(2..=5) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    if "1234567890,.".contains(c) {
                        s.push(c);
                    }
                } else {
                    bail!("Invalid popup state");
                }
            }
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
            _ => {}
        },
        Window::KeyBinds { .. } => match app.current_popup {
            Some(1 | 3) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.push(c);
                } else {
                    bail!("Invalid popup state")
                }
            }
            None => match c {
                'q' => {
                    return Ok(true);
                }
                's' => {
                    save_to_file(&app.current_config)?;
                    app.current_popup = Some(15);
                    app.current_popup_state = PopupState::None;
                }
                _ => {}
            },
            _ => {}
        },
    }

    Ok(false)
}

fn backspace(app: &mut App) -> Result<bool> {
    match app.current_window {
        Window::Home => {
            if let Some(2) = app.current_popup {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state");
                }
            }
        }
        Window::Workspaces { .. } => {
            if let Some(0..=5) = app.current_popup {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state");
                }
            } else {
                app.current_window = Window::Home;
                //nuke any default workspaces
                let workspaces: Vec<Workspace> = app
                    .current_config
                    .workspaces
                    .as_ref()
                    .try_unwrap()?
                    .iter()
                    .cloned()
                    .filter(|w| w.eq(&Workspace::default()))
                    .collect::<Vec<Workspace>>();
                app.current_config.workspaces = Some(workspaces);
            }
        }
        Window::Tags { .. } => {
            if let Some(0) = app.current_popup {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state");
                }
            } else {
                app.current_window = Window::Home;
                //nuke any default workspaces
                let workspaces: Vec<Workspace> = app
                    .current_config
                    .workspaces
                    .as_ref()
                    .try_unwrap()?
                    .iter()
                    .cloned()
                    .filter(|w| w.eq(&Workspace::default()))
                    .collect::<Vec<Workspace>>();
                app.current_config.workspaces = Some(workspaces);
            }
        }
        Window::WindowRules { .. } => match app.current_popup {
            Some(0 | 1) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state");
                }
            }
            Some(2) => {}
            Some(3) => {}
            None => app.current_window = Window::Home,
            _ => {}
        },
        Window::Scratchpads { .. } => match app.current_popup {
            Some(0..=5) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state");
                }
            }
            None => app.current_window = Window::Home,
            _ => {}
        },
        Window::KeyBinds { .. } => match app.current_popup {
            Some(1 | 3) => {
                if let PopupState::String(s) = &mut app.current_popup_state {
                    s.pop();
                } else {
                    bail!("Invalid popup state")
                }
            }
            None => app.current_window = Window::Home,
            _ => {}
        },
    }

    Ok(false)
}

fn delete(app: &mut App) -> Result<bool> {
    match app.current_window {
        Window::Workspaces { index, .. } => match app.config_list_state.selected().unwrap_or(0) {
            6 => {
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .id = None;
            }
            7 => {
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .max_window_width = None;
            }
            8 => {
                app.current_config
                    .workspaces
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .layouts = None;
            }
            _ => {}
        },
        Window::WindowRules { index, .. } => match app.config_list_state.selected().unwrap_or(0) {
            2 => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .window_title = None;
            }
            3 => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .window_class = None;
            }
            4 => {
                app.current_config
                    .window_rules
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .spawn_on_tag = None;
            }
            _ => {}
        },
        Window::Scratchpads { index, .. } => match app.config_list_state.selected().unwrap_or(0) {
            4 => {
                app.current_config
                    .scratchpad
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .x = None;
            }
            5 => {
                app.current_config
                    .scratchpad
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .y = None;
            }
            6 => {
                app.current_config
                    .scratchpad
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .width = None;
            }
            7 => {
                app.current_config
                    .scratchpad
                    .as_mut()
                    .try_unwrap()?
                    .get_mut(index)
                    .try_unwrap()?
                    .height = None;
            }
            _ => {}
        },
        _ => {}
    }

    Ok(false)
}
