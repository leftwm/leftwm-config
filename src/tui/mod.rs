use std::path::PathBuf;

use anyhow::Result;
use home_view::HomeView;
use model::{Model, UserEvent};
use tui_realm_stdlib::Label;
use tuirealm::props::{Alignment, Color};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{application::PollStrategy, Component, Event, MockComponent, Update};

use crate::config::modifier::Modifier as KeyModifier;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode, LogLevel};
use crate::config::{Backend, FocusOnActivationBehaviour, WindowHidingStrategy};
use leftwm_layouts::Layout as WMLayout;

mod home_view;
mod model;
mod popups;

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    SetHomePopup(Option<HomePopup>),
    // Config, close_popup
    UpdateConfig(ConfigUpdate, bool),
    None,
}

#[derive(Debug, Clone)]
pub enum ConfigUpdate {
    LogLevel(LogLevel),
    Backend(Backend),

    MouseKey(Option<KeyModifier>),
    DisableTileDrag(bool),
    DisableWindowSnap(bool),
    DisableTagSwap(bool),
    DisableCursorRepositionOnResize(bool),

    FocusBehaviour(FocusBehaviour),
    FocusNewWindows(bool),
    SloppyMouseFollowsFocus(bool),
    FocusOnActivation(FocusOnActivationBehaviour),

    InsertBehavior(InsertBehavior),
    CreateFollowsCursor(Option<bool>),

    Layouts(Vec<WMLayout>),
    LayoutMode(LayoutMode),
    // LayoutDefinitions

    // Workspaces
    AutoDeriveWorkspaces(bool),

    // ScratchPads

    // Tags
    WindowHidingStrategy(WindowHidingStrategy),

    // WindowRules
    SingleWindowBorder(bool),

    ModKey(String),
    // Keybinds
    StatePath(Option<PathBuf>),
}

// Two [`ConfigUpdate`]s are equal if they update the same thing
impl PartialEq for ConfigUpdate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ConfigUpdate::LogLevel(_), ConfigUpdate::LogLevel(_)) => true,
            (ConfigUpdate::Backend(_), ConfigUpdate::Backend(_)) => true,
            (ConfigUpdate::MouseKey(_), ConfigUpdate::MouseKey(_)) => true,
            (ConfigUpdate::DisableTileDrag(_), ConfigUpdate::DisableTileDrag(_)) => true,
            (ConfigUpdate::DisableWindowSnap(_), ConfigUpdate::DisableWindowSnap(_)) => true,
            (ConfigUpdate::DisableTagSwap(_), ConfigUpdate::DisableTagSwap(_)) => true,
            (
                ConfigUpdate::DisableCursorRepositionOnResize(_),
                ConfigUpdate::DisableCursorRepositionOnResize(_),
            ) => true,
            (ConfigUpdate::FocusBehaviour(_), ConfigUpdate::FocusBehaviour(_)) => true,
            (ConfigUpdate::FocusNewWindows(_), ConfigUpdate::FocusNewWindows(_)) => true,
            (
                ConfigUpdate::SloppyMouseFollowsFocus(_),
                ConfigUpdate::SloppyMouseFollowsFocus(_),
            ) => true,
            (ConfigUpdate::FocusOnActivation(_), ConfigUpdate::FocusOnActivation(_)) => true,
            (ConfigUpdate::InsertBehavior(_), ConfigUpdate::InsertBehavior(_)) => true,
            (ConfigUpdate::CreateFollowsCursor(_), ConfigUpdate::CreateFollowsCursor(_)) => true,
            (ConfigUpdate::Layouts(_), ConfigUpdate::Layouts(_)) => true,
            (ConfigUpdate::LayoutMode(_), ConfigUpdate::LayoutMode(_)) => true,
            (ConfigUpdate::AutoDeriveWorkspaces(_), ConfigUpdate::AutoDeriveWorkspaces(_)) => true,
            (ConfigUpdate::WindowHidingStrategy(_), ConfigUpdate::WindowHidingStrategy(_)) => true,
            (ConfigUpdate::SingleWindowBorder(_), ConfigUpdate::SingleWindowBorder(_)) => true,
            (ConfigUpdate::ModKey(_), ConfigUpdate::ModKey(_)) => true,
            (ConfigUpdate::StatePath(_), ConfigUpdate::StatePath(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    HomeView,
    Hints,

    LogLevelEditor,
    LogLevelHint,

    BackendEditor,
    BackendHint,

    MouseKeyEditor,
    MouseKeyHint,

    DisableTileDragEditor,
    DisableTileDragHint,

    DisableWindowSnapEditor,
    DisableWindowSnapHint,

    DisableTagSwapEditor,
    DisableTagSwapHint,

    DisableCursorRepositionOnResizeEditor,
    DisableCursorRepositionOnResizeHint,

    FocusNewWindowsEditor,
    SloppyMouseFollowsFocusEditor,
    FocusOnActivationEditor,

    FocusBehaviourEditor,
    FocusBehaviourHint,

    InsertBehaviorEditor,
    InsertBehaviorHint,

    CreateFollowsCursorEditor,
    CreateFollowsCursorHint,

    LayoutModeEditor,
    LayoutModeHint,

    LayoutsEditor,
    LayoutsHint,

    WindowHidingStrategyEditor,
    WindowHidingStrategyHint,

    SingleWindowBorderEditor,
    SingleWindowBorderHint,

    StatePathEditor,
    StatePathHint,

    AutoDeriveWorkspacesEditor,
    AutoDeriveWorkspacesHint,

    ModKeyEditor,
    ModKeyHint,
}

pub enum View {
    Home,
    LayoutDefinitions,
    Workspaces,
    Scratchpads,
    Tags,
    WindowRules,
    Keybinds,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HomePopup {
    LogLevel,
    Backend,
    ModKey,
    MouseKey,
    DisableTagSwap,
    DisableTileDrag,
    DisableWindowSnap,
    DisableCursorRepositionOnResize,
    FocusNewWindows,
    SloppyMouseFollowsFocus,
    FocusBehaviour,
    FocusOnActivationBehaviour,
    CreateFollowsCursor,
    InsertBehavior,
    LayoutMode,
    Layouts,
    WindowHidingStrategy,
    SingleWindowBorder,
    StatePath,
    AutoDeriveWorkspaces,
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

impl Component<Msg, UserEvent> for Hints {
    fn on(&mut self, _ev: Event<UserEvent>) -> Option<Msg> {
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
