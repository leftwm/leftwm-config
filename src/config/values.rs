use std::os::raw::c_ulong;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(untagged)]
pub enum Size {
    Pixel(i32),
    Ratio(f32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum LayoutMode {
    Tag,
    Workspace,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FocusBehaviour {
    Sloppy,
    ClickTo,
    Driven,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum BaseCommand {
    Execute,
    CloseWindow,
    SwapTags,
    SoftReload,
    HardReload,
    ToggleScratchPad,
    ToggleFullScreen,
    ToggleSticky,
    GotoTag,
    ReturnToLastTag,
    FloatingToTile,
    TileToFloating,
    ToggleFloating,
    MoveWindowUp,
    MoveWindowDown,
    MoveWindowTop,
    FocusNextTag,
    FocusPreviousTag,
    FocusWindow,
    FocusWindowUp,
    FocusWindowDown,
    FocusWindowTop,
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    MoveToTag,
    MoveToLastWorkspace,
    MoveWindowToNextWorkspace,
    MoveWindowToPreviousWorkspace,
    MouseMoveWindow,
    NextLayout,
    PreviousLayout,
    SetLayout,
    RotateTag,
    IncreaseMainWidth,
    DecreaseMainWidth,
    SetMarginMultiplier,
    // Custom commands
    UnloadTheme,
    LoadTheme,
    CloseAllOtherWindows,
}

pub type Window = c_ulong;
type MockHandle = i32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum WindowHandle {
    MockHandle(MockHandle),
    XlibHandle(Window),
}
