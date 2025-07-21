use std::{
    cmp::Ordering,
    io::Stdout,
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    time::Duration,
};

use tuirealm::{
    listener::{ListenerResult, Poll},
    props::TextSpan,
    terminal::TerminalBridge,
    tui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
        widgets::Clear,
        Frame,
    },
    Application, Event, EventListenerCfg, ListenerError, Sub, SubClause, SubEventClause, Update,
};

use anyhow::Result;

use crate::config::{
    filehandler,
    values::{FocusBehaviour, InsertBehavior, LayoutMode, LogLevel},
    Backend, Config, FocusOnActivationBehaviour, WindowHidingStrategy,
};

use super::{
    popups::{self},
    ConfigUpdate, Hints, HomePopup, HomeView, Id, Msg, View,
};

pub struct Model {
    pub alive: bool,
    pub dirty: bool,
    pub view: View,
    pub popup: Option<HomePopup>,
    pub config: Config,
    pub app: Application<Id, Msg, UserEvent>,
    user_event_sender: Sender<UserEvent>,
}

#[derive(Clone, Debug)]
pub enum UserEvent {
    ConfigUpdate(ConfigUpdate),
}

impl PartialOrd for UserEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if matches!(
            (self, other),
            (UserEvent::ConfigUpdate(_), UserEvent::ConfigUpdate(_))
        ) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl PartialEq for UserEvent {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (UserEvent::ConfigUpdate(_), UserEvent::ConfigUpdate(_))
        )
    }
}

impl Eq for UserEvent {}

pub struct UserEventPort(Receiver<UserEvent>);

impl Poll<UserEvent> for UserEventPort {
    fn poll(&mut self) -> ListenerResult<Option<Event<UserEvent>>> {
        match self.0.try_recv() {
            Ok(v) => Ok(Some(Event::User(v))),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(ListenerError::ListenerDied),
        }
    }
}

