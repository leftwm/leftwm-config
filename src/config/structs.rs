use crate::config::layout::Layout;
use crate::config::values::Size;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, Clone, PartialEq)]
pub struct Workspace {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
    pub id: Option<i32>,
    pub max_window_width: Option<Size>,
    pub layouts: Option<Vec<Layout>>,
}

#[derive(Serialize, Default, Deserialize, Debug, Clone, PartialEq)]
pub struct ScratchPad {
    pub name: String,
    pub value: String,
    // relative x of scratchpad, 25 means 25% of workspace x
    pub x: Option<Size>,
    // relative y of scratchpad, 25 means 25% of workspace y
    pub y: Option<Size>,
    // relative height of scratchpad, 50 means 50% of workspace height
    pub height: Option<Size>,
    // relative width of scratchpad, 50 means 50% of workspace width
    pub width: Option<Size>,
}

/// Selecting by `WM_CLASS` and/or window title, allow the user to define if a
/// window should spawn on a specified tag and/or its floating state.
///
/// # Example
///
/// In `config.toml`
///
/// ```toml
/// [[window_config_by_class]]
/// wm_class = "krita"
/// spawn_on_tag = 3
/// spawn_floating = false
/// ```
///
/// windows whose `WM_CLASS` is "krita" will spawn on tag 3 (1-indexed) and not floating.
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct WindowHook {
    /// `WM_CLASS` in X11
    pub window_class: Option<String>,
    /// `_NET_WM_NAME` in X11
    pub window_title: Option<String>,
    pub spawn_on_tag: Option<usize>,
    pub spawn_floating: Option<bool>,
}
