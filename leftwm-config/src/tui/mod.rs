use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
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

use leftwm_config_core::modifier::Modifier as KeyModifier;
use leftwm_config_core::Layout as WMLayout;
use leftwm_config_core::LayoutMode;
use leftwm_config_core::{filehandler, Config, FocusBehaviour, InsertBehavior, Size};

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
    ModKey(String),
    MouseKey(Option<KeyModifier>),
    MaxWindowWidth(Option<Size>),
    DisableTagSwap(bool),
    DisableTileDrag(bool),
    DisableWindowSnap(bool),
    FocusNewWindows(bool),
    SloppyMouseFollowsFocus(bool),
    FocusBehaviour(FocusBehaviour),
    InsertBehavior(InsertBehavior),
    LayoutMode(LayoutMode),
    Layouts(Vec<WMLayout>),
    StatePath(Option<PathBuf>),
    AutoDeriveWorkspaces(bool),
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

    DisableTagSwapEditor,
    DisableTagSwapHint,

    DisableTileDragEditor,
    DisableTileDragHint,

    DisableWindowSnapEditor,
    DisableWindowSnapHint,

    FocusNewWindowsEditor,
    SloppyMouseFollowsFocusEditor,

    FocusBehaviourEditor,
    FocusBehaviourHint,

    InsertBehaviorEditor,
    InsertBehaviorHint,

    LayoutModeEditor,
    LayoutModeHint,

    LayoutsEditor,
    LayoutsHint,

    StatePathEditor,
    StatePathHint,

    AutoDeriveWorkspacesEditor,
    AutoDeriveWorkspacesHint,
}

pub enum View {
    Home,
}

