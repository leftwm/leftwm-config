use crate::config::values::Size;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug, Clone, PartialEq)]
pub struct Workspace {
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
    pub output: String,
    pub relative: Option<bool>,
    pub layouts: Option<Vec<String>>,
    /// The default layout from the config; introduced in 0.5.4
    #[serde(default)]
    pub default_layout: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct ScratchPadName(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScratchPad {
    pub name: ScratchPadName,
    pub args: Option<Vec<String>>,
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

impl From<String> for ScratchPadName {
    fn from(other: String) -> Self {
        Self(other)
    }
}

impl From<ScratchPadName> for String {
    fn from(other: ScratchPadName) -> Self {
        other.0
    }
}

impl From<&str> for ScratchPadName {
    fn from(other: &str) -> Self {
        Self(other.to_string())
    }
}

impl PartialEq<&str> for ScratchPadName {
    fn eq(&self, other: &&str) -> bool {
        &self.0.as_str() == other
    }
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
