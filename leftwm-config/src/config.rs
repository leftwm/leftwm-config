use leftwm_config_core::{
    keydaemon_config::KeyDaemonConfig,
    reexports::{
        Gutter, InsertBehavior, Keybind, Layout, LayoutMode, Margins, ScratchPad, State, Window,
    },
    structs::Workspace,
    values::FocusBehaviour,
    wayland_config::WaylandConfig,
    wm_config::WMConfig,
};

pub struct Config {}

impl WMConfig for Config {
    fn create_list_of_tag_labels(&self) -> Vec<String> {
        todo!()
    }

    fn workspaces(&self) -> Option<Vec<Workspace>> {
        todo!()
    }

    fn focus_behaviour(&self) -> FocusBehaviour {
        todo!()
    }

    fn mousekey(&self) -> Vec<String> {
        todo!()
    }

    fn create_list_of_scratchpads(&self) -> Vec<ScratchPad> {
        todo!()
    }

    fn layouts(&self) -> Vec<String> {
        todo!()
    }

    fn layout_definitions(&self) -> Vec<Layout> {
        todo!()
    }

    fn layout_mode(&self) -> LayoutMode {
        todo!()
    }

    fn insert_behavior(&self) -> InsertBehavior {
        todo!()
    }

    fn single_window_border(&self) -> bool {
        todo!()
    }

    fn focus_new_windows(&self) -> bool {
        todo!()
    }

    fn command_handler<SERVER>(command: &str, manager: &mut Manager<Self, SERVER>) -> bool
    where
        SERVER: DisplayServer,
        Self: Sized,
    {
        todo!()
    }

    fn always_float(&self) -> bool {
        todo!()
    }

    fn default_width(&self) -> i32 {
        todo!()
    }

    fn default_height(&self) -> i32 {
        todo!()
    }

    fn border_width(&self) -> i32 {
        todo!()
    }

    fn margin(&self) -> Margins {
        todo!()
    }

    fn workspace_margin(&self) -> Option<Margins> {
        todo!()
    }

    fn gutter(&self) -> Option<Vec<Gutter>> {
        todo!()
    }

    fn default_border_color(&self) -> String {
        todo!()
    }

    fn floating_border_color(&self) -> String {
        todo!()
    }

    fn focused_border_color(&self) -> String {
        todo!()
    }

    fn background_color(&self) -> String {
        todo!()
    }

    fn on_new_window_cmd(&self) -> Option<String> {
        todo!()
    }

    fn get_list_of_gutters(&self) -> Vec<Gutter> {
        todo!()
    }

    fn auto_derive_workspaces(&self) -> bool {
        todo!()
    }

    fn disable_tile_drag(&self) -> bool {
        todo!()
    }

    fn disable_window_snap(&self) -> bool {
        todo!()
    }

    fn sloppy_mouse_follows_focus(&self) -> bool {
        todo!()
    }

    fn save_state(&self, state: &State) {
        todo!()
    }

    fn load_state(&self, state: &mut State) {
        todo!()
    }

    fn setup_predefined_window(&self, state: &mut State, window: &mut Window) -> bool {
        todo!()
    }
}

impl KeyDaemonConfig for Config {
    fn modkey(&self) -> String {
        todo!()
    }

    fn mapped_bindings(&self) -> Vec<Keybind> {
        todo!()
    }
}

impl WaylandConfig for Config {}
