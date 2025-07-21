use tui_realm_stdlib::Table;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextSpan},
    Component, Event, MockComponent,
};

use crate::config::{modifier::Modifier as KeyModifier, Config};

use super::{format_modkey_name, model::UserEvent, popups::SelectorEnum, HomePopup, Msg, View};

#[derive(MockComponent)]
pub struct HomeView {
    component: Table,
    popups: Vec<HomeAction>,
}

pub enum HomeAction {
    Popup(HomePopup),
    SwitchView(View),
    None,
}

impl HomeView {
    pub fn new(config: &Config) -> Self {
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

    pub fn build_inner(config: &Config) -> (Vec<Vec<TextSpan>>, Vec<HomeAction>) {
        let mut popups = Vec::new();

        let mut table = TableBuilder::default();

        let mut config_option = |name: &str, val: &str, popup: HomeAction| {
            table
                .add_col(TextSpan::new(name))
                .add_col(TextSpan::new(val))
                .add_row();
            popups.push(popup);
        };

        config_option(
            "Log level",
            &config.log_level,
            HomeAction::Popup(HomePopup::LogLevel),
        );
        config_option(
            "Backend",
            config.backend.variant_name(),
            HomeAction::Popup(HomePopup::Backend),
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
            HomeAction::Popup(HomePopup::MouseKey),
        );
        config_option(
            "Disable Tile Drag",
            &format!("{}", config.disable_tile_drag),
            HomeAction::Popup(HomePopup::DisableTileDrag),
        );
        config_option(
            "Disable Window Snap",
            &format!("{}", config.disable_window_snap),
            HomeAction::Popup(HomePopup::DisableWindowSnap),
        );
        config_option(
            "Disable Current Tag Swap",
            &format!("{}", config.disable_current_tag_swap),
            HomeAction::Popup(HomePopup::DisableTagSwap),
        );
        config_option(
            "Disable Cursor Reposition On Resize",
            &format!("{}", config.disable_cursor_reposition_on_resize),
            HomeAction::Popup(HomePopup::DisableCursorRepositionOnResize),
        );

        config_option(
            "Focus Behavior",
            config.focus_behaviour.variant_name(),
            HomeAction::Popup(HomePopup::FocusBehaviour),
        );
        config_option(
            "Focus New Windows",
            &format!("{}", config.focus_new_windows),
            HomeAction::Popup(HomePopup::FocusNewWindows),
        );
        config_option(
            "Sloppy Mouse Follows Focus",
            &format!("{}", config.sloppy_mouse_follows_focus),
            HomeAction::Popup(HomePopup::SloppyMouseFollowsFocus),
        );
        config_option(
            "Focus On Activation Behaviour",
            config.focus_on_activation.variant_name(),
            HomeAction::Popup(HomePopup::FocusOnActivationBehaviour),
        );

        config_option(
            "Insert Behavior",
            config.insert_behavior.variant_name(),
            HomeAction::Popup(HomePopup::InsertBehavior),
        );
        config_option(
            "Create Follows Cursor",
            &config
                .create_follows_cursor
                .map_or("unset".to_string(), |b| format!("{b}")),
            HomeAction::Popup(HomePopup::CreateFollowsCursor),
        );

        config_option(
            "Layout Mode",
            config.layout_mode.variant_name(),
            HomeAction::Popup(HomePopup::LayoutMode),
        );
        config_option(
            "Layouts",
            &format!("{} set", config.layouts.len()),
            HomeAction::None,
        );
        config_option(
            "Layout Definitions",
            &format!("{} set", config.layout_definitions.len()),
            HomeAction::None,
        );

        config_option(
            "Auto Derive Workspaces",
            &format!("{}", config.auto_derive_workspaces),
            HomeAction::Popup(HomePopup::AutoDeriveWorkspaces),
        );
        if !config.auto_derive_workspaces {
            config_option(
                "Workspaces",
                &format!(
                    "{} set",
                    config.workspaces.as_ref().unwrap_or(&vec![]).len()
                ),
                HomeAction::None,
            );
        }

        config_option(
            "Scratchpads",
            &format!(
                "{} set",
                config.scratchpad.as_ref().unwrap_or(&vec![]).len()
            ),
            HomeAction::None,
        );

        config_option(
            "Tags",
            &format!("{} set", config.tags.as_ref().unwrap_or(&vec![]).len()),
            HomeAction::None,
        );
        config_option(
            "Window Hiding Strategy",
            config.window_hiding_strategy.variant_name(),
            HomeAction::Popup(HomePopup::WindowHidingStrategy),
        );

        config_option(
            "Window Rules",
            &format!(
                "{} set",
                config.window_rules.as_ref().unwrap_or(&vec![]).len()
            ),
            HomeAction::None,
        );
        config_option(
            "Single Window Border",
            &format!("{}", config.single_window_border),
            HomeAction::Popup(HomePopup::SingleWindowBorder),
        );

        config_option(
            "Modkey",
            &format_modkey_name(config.modkey.clone()),
            HomeAction::Popup(HomePopup::ModKey),
        );
        config_option(
            "Keybinds",
            &format!("{} set", config.keybind.len()),
            HomeAction::None,
        );

        table
            .add_col(TextSpan::new("State Path"))
            .add_col(TextSpan::new(&match &config.state_path {
                Some(p) => format!("{}", p.display()),
                None => "Not set".to_string(),
            }));
        popups.push(HomeAction::Popup(HomePopup::StatePath));
        (table.build(), popups)
    }
}

impl Component<Msg, UserEvent> for HomeView {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
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
                HomeAction::Popup(home_popup) => return Some(Msg::SetHomePopup(Some(*home_popup))),
                HomeAction::SwitchView(_view) => todo!(),
                HomeAction::None => CmdResult::None,
            },
            _ => CmdResult::None,
        };
        None
    }
}
