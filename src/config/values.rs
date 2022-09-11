use serde::{Deserialize, Serialize};
use std::os::raw::c_ulong;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(untagged)]
pub enum Size {
    Pixel(i32),
    Ratio(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    Tag,
    Workspace,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertBehavior {
    Top,
    Bottom,
    BeforeCurrent,
    AfterCurrent,
}

impl Default for InsertBehavior {
    fn default() -> Self {
        InsertBehavior::Bottom
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FocusBehaviour {
    Sloppy,
    ClickTo,
    Driven,
}

pub type Window = c_ulong;
type MockHandle = i32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowHandle {
    MockHandle(MockHandle),
    XlibHandle(Window),
}
