use leftwm_core::{
    config::{InsertBehavior, Workspace},
    layouts::Layout,
    models::{FocusBehaviour, Gutter, LayoutMode, Margins, ScratchPad, WindowType},
    DisplayServer, Manager, State, Window,
};

pub trait WMConfig {
    fn create_list_of_tag_labels(&self) -> Vec<String>;

    fn workspaces(&self) -> Option<Vec<Workspace>>;

    fn focus_behaviour(&self) -> FocusBehaviour;

    fn mousekey(&self) -> Vec<String>;

    fn create_list_of_scratchpads(&self) -> Vec<ScratchPad>;

    fn layouts(&self) -> Vec<String>;

    fn layout_definitions(&self) -> Vec<Layout>;

    fn layout_mode(&self) -> LayoutMode;

    fn insert_behavior(&self) -> InsertBehavior;

    fn single_window_border(&self) -> bool;

    fn focus_new_windows(&self) -> bool;

    fn command_handler<SERVER>(command: &str, manager: &mut Manager<Self, SERVER>) -> bool
    where
        SERVER: DisplayServer,
        Self: Sized;

    fn always_float(&self) -> bool;
    fn default_width(&self) -> i32;
    fn default_height(&self) -> i32;
    fn border_width(&self) -> i32;
    fn margin(&self) -> Margins;
    fn workspace_margin(&self) -> Option<Margins>;
    fn gutter(&self) -> Option<Vec<Gutter>>;
    fn default_border_color(&self) -> String;
    fn floating_border_color(&self) -> String;
    fn focused_border_color(&self) -> String;
    fn background_color(&self) -> String;
    fn on_new_window_cmd(&self) -> Option<String>;
    fn get_list_of_gutters(&self) -> Vec<Gutter>;
    fn auto_derive_workspaces(&self) -> bool;
    fn disable_tile_drag(&self) -> bool;
    fn disable_window_snap(&self) -> bool;
    fn sloppy_mouse_follows_focus(&self) -> bool;

    /// Attempt to write current state to a file.
    ///
    /// It will be used to restore the state after soft reload.
    ///
    /// **Note:** this function cannot fail.
    fn save_state(&self, state: &State);

    /// Load saved state if it exists.
    fn load_state(&self, state: &mut State);

    /// Handle window placement based on `WM_CLASS`
    fn setup_predefined_window(&self, state: &mut State, window: &mut Window) -> bool;

    fn load_window(&self, window: &mut Window) {
        if window.r#type == WindowType::Normal {
            window.margin = self.margin();
            window.border = self.border_width();
            // window.must_float = self.always_float(); //NOTE: renable?
        } else {
            window.margin = Margins::new(0);
            window.border = 0;
        }
    }
}