#[derive(Debug, PartialEq)]
pub enum Popup {
    ModKey,
    MouseKey,
    MaxWindowWidth,
    DisableTagSwap,
    DisableTileDrag,
    DisableWindowSnap,
    FocusNewWindows,
    SloppyMouseFollowsFocus,
    FocusBehaviour,
    InsertBehavior,
    LayoutMode,
    Layouts,
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
            Id::MaxWindowWidthEditor,
            Box::new(popups::MaxWindowWidthEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::MaxWindowWidthHint,
            Box::new(popups::DocBlock::new(&[
                    TextSpan::new("Enter: Save"),
                    TextSpan::new("A red border indicates and invalid input. An empty value unsets the max window width."),
                    TextSpan::new("LeftWM-Config will try to parse the entered value as either a fraction between 0 and 1, a percentage (if ending in a pecent sign) or as an absolute value."),
                    TextSpan::new("You can configure a max_window_width to limit the width of the tiled windows (or rather, the width of columns in a layout). This feature comes in handy when working on ultra-wide monitors where you don't want a single window to take the complete workspace width.")
            ])),
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
            Box::new(popups::FocusBehaviorEditor::new(&config)),
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
            Id::InsertBehaviorEditor,
            Box::new(popups::InsertBehaviorEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::InsertBehaviorHint,
            Box::new(popups::DocBlock::new(&[TextSpan::from("Enter: Save")])),
            vec![],
        )?;

        app.mount(
            Id::LayoutModeEditor,
            Box::new(popups::LayoutModeEditor::new(&config)),
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

        app.mount(
            Id::LayoutsEditor,
            Box::new(popups::LayoutsEditor::new(&config)),
            vec![],
        )?;
        app.mount(
            Id::LayoutsHint,
            Box::new(popups::DocBlock::new(&[
                TextSpan::new("Space: Toggle Layout, Enter: Save Selection"),
                TextSpan::from("Leftwm supports an ever-growing amount layouts, which define the way that windows are tiled in the workspace."),
            ])),
            vec![]
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
                    f.render_widget(Clear, space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::MaxWindowWidthEditor, f, space[1]);
                    self.app.view(&Id::MaxWindowWidthHint, f, popup_space[2]);
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
                Some(Popup::InsertBehavior) => {
                    f.render_widget(Clear, popup_space[1]);
                    f.render_widget(Clear, popup_space[2]);
                    self.app.view(&Id::InsertBehaviorEditor, f, popup_space[1]);
                    self.app.view(&Id::InsertBehaviorHint, f, popup_space[2]);
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
                    Some(Popup::ModKey) => self.app.active(&Id::ModKeyEditor),
                    Some(Popup::MouseKey) => self.app.active(&Id::MouseKeyEditor),
                    Some(Popup::MaxWindowWidth) => self.app.active(&Id::MaxWindowWidthEditor),
                    Some(Popup::DisableTagSwap) => self.app.active(&Id::DisableTagSwapEditor),
                    Some(Popup::DisableTileDrag) => self.app.active(&Id::DisableTileDragEditor),
                    Some(Popup::FocusNewWindows) => self.app.active(&Id::FocusNewWindowsEditor),
                    Some(Popup::DisableWindowSnap) => self.app.active(&Id::DisableWindowSnapEditor),
                    Some(Popup::SloppyMouseFollowsFocus) => {
                        self.app.active(&Id::SloppyMouseFollowsFocusEditor)
                    }
                    Some(Popup::FocusBehaviour) => self.app.active(&Id::FocusBehaviourEditor),
                    Some(Popup::InsertBehavior) => self.app.active(&Id::InsertBehaviorEditor),
                    Some(Popup::LayoutMode) => self.app.active(&Id::LayoutModeEditor),
                    Some(Popup::Layouts) => self.app.active(&Id::LayoutsEditor),
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
                                Box::new(popups::FocusBehaviorEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::InsertBehavior(behavior) => {
                        self.config.insert_behavior = behavior;
                        self.app
                            .remount(
                                Id::InsertBehaviorEditor,
                                Box::new(popups::InsertBehaviorEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::LayoutMode(mode) => {
                        self.config.layout_mode = mode;
                        self.app
                            .remount(
                                Id::LayoutModeEditor,
                                Box::new(popups::LayoutModeEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
                    }
                    ConfigUpdate::Layouts(layouts) => {
                        self.config.layouts = layouts;
                        self.app
                            .remount(
                                Id::LayoutsEditor,
                                Box::new(popups::LayoutsEditor::new(&self.config)),
                                vec![],
                            )
                            .unwrap();
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
            .add_col(TextSpan::from("Disable Window Snap"))
            .add_col(TextSpan::from(format!("{}", config.disable_window_snap)))
            .add_row()
            .add_col(TextSpan::from("Focus New Windows"))
            .add_col(TextSpan::from(format!("{}", config.focus_new_windows)))
            .add_row()
            .add_col(TextSpan::from("Sloppy Mouse Follows Focus"))
            .add_col(TextSpan::from(format!(
                "{}",
                config.sloppy_mouse_follows_focus
            )))
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
            .add_col(TextSpan::from("State Path"))
            .add_col(TextSpan::from(match &config.state_path {
                Some(p) => format!("{}", p.display()),
                None => "Not set".to_string(),
            }))
            .add_row()
            .add_col(TextSpan::from("Auto Derive Workspaces"))
            .add_col(TextSpan::from(format!("{}", config.auto_derive_workspaces)))
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
                    3 => return Some(Msg::SetPopup(Some(Popup::DisableTagSwap))),
                    4 => return Some(Msg::SetPopup(Some(Popup::DisableTileDrag))),
                    5 => return Some(Msg::SetPopup(Some(Popup::DisableWindowSnap))),
                    6 => return Some(Msg::SetPopup(Some(Popup::FocusNewWindows))),
                    7 => return Some(Msg::SetPopup(Some(Popup::SloppyMouseFollowsFocus))),
                    8 => return Some(Msg::SetPopup(Some(Popup::FocusBehaviour))),
                    9 => return Some(Msg::SetPopup(Some(Popup::InsertBehavior))),
                    10 => return Some(Msg::SetPopup(Some(Popup::LayoutMode))),
                    11 => return Some(Msg::SetPopup(Some(Popup::Layouts))),
                    12 => return Some(Msg::SetPopup(Some(Popup::StatePath))),
                    13 => return Some(Msg::SetPopup(Some(Popup::AutoDeriveWorkspaces))),
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
