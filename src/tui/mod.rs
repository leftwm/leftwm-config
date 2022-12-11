use std::time::Duration;

use anyhow::Result;
use tui_realm_stdlib::{Label, Table};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
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

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    None,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    TableConfig,
    Hints,
}

struct Model {
    alive: bool,
    dirty: bool,
    config: Config,
    app: Application<Id, Msg, NoUserEvent>,
}

impl Model {
    fn try_new() -> Result<Self> {
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );

        let config = filehandler::load();

        app.mount(Id::TableConfig, Box::new(TableConfig::new(&config)), vec![])?;
        app.mount(Id::Hints, Box::new(Hints::new()), vec![])?;
        app.active(&Id::TableConfig)?;

        Ok(Self {
            alive: true,
            dirty: true,
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
            self.app.view(&Id::TableConfig, f, chunks[1]);
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
        if model.dirty {
            model.view(&mut terminal)?;
            model.dirty = false;
        }
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
            Msg::None => None,
        }
    }
}

#[derive(MockComponent)]
struct TableConfig {
    component: Table,
}

impl TableConfig {
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
                .highlighted_str(">>")
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
                Some(Size::Ratio(w)) => format!("{} %", w * 100f32),
                None => format!("Not set"),
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

impl Component<Msg, NoUserEvent> for TableConfig {
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
            _ => CmdResult::None,
        };
        Some(Msg::None)
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
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
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
