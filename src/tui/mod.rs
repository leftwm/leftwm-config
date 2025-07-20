use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use popups::{SelectorEnum, ToggleValueEditor};
use tui_realm_stdlib::{Label, Table};
use tuirealm::command::{Cmd, CmdResult, Direction};
use tuirealm::props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan};
use tuirealm::terminal::TerminalBridge;
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
use tuirealm::{AttrValue, Attribute};

use crate::config::modifier::Modifier as KeyModifier;
use crate::config::values::{FocusBehaviour, InsertBehavior, LayoutMode, LogLevel};
use crate::config::{
    filehandler, Backend, Config, FocusOnActivationBehaviour, WindowHidingStrategy,
};
use leftwm_layouts::Layout as WMLayout;

use self::popups::Setting;

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
pub enum Popup {
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
            Id::LogLevelEditor,
            Box::new(popups::EnumSelector::<LogLevel>::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::LogLevelHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::new("Enter: Save"),
                TextSpan::new("Logging level is controlled by the log_level option. You can change it at any time and reload LeftWM for it to apply (SoftReload or HardReload commands)."),
                TextSpan::new("Possible values are the usual logging levels, from most verbose to less: trace, info, debug, error."),
            ])),
            vec![],
        )?;

        app.mount(
            Id::BackendEditor,
            Box::new(popups::EnumSelector::<Backend>::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::BackendHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::new("Enter: Save"),
                TextSpan::new("Leftwm has currently two implemented backends:"),
                TextSpan::new("  - XLib is the legacy backend, using the libX11 C library."),
                TextSpan::new("  - X11rb, based on the x11rb crate, a rust implementation of x11."),
            ])),
            vec![],
        )?;

        app.mount(
            Id::ModKeyEditor,
            Box::new(popups::ModKeyEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::ModKeyHint,
            Box::new(popups::DocBlock::new(&[
                    TextSpan::new("Enter: Save"),
                    TextSpan::new("The modkey is the most important setting. It is used by many other settings and controls how key bindings work.")
                ])
            ),
            vec![]
        )?;

        app.mount(
            Id::MouseKeyEditor,
            Box::new(popups::MouseKeyEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::MouseKeyHint,
            Box::new(popups::DocBlock::new(&[
                    TextSpan::new("Space: Toggle Key, Enter: Save Selection"),
                    TextSpan::new("The mousekey is similarly quite important. This value can be used to determine which key, when held, can assist a mouse drag in resizing or moving a floating window or making a window float or tile."),
            ])),
            vec![],
        )?;

        app.mount(
            Id::DisableTileDragEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::DisableTileDrag,
            )),
            vec![],
        )?;
        app.mount(Id::DisableTileDragHint, Box::new(popups::DocBlock::new(&[
            TextSpan::from("Enter: Save"),
            TextSpan::from("This allows you to make it so tiled windows can not be moved or resized with the mouse. However the mouse will still be able to interact with floating windows."),
        ])), vec![])?;

        app.mount(
            Id::DisableCursorRepositionOnResizeEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::DisableCursorRepositionOnResize,
            )),
            vec![],
        )?;
        app.mount(
            Id::DisableCursorRepositionOnResizeHint,
            Box::new(popups::DocBlock::new(&[TextSpan::from("Enter: Save")])),
            vec![],
        )?;

        app.mount(
            Id::DisableTagSwapEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::DisableTagSwap,
            )),
            vec![],
        )?;
        app.mount(
            Id::DisableTagSwapHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::from("Enter: Save"),
                TextSpan::from("Starting with LeftWM 0.2.7, the behaviour of SwapTags was changed such that if you are on a tag, such as tag 1, and then SwapTags to tag 1, LeftWM will go to the previous tag instead. This behaviour can be disabled with disable_current_tag_swap.")
            ])),
            vec![],
        )?;

        app.mount(
            Id::DisableWindowSnapEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::DisableWindowSnap,
            )),
            vec![],
        )?;
        app.mount(
            Id::DisableWindowSnapHint,
            Box::new(popups::DocBlock::new(&[])),
            vec![],
        )?;

        app.mount(
            Id::FocusNewWindowsEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::FocusNewWindows,
            )),
            vec![],
        )?;
        app.mount(
            Id::SloppyMouseFollowsFocusEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::SloppyMouseFollowsFocus,
            )),
            vec![],
        )?;

        app.mount(
            Id::FocusBehaviourEditor,
            Box::new(popups::EnumSelector::<FocusBehaviour>::new(&config)),
            vec![],
        )?;
        app.mount(Id::FocusBehaviourHint, Box::new(popups::DocBlock::new(&[
            TextSpan::from("Enter: Save"),
            TextSpan::from("LeftWM now has 3 focusing behaviours (Sloppy, ClickTo, and Driven) and 2 options (focus_new_windows, sloppy_mouse_follows_focus), which alter the way focus is handled. These encompass 5 different patterns:"),
            TextSpan::from("  1. Sloppy Focus. Focus follows the mouse, hovering over a window brings it to focus. This behaviour have a variant which is toggled with the sloppy_mouse_follows_focus option:"),
            TextSpan::from("    - When true, the cursor will follow the focus and teleport to the window that takes focus."),
            TextSpan::from("    - When false, the cursor isn't moved by LeftWM at all."),
            TextSpan::from("  2. Click-to-Focus. Focus follows the mouse, but only clicks change focus."),
            TextSpan::from("  3. Driven Focus. Focus disregards the mouse, only keyboard actions drive the focus."),
            TextSpan::from("  4. Event Focus. Focuses when requested by the window/new windows."),
        ])), vec![])?;

        app.mount(
            Id::FocusOnActivationEditor,
            Box::new(popups::EnumSelector::<FocusOnActivationBehaviour>::new(
                &config,
            )),
            vec![],
        )?;

        app.mount(
            Id::InsertBehaviorEditor,
            Box::new(popups::EnumSelector::<InsertBehavior>::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::InsertBehaviorHint,
            Box::new(popups::DocBlock::new(&[TextSpan::from("Enter: Save")])),
            vec![],
        )?;

        app.mount(
            Id::CreateFollowsCursorEditor,
            Box::new(popups::EnumSelector::<popups::CreateFollowsCursor>::new(
                &config,
            )),
            vec![],
        )?;
        app.mount(
            Id::CreateFollowsCursorHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::from("Enter: Save"),
                TextSpan::from("In multi-workspace layouts (such as with multiple monitors), LeftWM will, by default, create a new window on the workspace where the cursor is currently located, even if that workspace is not the workspace which is focused. In Click-to-Focus and Driven Focus modes, however, it is often desirable to create the window in the focused workspace, not the one wherein the mouse is located. The create_follows_cursor feature allows for changing this behavior. New windows will be created in the workspace:"),
                TextSpan::from("  - Containing the cursor when unset (None), Some(true), or when the cursor is in Sloppy mode "),
                TextSpan::from("  - Which is focused when set to Some(false) "),
            ])),
            vec![],
        )?;

        app.mount(
            Id::LayoutModeEditor,
            Box::new(popups::EnumSelector::<LayoutMode>::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::LayoutModeHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::from("Enter: Save"),
                TextSpan::from("Leftwm now has 2 layout modes, Workspace and Tag. These determine how layouts are remembered. When in Workspace mode, layouts will be remembered per workspace. When in Tag mode, layouts are remembered per tag.")
            ])),
            vec![],
        )?;

        // app.mount(
        //     Id::LayoutsEditor,
        //     Box::new(popups::LayoutsEditor::new(&config)),
        //     vec![],
        // )?;
        app.mount(
            Id::LayoutsHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::new("Space: Toggle Layout, Enter: Save Selection"),
                TextSpan::from("Leftwm supports an ever-growing amount layouts, which define the way that windows are tiled in the workspace."),
            ])),
            vec![]
        )?;

        app.mount(
            Id::WindowHidingStrategyEditor,
            Box::new(popups::EnumSelector::<WindowHidingStrategy>::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::WindowHidingStrategyHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::from("Enter: Save"),
                TextSpan::from("Window Hiding Strategies "),
                TextSpan::from("  - MoveOnly (Default) Move the windows out of the visible area and don't minilize them. This should allow all applications to be captured by any other applications.This could result in higher resource usage, since windows will render their content like normal even if hidden. "),
                TextSpan::from("  - Unmap The common behaviour for a window manager, but it prevents hidden windows from being captured by other applications"),
                TextSpan::from("  - MoveMinimize Move the windows out of the visible area, so it can still be captured by some applications. We still inform the window that it is in a \"minimized\"-like state, so it can probably decide to not render its content as if it was focused."),
            ])),
            vec![],
        )?;

        app.mount(
            Id::SingleWindowBorderEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::SingleWindowBorder,
            )),
            vec![],
        )?;
        app.mount(
            Id::SingleWindowBorderHint,
            Box::new(popups::DocBlock::new(&[])),
            vec![],
        )?;

        app.mount(
            Id::StatePathEditor,
            Box::new(popups::StatePathEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::StatePathHint,
            Box::new(popups::DocBlock::new(&[])),
            vec![],
        )?;

        app.mount(
            Id::AutoDeriveWorkspacesEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                Setting::AutoDeriveWorkspace,
            )),
            vec![],
        )?;
        app.mount(
            Id::AutoDeriveWorkspacesHint,
            Box::new(popups::DocBlock::new(&[])),
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
                _ => {
                    todo!()
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
                Some(Popup::LogLevel) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::LogLevelEditor, f, popup_space[1]);
                    self.app.view(&Id::LogLevelHint, f, popup_space[2]);
                }
                Some(Popup::Backend) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::BackendEditor, f, popup_space[1]);
                    self.app.view(&Id::BackendHint, f, popup_space[2]);
                }
                Some(Popup::ModKey) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::ModKeyEditor, f, popup_space[1]);
                    self.app.view(&Id::ModKeyHint, f, popup_space[2]);
                }
                Some(Popup::MouseKey) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::MouseKeyEditor, f, popup_space[1]);
                    self.app.view(&Id::MouseKeyHint, f, popup_space[2]);
                }
                Some(Popup::DisableTagSwap) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::DisableTagSwapEditor, f, popup_space[1]);
                    self.app.view(&Id::DisableTagSwapHint, f, popup_space[2]);
                }
                Some(Popup::DisableTileDrag) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::DisableTileDragEditor, f, popup_space[1]);
                    self.app.view(&Id::DisableTileDragHint, f, popup_space[2]);
                }
                Some(Popup::DisableWindowSnap) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::DisableWindowSnapEditor, f, popup_space[1]);
                    self.app.view(&Id::DisableWindowSnapHint, f, popup_space[2]);
                }
                Some(Popup::DisableCursorRepositionOnResize) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(
                        &Id::DisableCursorRepositionOnResizeEditor,
                        f,
                        popup_space[1],
                    );
                    self.app
                        .view(&Id::DisableCursorRepositionOnResizeHint, f, popup_space[2]);
                }
                Some(Popup::FocusNewWindows) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::FocusNewWindowsEditor, f, popup_space[1]);
                    self.app.view(&Id::FocusBehaviourHint, f, popup_space[2]);
                }
                Some(Popup::SloppyMouseFollowsFocus) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::SloppyMouseFollowsFocusEditor, f, popup_space[1]);
                    self.app.view(&Id::FocusBehaviourHint, f, popup_space[2]);
                }
                Some(Popup::FocusBehaviour) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::FocusBehaviourEditor, f, popup_space[1]);
                    self.app.view(&Id::FocusBehaviourHint, f, popup_space[2]);
                }
                Some(Popup::FocusOnActivationBehaviour) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::FocusOnActivationEditor, f, popup_space[1]);
                    self.app.view(&Id::FocusBehaviourHint, f, popup_space[2]);
                }
                Some(Popup::InsertBehavior) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::InsertBehaviorEditor, f, popup_space[1]);
                    self.app.view(&Id::InsertBehaviorHint, f, popup_space[2]);
                }
                Some(Popup::CreateFollowsCursor) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::CreateFollowsCursorEditor, f, popup_space[1]);
                    self.app
                        .view(&Id::CreateFollowsCursorHint, f, popup_space[2]);
                }
                Some(Popup::LayoutMode) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::LayoutModeEditor, f, popup_space[1]);
                    self.app.view(&Id::LayoutModeHint, f, popup_space[2]);
                }
                Some(Popup::Layouts) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::LayoutsEditor, f, popup_space[1]);
                    self.app.view(&Id::LayoutsHint, f, popup_space[2]);
                }
                Some(Popup::WindowHidingStrategy) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::WindowHidingStrategyEditor, f, popup_space[1]);
                    self.app
                        .view(&Id::WindowHidingStrategyHint, f, popup_space[2]);
                }
                Some(Popup::SingleWindowBorder) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::SingleWindowBorderEditor, f, popup_space[1]);
                    self.app
                        .view(&Id::SingleWindowBorderHint, f, popup_space[2]);
                }
                Some(Popup::StatePath) => {
                    let space = Layout::default()
                        .direction(LayoutDirection::Vertical)
                        .margin(1)
                        .constraints([
                            Constraint::Max(0),
                            Constraint::Length(3),
                            Constraint::Max(0),
                        ])
                        .split(popup_space[1]);
                    f.render_widget(Clear, space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::StatePathEditor, f, space[1]);
                    self.app.view(&Id::StatePathHint, f, popup_space[2]);
                }
                Some(Popup::AutoDeriveWorkspaces) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app
                        .view(&Id::AutoDeriveWorkspacesEditor, f, popup_space[1]);
                    self.app
                        .view(&Id::AutoDeriveWorkspacesHint, f, popup_space[2]);
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
                    Some(Popup::LogLevel) => self.app.active(&Id::LogLevelEditor),
                    Some(Popup::Backend) => self.app.active(&Id::BackendEditor),
                    Some(Popup::ModKey) => self.app.active(&Id::ModKeyEditor),
                    Some(Popup::MouseKey) => self.app.active(&Id::MouseKeyEditor),
                    Some(Popup::DisableTagSwap) => self.app.active(&Id::DisableTagSwapEditor),
                    Some(Popup::DisableTileDrag) => self.app.active(&Id::DisableTileDragEditor),
                    Some(Popup::DisableWindowSnap) => self.app.active(&Id::DisableWindowSnapEditor),
                    Some(Popup::DisableCursorRepositionOnResize) => {
                        self.app.active(&Id::DisableCursorRepositionOnResizeEditor)
                    }
                    Some(Popup::FocusNewWindows) => self.app.active(&Id::FocusNewWindowsEditor),
                    Some(Popup::SloppyMouseFollowsFocus) => {
                        self.app.active(&Id::SloppyMouseFollowsFocusEditor)
                    }
                    Some(Popup::FocusBehaviour) => self.app.active(&Id::FocusBehaviourEditor),
                    Some(Popup::FocusOnActivationBehaviour) => {
                        self.app.active(&Id::FocusOnActivationEditor)
                    }
                    Some(Popup::InsertBehavior) => self.app.active(&Id::InsertBehaviorEditor),
                    Some(Popup::CreateFollowsCursor) => {
                        self.app.active(&Id::CreateFollowsCursorEditor)
                    }
                    Some(Popup::LayoutMode) => self.app.active(&Id::LayoutModeEditor),
                    Some(Popup::Layouts) => self.app.active(&Id::LayoutsEditor),
                    Some(Popup::WindowHidingStrategy) => {
                        self.app.active(&Id::WindowHidingStrategyEditor)
                    }
                    Some(Popup::SingleWindowBorder) => {
                        self.app.active(&Id::SingleWindowBorderEditor)
                    }
                    Some(Popup::StatePath) => self.app.active(&Id::StatePathEditor),
                    Some(Popup::AutoDeriveWorkspaces) => {
                        self.app.active(&Id::AutoDeriveWorkspacesEditor)
                    }
                    None => self.app.active(&Id::HomeView),
                }
                .unwrap();
                self.popup = p;
                None
            }
            Msg::UpdateConfig(config_update, close_popup) => {
                match config_update {
                    ConfigUpdate::LogLevel(log_level) => {
                        self.config.log_level = log_level.to_string();
                        self.app
                            .remount(
                                Id::LogLevelEditor,
                                Box::new(popups::EnumSelector::<LogLevel>::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::Backend(backend) => {
                        self.config.backend = backend;
                        self.app
                            .remount(
                                Id::BackendEditor,
                                Box::new(popups::EnumSelector::<Backend>::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
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
                    ConfigUpdate::DisableTagSwap(swap) => {
                        self.config.disable_current_tag_swap = swap;
                        self.app
                            .remount(
                                Id::DisableTagSwapEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::DisableTagSwap,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::DisableTileDrag(drag) => {
                        self.config.disable_tile_drag = drag;
                        self.app
                            .remount(
                                Id::DisableTileDragEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::DisableTileDrag,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::DisableWindowSnap(snap) => {
                        self.config.disable_window_snap = snap;
                        self.app
                            .remount(
                                Id::DisableWindowSnapEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::DisableWindowSnap,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::FocusNewWindows(focus) => {
                        self.config.focus_new_windows = focus;
                        self.app
                            .remount(
                                Id::FocusNewWindowsEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::FocusNewWindows,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::SloppyMouseFollowsFocus(follow) => {
                        self.config.sloppy_mouse_follows_focus = follow;
                        self.app
                            .remount(
                                Id::SloppyMouseFollowsFocusEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::SloppyMouseFollowsFocus,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::FocusBehaviour(behavior) => {
                        self.config.focus_behaviour = behavior;
                        self.app
                            .remount(
                                Id::FocusBehaviourEditor,
                                Box::new(popups::EnumSelector::<FocusBehaviour>::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::InsertBehavior(behavior) => {
                        self.config.insert_behavior = behavior;
                        self.app
                            .remount(
                                Id::InsertBehaviorEditor,
                                Box::new(popups::EnumSelector::<InsertBehavior>::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::LayoutMode(mode) => {
                        self.config.layout_mode = mode;
                        self.app
                            .remount(
                                Id::LayoutModeEditor,
                                Box::new(popups::EnumSelector::<LayoutMode>::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::Layouts(_layouts) => {
                        // self.config.layouts = layouts;
                        // self.app
                        //     .remount(
                        //         Id::LayoutsEditor,
                        //         Box::new(popups::LayoutsEditor::new(&self.config)),
                        //         vec![],
                        //     )
                        //     .unwrap();
                        todo!()
                    }
                    ConfigUpdate::StatePath(path) => {
                        self.config.state_path = path;
                        self.app
                            .remount(
                                Id::StatePathEditor,
                                Box::new(popups::StatePathEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::AutoDeriveWorkspaces(val) => {
                        self.config.auto_derive_workspaces = val;
                        self.app
                            .remount(
                                Id::AutoDeriveWorkspacesEditor,
                                Box::new(popups::ToggleValueEditor::new(
                                    &self.config,
                                    Setting::AutoDeriveWorkspace,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::DisableCursorRepositionOnResize(val) => {
                        self.config.disable_cursor_reposition_on_resize = val;
                        self.app
                            .remount(
                                Id::DisableCursorRepositionOnResizeEditor,
                                Box::new(ToggleValueEditor::new(
                                    &self.config,
                                    Setting::DisableCursorRepositionOnResize,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::FocusOnActivation(focus_on_activation_behaviour) => {
                        self.config.focus_on_activation = focus_on_activation_behaviour;
                        self.app
                            .remount(
                                Id::FocusOnActivationEditor,
                                Box::new(popups::EnumSelector::<FocusOnActivationBehaviour>::new(
                                    &self.config,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::CreateFollowsCursor(val) => {
                        self.config.create_follows_cursor = val;
                        self.app
                            .remount(
                                Id::CreateFollowsCursorEditor,
                                Box::new(popups::EnumSelector::<popups::CreateFollowsCursor>::new(
                                    &self.config,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::WindowHidingStrategy(window_hiding_strategy) => {
                        self.config.window_hiding_strategy = window_hiding_strategy;
                        self.app
                            .remount(
                                Id::WindowHidingStrategyEditor,
                                Box::new(popups::EnumSelector::<WindowHidingStrategy>::new(
                                    &self.config,
                                )),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::SingleWindowBorder(val) => {
                        self.config.single_window_border = val;
                        self.app
                            .remount(
                                Id::SingleWindowBorderEditor,
                                Box::new(ToggleValueEditor::new(
                                    &self.config,
                                    Setting::SingleWindowBorder,
                                )),
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
    popups: Vec<Option<Popup>>,
}

impl HomeView {
    fn new(config: &Config) -> Self {
        let (table, popups) = Self::build_inner(config);
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
                .column_spacing(3)
                .widths(&[50, 50])
                .table(table),
            popups,
        }
    }

    fn build_inner(config: &Config) -> (Vec<Vec<TextSpan>>, Vec<Option<Popup>>) {
        let mut popups = Vec::new();

        let mut table = TableBuilder::default();

        let mut config_option = |name: &str, val: &str, popup: Option<Popup>| {
            table
                .add_col(TextSpan::new(name))
                .add_col(TextSpan::new(val))
                .add_row();
            popups.push(popup);
        };

        config_option("Log level", &config.log_level, Some(Popup::LogLevel));
        config_option(
            "Backend",
            config.backend.variant_name(),
            Some(Popup::Backend),
        );

        config_option(
            "Mousekey",
            &format_modkey_name(
                config
                    .mousekey
                    .clone()
                    .unwrap_or_else(|| KeyModifier::Single("None".to_string()))
                    .to_string(),
            ),
            Some(Popup::MouseKey),
        );
        config_option(
            "Disable Tile Drag",
            &format!("{}", config.disable_tile_drag),
            Some(Popup::DisableTileDrag),
        );
        config_option(
            "Disable Window Snap",
            &format!("{}", config.disable_window_snap),
            Some(Popup::DisableWindowSnap),
        );
        config_option(
            "Disable Current Tag Swap",
            &format!("{}", config.disable_current_tag_swap),
            Some(Popup::DisableTagSwap),
        );
        config_option(
            "Disable Cursor Reposition On Resize",
            &format!("{}", config.disable_cursor_reposition_on_resize),
            Some(Popup::DisableCursorRepositionOnResize),
        );

        config_option(
            "Focus Behavior",
            config.focus_behaviour.variant_name(),
            Some(Popup::FocusBehaviour),
        );
        config_option(
            "Focus New Windows",
            &format!("{}", config.focus_new_windows),
            Some(Popup::FocusNewWindows),
        );
        config_option(
            "Sloppy Mouse Follows Focus",
            &format!("{}", config.sloppy_mouse_follows_focus),
            Some(Popup::SloppyMouseFollowsFocus),
        );
        config_option(
            "Focus On Activation Behaviour",
            config.focus_on_activation.variant_name(),
            Some(Popup::FocusOnActivationBehaviour),
        );

        config_option(
            "Insert Behavior",
            config.insert_behavior.variant_name(),
            Some(Popup::InsertBehavior),
        );
        config_option(
            "Create Follows Cursor",
            &config
                .create_follows_cursor
                .map_or("unset".to_string(), |b| format!("{b}")),
            Some(Popup::CreateFollowsCursor),
        );

        config_option(
            "Layout Mode",
            config.layout_mode.variant_name(),
            Some(Popup::LayoutMode),
        );
        config_option("Layouts", &format!("{} set", config.layouts.len()), None);
        config_option(
            "Layout Definitions",
            &format!("{} set", config.layout_definitions.len()),
            None,
        );

        config_option(
            "Auto Derive Workspaces",
            &format!("{}", config.auto_derive_workspaces),
            Some(Popup::AutoDeriveWorkspaces),
        );
        if !config.auto_derive_workspaces {
            config_option(
                "Workspaces",
                &format!(
                    "{} set",
                    config.workspaces.as_ref().unwrap_or(&vec![]).len()
                ),
                None,
            );
        }

        config_option(
            "Scratchpads",
            &format!(
                "{} set",
                config.scratchpad.as_ref().unwrap_or(&vec![]).len()
            ),
            None,
        );

        config_option(
            "Tags",
            &format!("{} set", config.tags.as_ref().unwrap_or(&vec![]).len()),
            None,
        );
        config_option(
            "Window Hiding Strategy",
            config.window_hiding_strategy.variant_name(),
            Some(Popup::WindowHidingStrategy),
        );

        config_option(
            "Window Rules",
            &format!(
                "{} set",
                config.window_rules.as_ref().unwrap_or(&vec![]).len()
            ),
            None,
        );
        config_option(
            "Single Window Border",
            &format!("{}", config.single_window_border),
            Some(Popup::SingleWindowBorder),
        );

        config_option(
            "Modkey",
            &format_modkey_name(config.modkey.clone()),
            Some(Popup::ModKey),
        );
        config_option("Keybinds", &format!("{} set", config.keybind.len()), None);

        table
            .add_col(TextSpan::new("State Path"))
            .add_col(TextSpan::new(&match &config.state_path {
                Some(p) => format!("{}", p.display()),
                None => "Not set".to_string(),
            }));
        popups.push(Some(Popup::StatePath));
        (table.build(), popups)
    }

    fn update(&mut self, config: &Config) {
        self.component.attr(
            Attribute::Content,
            AttrValue::Table(Self::build_inner(config).0),
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
            }) => match &self.popups[self.component.states.list_index] {
                Some(p) => return Some(Msg::SetPopup(Some(*p))),
                None => CmdResult::None,
            },
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
