use crate::config::command::{BaseCommand, CoreCommand};
use crate::config::layout::Layout;
use crate::config::modifier::Modifier;
use crate::config::Config;
use anyhow::ensure;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
pub struct Keybind {
    pub command: BaseCommand,
    #[serde(default)]
    pub value: String,
    pub modifier: Option<Modifier>,
    pub key: String,
}

pub struct CoreKeybind {
    pub command: CoreCommand,
    pub modifier: Vec<String>,
    pub key: String,
}

macro_rules! ensure_non_empty {
    ($value:expr) => {{
        ensure!(!$value.is_empty(), "value must not be empty");
        $value
    }};
}

impl Keybind {
    pub fn try_convert_to_core_keybind(&self, config: &Config) -> Result<CoreKeybind> {
        let command = match &self.command {
            BaseCommand::Execute => CoreCommand::Execute(ensure_non_empty!(self.value.clone())),
            BaseCommand::CloseWindow => CoreCommand::CloseWindow,
            BaseCommand::SwapTags => CoreCommand::SwapScreens,
            BaseCommand::SoftReload => CoreCommand::SoftReload,
            BaseCommand::HardReload => CoreCommand::HardReload,
            BaseCommand::ToggleScratchPad => {
                CoreCommand::ToggleScratchPad(ensure_non_empty!(self.value.clone()))
            }
            BaseCommand::ToggleFullScreen => CoreCommand::ToggleFullScreen,
            BaseCommand::ToggleSticky => CoreCommand::ToggleSticky,
            BaseCommand::GotoTag => CoreCommand::GoToTag {
                tag: usize::from_str(&self.value).context("invalid index value for GotoTag")?,
                swap: !config.disable_current_tag_swap,
            },
            BaseCommand::ReturnToLastTag => CoreCommand::ReturnToLastTag,
            BaseCommand::FloatingToTile => CoreCommand::FloatingToTile,
            BaseCommand::TileToFloating => CoreCommand::TileToFloating,
            BaseCommand::ToggleFloating => CoreCommand::ToggleFloating,
            BaseCommand::MoveWindowUp => CoreCommand::MoveWindowUp,
            BaseCommand::MoveWindowDown => CoreCommand::MoveWindowDown,
            BaseCommand::MoveWindowTop => CoreCommand::MoveWindowTop {
                swap: if self.value.is_empty() {
                    true
                } else {
                    bool::from_str(&self.value)
                        .context("invalid boolean value for MoveWindowTop")?
                },
            },
            BaseCommand::FocusNextTag => CoreCommand::FocusNextTag,
            BaseCommand::FocusPreviousTag => CoreCommand::FocusPreviousTag,
            BaseCommand::FocusWindow => CoreCommand::FocusWindow(self.value.clone()),
            BaseCommand::FocusWindowUp => CoreCommand::FocusWindowUp,
            BaseCommand::FocusWindowDown => CoreCommand::FocusWindowDown,
            BaseCommand::FocusWindowTop => CoreCommand::FocusWindowTop {
                swap: if self.value.is_empty() {
                    false
                } else {
                    bool::from_str(&self.value)
                        .context("invalid boolean value for FocusWindowTop")?
                },
            },
            BaseCommand::FocusWorkspaceNext => CoreCommand::FocusWorkspaceNext,
            BaseCommand::FocusWorkspacePrevious => CoreCommand::FocusWorkspacePrevious,
            BaseCommand::MoveToTag => CoreCommand::SendWindowToTag {
                window: None,
                tag: usize::from_str(&self.value)
                    .context("invalid index value for SendWindowToTag")?,
            },
            BaseCommand::MoveToLastWorkspace => CoreCommand::MoveWindowToLastWorkspace,
            BaseCommand::MoveWindowToNextWorkspace => CoreCommand::MoveWindowToNextWorkspace,
            BaseCommand::MoveWindowToPreviousWorkspace => {
                CoreCommand::MoveWindowToPreviousWorkspace
            }
            BaseCommand::MouseMoveWindow => CoreCommand::MouseMoveWindow,
            BaseCommand::NextLayout => CoreCommand::NextLayout,
            BaseCommand::PreviousLayout => CoreCommand::PreviousLayout,
            BaseCommand::SetLayout => CoreCommand::SetLayout(
                Layout::from_str(&self.value)
                    .context("could not parse layout for command SetLayout")?,
            ),
            BaseCommand::RotateTag => CoreCommand::RotateTag,
            BaseCommand::IncreaseMainWidth => CoreCommand::IncreaseMainWidth(
                i8::from_str(&self.value).context("invalid width value for IncreaseMainWidth")?,
            ),
            BaseCommand::DecreaseMainWidth => CoreCommand::DecreaseMainWidth(
                i8::from_str(&self.value).context("invalid width value for DecreaseMainWidth")?,
            ),
            BaseCommand::SetMarginMultiplier => CoreCommand::SetMarginMultiplier(
                f32::from_str(&self.value)
                    .context("invalid margin multiplier for SetMarginMultiplier")?,
            ),
            BaseCommand::UnloadTheme => CoreCommand::Other("UnloadTheme".into()),
            BaseCommand::LoadTheme => CoreCommand::Other(format!(
                "LoadTheme {}",
                ensure_non_empty!(self.value.clone())
            )),
            BaseCommand::CloseAllOtherWindows => CoreCommand::CloseAllOtherWindows,
        };

        Ok(CoreKeybind {
            command,
            modifier: self
                .modifier
                .as_ref()
                .unwrap_or(&"None".into())
                .clone()
                .into(),
            key: self.key.clone(),
        })
    }
}
