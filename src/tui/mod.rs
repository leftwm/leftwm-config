use std::time::Duration;

use anyhow::Result;
use tui_realm_stdlib::{Label, Table};
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
use tuirealm::{AttrValue, Attribute};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

use crate::config::modifier::Modifier as KeyModifier;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode, Size};
use crate::config::{filehandler, Config};

use self::popups::MaxWindowWidthHint;

mod popups;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    SetPopup(Option<Popup>),
    // Config, close_popup
    UpdateConfig(ConfigUpdate, bool),
    None,
}

#[derive(Debug, PartialEq)]
pub enum ConfigUpdate {
    ModKey(String),
    MouseKey(Option<KeyModifier>),
    MaxWindowWidth(Option<Size>),
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    HomeView,
    Hints,
    ModKeyEditor,
    ModKeyHint,
    MouseKeyEditor,
    MouseKeyHint,
    MaxWindowWidthEditor,
    MaxWindowWidthHint,
}

pub enum View {
    Home,
}

#[derive(Debug, PartialEq)]
pub enum Popup {
    ModKey,
    MouseKey,
    MaxWindowWidth,
}

struct Model {
    alive: bool,
    dirty: bool,
    view: View,
    popup: Option<Popup>,
    config: Config,
    app: Application<Id, Msg, NoUserEvent>,
}

impl Model {
    fn try_new() -> Result<Self> {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );

        let config = filehandler::load();

        app.mount(Id::HomeView, Box::new(HomeView::new(&config)), vec![])?;
        app.mount(Id::Hints, Box::new(Hints::new()), vec![])?;

        app.mount(
            Id::ModKeyEditor,
            Box::new(popups::ModKeyEditor::new(&config)),
            vec![],
        )?;
        app.mount(Id::ModKeyHint, Box::new(popups::ModKeyHint::new()), vec![])?;

        app.mount(
            Id::MouseKeyEditor,
            Box::new(popups::MouseKeyEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::MouseKeyHint,
            Box::new(popups::MouseKeyHint::new()),
            vec![],
        )?;

        app.mount(
            Id::MaxWindowWidthEditor,
            Box::new(popups::MaxWindowWidthEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::MaxWindowWidthHint,
            Box::new(MaxWindowWidthHint::new()),
            vec![],
        )?;

        app.active(&Id::HomeView)?;

        Ok(Self {
            alive: true,
            dirty: true,
            view: View::Home,
            popup: None,
            config,
            app,
        })
    }
}

impl Model {
    fn view(&mut self, terminal: &mut TerminalBridge) -> Result<()> {
        terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            match self.view {
                View::Home => {
                    self.app.view(&Id::HomeView, f, chunks[1]);
                }
            }

            let popup_space = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(
                    Layout::default()
                        .direction(LayoutDirection::Horizontal)
                        .margin(1)
                        .constraints(
                            [
                                Constraint::Percentage(25),
                                Constraint::Percentage(50),
                                Constraint::Percentage(25),
                            ]
                            .as_ref(),
                        )
                        .split(chunks[1])[1],
                );

            match self.popup {
                Some(Popup::ModKey) => {
                    self.app.view(&Id::ModKeyEditor, f, popup_space[1]);
                    self.app.view(&Id::ModKeyHint, f, popup_space[2]);
                }
                Some(Popup::MouseKey) => {
                    self.app.view(&Id::MouseKeyEditor, f, popup_space[1]);
                    self.app.view(&Id::MouseKeyHint, f, popup_space[2]);
                }
                Some(Popup::MaxWindowWidth) => {
                    let space = Layout::default()
                        .direction(LayoutDirection::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Max(0),
                            Constraint::Length(3),
                            Constraint::Max(0),
                        ])
                        .split(popup_space[1]);
                    self.app.view(&Id::MaxWindowWidthEditor, f, space[1]);
                    self.app.view(&Id::MaxWindowWidthHint, f, popup_space[2]);
                }
                None => {}
            }

