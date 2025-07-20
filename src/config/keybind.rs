use crate::config::command::{BaseCommand, NormalisedCommand};
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

pub struct LeftHKKeybind {
    pub command: NormalisedCommand,
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
    pub fn try_convert_to_lefthk_keybind(&self, config: &Config) -> Result<LeftHKKeybind> {
        let command = match &self.command {
            BaseCommand::Execute => {
                NormalisedCommand::Execute(ensure_non_empty!(self.value.clone()))
            }
            BaseCommand::CloseWindow => NormalisedCommand::CloseWindow,
            BaseCommand::SwapTags => NormalisedCommand::SwapScreens,
            BaseCommand::SoftReload => NormalisedCommand::SoftReload,
            BaseCommand::HardReload => NormalisedCommand::HardReload,
            BaseCommand::ToggleScratchPad => {
                NormalisedCommand::ToggleScratchPad(ensure_non_empty!(self.value.clone()))
            }
            BaseCommand::ToggleFullScreen => NormalisedCommand::ToggleFullScreen,
            BaseCommand::ToggleSticky => NormalisedCommand::ToggleSticky,
            BaseCommand::GotoTag => NormalisedCommand::GoToTag {
                tag: usize::from_str(&self.value).context("invalid index value for GotoTag")?,
                swap: !config.disable_current_tag_swap,
            },
            BaseCommand::ReturnToLastTag => NormalisedCommand::ReturnToLastTag,
            BaseCommand::FloatingToTile => NormalisedCommand::FloatingToTile,
            BaseCommand::TileToFloating => NormalisedCommand::TileToFloating,
            BaseCommand::ToggleFloating => NormalisedCommand::ToggleFloating,
            BaseCommand::MoveWindowUp => NormalisedCommand::MoveWindowUp,
            BaseCommand::MoveWindowDown => NormalisedCommand::MoveWindowDown,
            BaseCommand::MoveWindowTop => NormalisedCommand::MoveWindowTop {
                swap: if self.value.is_empty() {
                    true
                } else {
                    bool::from_str(&self.value)
                        .context("invalid boolean value for MoveWindowTop")?
                },
            },
            BaseCommand::FocusNextTag => NormalisedCommand::FocusNextTag,
            BaseCommand::FocusPreviousTag => NormalisedCommand::FocusPreviousTag,
            BaseCommand::FocusWindow => NormalisedCommand::FocusWindow(self.value.clone()),
            BaseCommand::FocusWindowUp => NormalisedCommand::FocusWindowUp,
            BaseCommand::FocusWindowDown => NormalisedCommand::FocusWindowDown,
            BaseCommand::FocusWindowTop => NormalisedCommand::FocusWindowTop {
                swap: if self.value.is_empty() {
                    false
                } else {
                    bool::from_str(&self.value)
                        .context("invalid boolean value for FocusWindowTop")?
                },
            },
            BaseCommand::FocusWorkspaceNext => NormalisedCommand::FocusWorkspaceNext,
            BaseCommand::FocusWorkspacePrevious => NormalisedCommand::FocusWorkspacePrevious,
            BaseCommand::MoveToTag => NormalisedCommand::SendWindowToTag {
                window: None,
                tag: usize::from_str(&self.value)
                    .context("invalid index value for SendWindowToTag")?,
            },
            BaseCommand::MoveToLastWorkspace => NormalisedCommand::MoveWindowToLastWorkspace,
            BaseCommand::MoveWindowToNextWorkspace => NormalisedCommand::MoveWindowToNextWorkspace,
            BaseCommand::MoveWindowToPreviousWorkspace => {
                NormalisedCommand::MoveWindowToPreviousWorkspace
            }
            BaseCommand::MouseMoveWindow => NormalisedCommand::MouseMoveWindow,
            BaseCommand::NextLayout => NormalisedCommand::NextLayout,
            BaseCommand::PreviousLayout => NormalisedCommand::PreviousLayout,
            BaseCommand::SetLayout => NormalisedCommand::SetLayout(self.value.clone()),
            BaseCommand::RotateTag => NormalisedCommand::RotateTag,
            BaseCommand::IncreaseMainWidth => NormalisedCommand::IncreaseMainWidth(
                i8::from_str(&self.value).context("invalid width value for IncreaseMainWidth")?,
            ),
            BaseCommand::DecreaseMainWidth => NormalisedCommand::DecreaseMainWidth(
                i8::from_str(&self.value).context("invalid width value for DecreaseMainWidth")?,
            ),
            BaseCommand::SetMarginMultiplier => NormalisedCommand::SetMarginMultiplier(
                f32::from_str(&self.value)
                    .context("invalid margin multiplier for SetMarginMultiplier")?,
            ),
            BaseCommand::UnloadTheme => NormalisedCommand::Other("UnloadTheme".into()),
            BaseCommand::LoadTheme => NormalisedCommand::Other(format!(
                "LoadTheme {}",
                ensure_non_empty!(self.value.clone())
            )),
            BaseCommand::CloseAllOtherWindows => NormalisedCommand::CloseAllOtherWindows,
        };

        Ok(LeftHKKeybind {
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