impl Model {
    pub fn try_new() -> Result<Self> {
        let (tx, rx) = channel();
        let event_listener = EventListenerCfg::default()
            .port(Box::new(UserEventPort(rx)), Duration::from_millis(10))
            .default_input_listener(Duration::from_millis(10));

        let mut app: Application<Id, Msg, UserEvent> = Application::init(event_listener);

        let config = filehandler::load();

        app.mount(Id::HomeView, Box::new(HomeView::new(&config)), vec![])?;
        app.mount(Id::Hints, Box::new(Hints::new()), vec![])?;

        app.mount(
            Id::LogLevelEditor,
            Box::new(popups::EnumSelector::<LogLevel>::new(&config)),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::LogLevel(
                    LogLevel::Off,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::Backend(
                    Backend::XLib,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::ModKey(
                    "Super".to_string(),
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::MouseKey(None))),
                SubClause::Always,
            )],
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
                popups::Setting::DisableTileDrag,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::DisableTileDrag(
                    false,
                ))),
                SubClause::Always,
            )],
        )?;
        app.mount(Id::DisableTileDragHint, Box::new(popups::DocBlock::new(&[
            TextSpan::from("Enter: Save"),
            TextSpan::from("This allows you to make it so tiled windows can not be moved or resized with the mouse. However the mouse will still be able to interact with floating windows."),
        ])), vec![])?;

        app.mount(
            Id::DisableCursorRepositionOnResizeEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                popups::Setting::DisableCursorRepositionOnResize,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(
                    ConfigUpdate::DisableCursorRepositionOnResize(false),
                )),
                SubClause::Always,
            )],
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
                popups::Setting::DisableTagSwap,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::DisableTagSwap(false))),
                SubClause::Always,
            )],
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
                popups::Setting::DisableWindowSnap,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::DisableWindowSnap(
                    false,
                ))),
                SubClause::Always,
            )],
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
                popups::Setting::FocusNewWindows,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::FocusNewWindows(
                    false,
                ))),
                SubClause::Always,
            )],
        )?;
        app.mount(
            Id::SloppyMouseFollowsFocusEditor,
            Box::new(popups::ToggleValueEditor::new(
                &config,
                popups::Setting::SloppyMouseFollowsFocus,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(
                    ConfigUpdate::SloppyMouseFollowsFocus(false),
                )),
                SubClause::Always,
            )],
        )?;

        app.mount(
            Id::FocusBehaviourEditor,
            Box::new(popups::EnumSelector::<FocusBehaviour>::new(&config)),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::FocusBehaviour(
                    FocusBehaviour::Sloppy,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::FocusOnActivation(
                    FocusOnActivationBehaviour::SwitchTo,
                ))),
                SubClause::Always,
            )],
        )?;

        app.mount(
            Id::InsertBehaviorEditor,
            Box::new(popups::EnumSelector::<InsertBehavior>::new(&config)),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::InsertBehavior(
                    InsertBehavior::Top,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::CreateFollowsCursor(
                    None,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::LayoutMode(
                    LayoutMode::Tag,
                ))),
                SubClause::Always,
            )],
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
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::WindowHidingStrategy(
                    WindowHidingStrategy::Unmap,
                ))),
                SubClause::Always,
            )],
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
                popups::Setting::SingleWindowBorder,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::SingleWindowBorder(
                    false,
                ))),
                SubClause::Always,
            )],
        )?;
        app.mount(
            Id::SingleWindowBorderHint,
            Box::new(popups::DocBlock::new(&[])),
            vec![],
        )?;

        app.mount(
            Id::StatePathEditor,
            Box::new(popups::StatePathEditor::new(&config)),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::StatePath(None))),
                SubClause::Always,
            )],
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
                popups::Setting::AutoDeriveWorkspace,
            )),
            vec![Sub::new(
                // The contents of the ConfigUpdate variant are irrelevant
                SubEventClause::User(UserEvent::ConfigUpdate(ConfigUpdate::AutoDeriveWorkspaces(
                    false,
                ))),
                SubClause::Always,
            )],
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
            user_event_sender: tx,
        })
    }

    pub fn view(&mut self, terminal: &mut TerminalBridge) -> Result<()> {
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
                View::Home => self.render_homeview(f, &chunks),
                _ => {
                    todo!()
                }
            }

            self.app.view(&Id::Hints, f, chunks[2]);
        })?;
        Ok(())
    }

    fn render_homeview(&mut self, f: &mut Frame<'_, CrosstermBackend<Stdout>>, chunks: &[Rect]) {
        self.app.view(&Id::HomeView, f, chunks[1]);

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

        let mut render_home_popup = |editor: Id, hint: Id| {
            f.render_widget(Clear, popup_space[1]);
            f.render_widget(Clear, popup_space[2]);
            self.app.view(&editor, f, popup_space[1]);
            self.app.view(&hint, f, popup_space[2]);
        };

        match self.popup {
            Some(HomePopup::LogLevel) => {
                render_home_popup(Id::LogLevelEditor, Id::LogLevelHint);
            }
            Some(HomePopup::Backend) => {
                render_home_popup(Id::BackendEditor, Id::BackendHint);
            }
            Some(HomePopup::ModKey) => {
                render_home_popup(Id::ModKeyEditor, Id::ModKeyHint);
            }
            Some(HomePopup::MouseKey) => {
                render_home_popup(Id::MouseKeyEditor, Id::MouseKeyHint);
            }
            Some(HomePopup::DisableTagSwap) => {
                render_home_popup(Id::DisableTagSwapEditor, Id::DisableTagSwapHint);
            }
            Some(HomePopup::DisableTileDrag) => {
                render_home_popup(Id::DisableTileDragEditor, Id::DisableTileDragHint);
            }
            Some(HomePopup::DisableWindowSnap) => {
                render_home_popup(Id::DisableWindowSnapEditor, Id::DisableWindowSnapHint);
            }
            Some(HomePopup::DisableCursorRepositionOnResize) => {
                render_home_popup(
                    Id::DisableCursorRepositionOnResizeEditor,
                    Id::DisableCursorRepositionOnResizeHint,
                );
            }
            Some(HomePopup::FocusNewWindows) => {
                render_home_popup(Id::FocusNewWindowsEditor, Id::FocusBehaviourHint);
            }
            Some(HomePopup::SloppyMouseFollowsFocus) => {
                render_home_popup(Id::SloppyMouseFollowsFocusEditor, Id::FocusBehaviourHint);
            }
            Some(HomePopup::FocusBehaviour) => {
                render_home_popup(Id::FocusBehaviourEditor, Id::FocusBehaviourHint);
            }
            Some(HomePopup::FocusOnActivationBehaviour) => {
                render_home_popup(Id::FocusOnActivationEditor, Id::FocusBehaviourHint);
            }
            Some(HomePopup::InsertBehavior) => {
                render_home_popup(Id::InsertBehaviorEditor, Id::InsertBehaviorHint);
            }
            Some(HomePopup::CreateFollowsCursor) => {
                render_home_popup(Id::CreateFollowsCursorEditor, Id::CreateFollowsCursorHint);
            }
            Some(HomePopup::LayoutMode) => {
                render_home_popup(Id::LayoutModeEditor, Id::LayoutModeHint);
            }
            Some(HomePopup::Layouts) => {
                render_home_popup(Id::LayoutsEditor, Id::LayoutsHint);
            }
            Some(HomePopup::WindowHidingStrategy) => {
                render_home_popup(Id::WindowHidingStrategyEditor, Id::WindowHidingStrategyHint);
            }
            Some(HomePopup::SingleWindowBorder) => {
                render_home_popup(Id::SingleWindowBorderEditor, Id::SingleWindowBorderHint);
            }
            Some(HomePopup::StatePath) => {
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
            Some(HomePopup::AutoDeriveWorkspaces) => {
                render_home_popup(Id::AutoDeriveWorkspacesEditor, Id::AutoDeriveWorkspacesHint);
            }
            None => {}
        }
    }
}