            self.app.view(&Id::Hints, f, chunks[2]);
        })?;
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let mut model = Model::try_new()?;
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    terminal.enable_raw_mode()?;
    terminal.enter_alternate_screen()?;
    // Now we use the Model struct to keep track of some states

    while model.alive {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages.into_iter() {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // Redraw
        model.view(&mut terminal)?;
    }
    // Terminate terminal
    terminal.leave_alternate_screen()?;
    terminal.disable_raw_mode()?;
    terminal.clear_screen()?;

    Ok(())
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.dirty = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.alive = false;
                None
            }
            Msg::SetPopup(p) => {
                match p {
                    Some(Popup::ModKey) => self.app.active(&Id::ModKeyEditor),
                    Some(Popup::MouseKey) => self.app.active(&Id::MouseKeyEditor),
                    Some(Popup::MaxWindowWidth) => self.app.active(&Id::MaxWindowWidthEditor),
                    None => self.app.active(&Id::HomeView),
                }
                .unwrap();
                self.popup = p;
                None
            }
            Msg::UpdateConfig(config_update, close_popup) => {
                match config_update {
                    ConfigUpdate::ModKey(key) => {
                        self.config.modkey = key;
                        self.app
                            .remount(
                                Id::ModKeyEditor,
                                Box::new(popups::ModKeyEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::MouseKey(key) => {
                        self.config.mousekey = key;
                        self.app
                            .remount(
                                Id::MouseKeyEditor,
                                Box::new(popups::MouseKeyEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::MaxWindowWidth(mww) => {
                        self.config.max_window_width = mww;
                        self.app
                            .remount(
                                Id::MaxWindowWidthEditor,
                                Box::new(popups::MaxWindowWidthEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                }
                self.app
                    .remount(Id::HomeView, Box::new(HomeView::new(&self.config)), vec![])
                    .unwrap();
                if close_popup {
                    Some(Msg::SetPopup(None))
                } else {
                    None
                }
            }
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct HomeView {
    component: Table,
}

impl HomeView {
    fn new(config: &Config) -> Self {
        Self {
            component: Table::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::White),
                )
                .title("LeftWM Config", Alignment::Center)
                .scroll(true)
                .highlighted_color(Color::DarkGray)
                .highlighted_str("> ")
                .rewind(true)
                .step(4)
                .row_height(1)
                .headers(&["Option", "Value"])
                .column_spacing(3)
                .widths(&[50, 50])
                .table(Self::build_inner(config)),
        }
    }

    fn build_inner(config: &Config) -> Vec<Vec<TextSpan>> {
        TableBuilder::default()
            .add_col(TextSpan::from("Modkey"))
            .add_col(TextSpan::from(format_modkey_name(config.modkey.clone())))
            .add_row()
            .add_col(TextSpan::from("Mousekey"))
            .add_col(TextSpan::from(format_modkey_name(
                config
                    .mousekey
                    .clone()
                    .unwrap_or_else(|| KeyModifier::Single("None".to_string()))
                    .to_string(),
            )))
            .add_row()
            .add_col(TextSpan::from("Max Window Width"))
            .add_col(TextSpan::from(match config.max_window_width {
                Some(Size::Pixel(w)) => format!("{} px", w),
                Some(Size::Ratio(w)) => format!("{} %", (w * 100f32) as i32),
                None => "Not set".to_string(),
            }))
            .add_row()
            .add_col(TextSpan::from("Disable Current Tag Swap"))
            .add_col(TextSpan::from(format!(
                "{}",
                config.disable_current_tag_swap
            )))
            .add_row()
            .add_col(TextSpan::from("Disable Tile Drag"))
            .add_col(TextSpan::from(format!("{}", config.disable_tile_drag)))
            .add_row()
            .add_col(TextSpan::from("Focus New Windows"))
            .add_col(TextSpan::from(format!("{}", config.focus_new_windows)))
            .add_row()
            .add_col(TextSpan::from("Focus Behavior"))
            .add_col(TextSpan::from(match config.focus_behaviour {
                FocusBehaviour::Sloppy => "Sloppy".to_string(),
                FocusBehaviour::ClickTo => "Click To".to_string(),
                FocusBehaviour::Driven => "Driven".to_string(),
            }))
            .add_row()
            .add_col(TextSpan::from("Insert Behavior"))
            .add_col(TextSpan::from(match config.insert_behavior {
                InsertBehavior::Top => "Top".to_string(),
                InsertBehavior::Bottom => "Bottom".to_owned(),
                InsertBehavior::BeforeCurrent => "Before Current".to_string(),
                InsertBehavior::AfterCurrent => "After Current".to_string(),
            }))
            .add_row()
            .add_col(TextSpan::from("Layout Mode"))
            .add_col(TextSpan::from(match config.layout_mode {
                LayoutMode::Tag => "Tag".to_string(),
                LayoutMode::Workspace => "Workspace".to_string(),
            }))
            .add_row()
            .add_col(TextSpan::from("Layouts"))
            .add_col(TextSpan::from(format!("{} set", config.layouts.len())))
            .add_row()
            .add_col(TextSpan::from("Workspaces"))
            .add_col(TextSpan::from(format!(
                "{} set",
                config.workspaces.as_ref().unwrap_or(&vec![]).len()
            )))
            .add_row()
            .add_col(TextSpan::from("Tags"))
            .add_col(TextSpan::from(format!(
                "{} set",
                config.tags.as_ref().unwrap_or(&vec![]).len()
            )))
            .add_row()
            .add_col(TextSpan::from("Window Rules"))
            .add_col(TextSpan::from(format!(
                "{} set",
                config.window_rules.as_ref().unwrap_or(&vec![]).len()
            )))
            .add_row()
            .add_col(TextSpan::from("Scratchpads"))
            .add_col(TextSpan::from(format!(
                "{} set",
                config.scratchpad.as_ref().unwrap_or(&vec![]).len()
            )))
            .add_row()
            .add_col(TextSpan::from("Keybinds"))
            .add_col(TextSpan::from(format!("{} set", config.keybind.len())))
            .build()
    }

    fn update(&mut self, config: &Config) {
        self.component.attr(
            Attribute::Content,
            AttrValue::Table(Self::build_inner(config)),
        );
    }
}

impl Component<Msg, NoUserEvent> for HomeView {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('q'),
                ..
            }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                match self.component.states.list_index {
                    0 => return Some(Msg::SetPopup(Some(Popup::ModKey))),
                    1 => return Some(Msg::SetPopup(Some(Popup::MouseKey))),
                    2 => return Some(Msg::SetPopup(Some(Popup::MaxWindowWidth))),
                    _ => {}
                }
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        None
    }
}

#[derive(MockComponent)]
struct Hints {
    component: Label,
}

impl Hints {
    fn new() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Center)
                .foreground(Color::White)
                .text("q: Quit, Enter: Edit"),
        }
    }
}

impl Component<Msg, NoUserEvent> for Hints {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Msg> {
        None
    }
}

fn format_modkey_name(modkey: String) -> String {
    match modkey.as_str() {
        "Mod1" | "Alt" => "Alt".to_string(),
        "Mod4" | "Super" => "Super".to_string(),
        _ => modkey,
    }
}
