use crate::config::values::WindowHandle;
use crate::config::Layout;
use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize, Default, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum BaseCommand {
    #[default]
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

impl BaseCommand {
    pub fn needs_value(&self) -> bool {
        match self {
            BaseCommand::Execute => true,
            BaseCommand::CloseWindow => false,
            BaseCommand::SwapTags => false,
            BaseCommand::SoftReload => false,
            BaseCommand::HardReload => false,
            BaseCommand::ToggleScratchPad => true,
            BaseCommand::ToggleFullScreen => false,
            BaseCommand::ToggleSticky => false,
            BaseCommand::GotoTag => true,
            BaseCommand::ReturnToLastTag => false,
            BaseCommand::FloatingToTile => false,
            BaseCommand::TileToFloating => false,
            BaseCommand::ToggleFloating => false,
            BaseCommand::MoveWindowUp => false,
            BaseCommand::MoveWindowDown => false,
            BaseCommand::MoveWindowTop => false,
            BaseCommand::FocusNextTag => false,
            BaseCommand::FocusPreviousTag => false,
            BaseCommand::FocusWindow => false,
            BaseCommand::FocusWindowUp => false,
            BaseCommand::FocusWindowDown => false,
            BaseCommand::FocusWindowTop => false,
            BaseCommand::FocusWorkspaceNext => false,
            BaseCommand::FocusWorkspacePrevious => false,
            BaseCommand::MoveToTag => true,
            BaseCommand::MoveToLastWorkspace => false,
            BaseCommand::MoveWindowToNextWorkspace => false,
            BaseCommand::MoveWindowToPreviousWorkspace => false,
            BaseCommand::MouseMoveWindow => false,
            BaseCommand::NextLayout => false,
            BaseCommand::PreviousLayout => false,
            BaseCommand::SetLayout => true,
            BaseCommand::RotateTag => false,
            BaseCommand::IncreaseMainWidth => true,
            BaseCommand::DecreaseMainWidth => true,
            BaseCommand::SetMarginMultiplier => true,
            //
            BaseCommand::UnloadTheme => true,
            BaseCommand::LoadTheme => true,
            BaseCommand::CloseAllOtherWindows => false,
        }
    }
}

pub type TagId = usize;

#[allow(dead_code)]
pub enum CoreCommand {
    Execute(String),
    CloseWindow,
    SwapScreens,
    SoftReload,
    HardReload,
    ToggleScratchPad(String),
    ToggleFullScreen,
    ToggleSticky,
    GoToTag {
        tag: TagId,
        swap: bool,
    },
    ReturnToLastTag,
    FloatingToTile,
    TileToFloating,
    ToggleFloating,
    MoveWindowUp,
    MoveWindowDown,
    MoveWindowTop {
        swap: bool,
    },
    FocusNextTag,
    FocusPreviousTag,
    FocusWindow(String),
    FocusWindowUp,
    FocusWindowDown,
    FocusWindowTop {
        swap: bool,
    },
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    SendWindowToTag {
        window: Option<WindowHandle>,
        tag: TagId,
    },
    MoveWindowToLastWorkspace,
    MoveWindowToNextWorkspace,
    MoveWindowToPreviousWorkspace,
    MouseMoveWindow,
    NextLayout,
    PreviousLayout,
    SetLayout(Layout),
    RotateTag,
    IncreaseMainWidth(i8),
    DecreaseMainWidth(i8),
    SetMarginMultiplier(f32),
    SendWorkspaceToTag(usize, usize),
    CloseAllOtherWindows,
    Other(String),
}