impl Update<Msg> for Model {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.dirty = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.alive = false;
                None
            }
            Msg::SetHomePopup(p) => {
                match p {
                    Some(HomePopup::LogLevel) => self.app.active(&Id::LogLevelEditor),
                    Some(HomePopup::Backend) => self.app.active(&Id::BackendEditor),
                    Some(HomePopup::ModKey) => self.app.active(&Id::ModKeyEditor),
                    Some(HomePopup::MouseKey) => self.app.active(&Id::MouseKeyEditor),
                    Some(HomePopup::DisableTagSwap) => self.app.active(&Id::DisableTagSwapEditor),
                    Some(HomePopup::DisableTileDrag) => self.app.active(&Id::DisableTileDragEditor),
                    Some(HomePopup::DisableWindowSnap) => {
                        self.app.active(&Id::DisableWindowSnapEditor)
                    }
                    Some(HomePopup::DisableCursorRepositionOnResize) => {
                        self.app.active(&Id::DisableCursorRepositionOnResizeEditor)
                    }
                    Some(HomePopup::FocusNewWindows) => self.app.active(&Id::FocusNewWindowsEditor),
                    Some(HomePopup::SloppyMouseFollowsFocus) => {
                        self.app.active(&Id::SloppyMouseFollowsFocusEditor)
                    }
                    Some(HomePopup::FocusBehaviour) => self.app.active(&Id::FocusBehaviourEditor),
                    Some(HomePopup::FocusOnActivationBehaviour) => {
                        self.app.active(&Id::FocusOnActivationEditor)
                    }
                    Some(HomePopup::InsertBehavior) => self.app.active(&Id::InsertBehaviorEditor),
                    Some(HomePopup::CreateFollowsCursor) => {
                        self.app.active(&Id::CreateFollowsCursorEditor)
                    }
                    Some(HomePopup::LayoutMode) => self.app.active(&Id::LayoutModeEditor),
                    Some(HomePopup::Layouts) => self.app.active(&Id::LayoutsEditor),
                    Some(HomePopup::WindowHidingStrategy) => {
                        self.app.active(&Id::WindowHidingStrategyEditor)
                    }
                    Some(HomePopup::SingleWindowBorder) => {
                        self.app.active(&Id::SingleWindowBorderEditor)
                    }
                    Some(HomePopup::StatePath) => self.app.active(&Id::StatePathEditor),
                    Some(HomePopup::AutoDeriveWorkspaces) => {
                        self.app.active(&Id::AutoDeriveWorkspacesEditor)
                    }
                    None => self.app.active(&Id::HomeView),
                }
                .unwrap();
                self.popup = p;
                None
            }
            Msg::UpdateConfig(config_update, close_popup) => {
                self.user_event_sender
                    .send(UserEvent::ConfigUpdate(config_update.clone()))
                    .unwrap();
                match config_update {
                    ConfigUpdate::LogLevel(log_level) => {
                        self.config.log_level = log_level.to_string();
                    }
                    ConfigUpdate::Backend(backend) => {
                        self.config.backend = backend;
                    }
                    ConfigUpdate::ModKey(key) => {
                        self.config.modkey = key;
                    }
                    ConfigUpdate::MouseKey(key) => {
                        self.config.mousekey = key;
                    }
                    ConfigUpdate::DisableTagSwap(swap) => {
                        self.config.disable_current_tag_swap = swap;
                    }
                    ConfigUpdate::DisableTileDrag(drag) => {
                        self.config.disable_tile_drag = drag;
                    }
                    ConfigUpdate::DisableWindowSnap(snap) => {
                        self.config.disable_window_snap = snap;
                    }
                    ConfigUpdate::FocusNewWindows(focus) => {
                        self.config.focus_new_windows = focus;
                    }
                    ConfigUpdate::SloppyMouseFollowsFocus(follow) => {
                        self.config.sloppy_mouse_follows_focus = follow;
                    }
                    ConfigUpdate::FocusBehaviour(behavior) => {
                        self.config.focus_behaviour = behavior;
                    }
                    ConfigUpdate::InsertBehavior(behavior) => {
                        self.config.insert_behavior = behavior;
                    }
                    ConfigUpdate::LayoutMode(mode) => {
                        self.config.layout_mode = mode;
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
                    }
                    ConfigUpdate::AutoDeriveWorkspaces(val) => {
                        self.config.auto_derive_workspaces = val;
                    }
                    ConfigUpdate::DisableCursorRepositionOnResize(val) => {
                        self.config.disable_cursor_reposition_on_resize = val;
                    }
                    ConfigUpdate::FocusOnActivation(focus_on_activation_behaviour) => {
                        self.config.focus_on_activation = focus_on_activation_behaviour;
                    }
                    ConfigUpdate::CreateFollowsCursor(val) => {
                        self.config.create_follows_cursor = val;
                    }
                    ConfigUpdate::WindowHidingStrategy(window_hiding_strategy) => {
                        self.config.window_hiding_strategy = window_hiding_strategy;
                    }
                    ConfigUpdate::SingleWindowBorder(val) => {
                        self.config.single_window_border = val;
                    }
                }
                // let (inner, actions) = HomeView::build_inner(&self.config);
                // self.app
                // .attr(&Id::HomeView, Attribute::Content, AttrValue::Table(inner));
                self.app
                    .remount(Id::HomeView, Box::new(HomeView::new(&self.config)), vec![])
                    .unwrap();
                if close_popup {
                    Some(Msg::SetHomePopup(None))
                } else {
                    None
                }
            }
            Msg::None => None,
        }
    }
}
