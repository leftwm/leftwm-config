use crate::{
    config::{values::LayoutMode, Config},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for LayoutMode {
    const ALL_VARIANTS: &'static [Self] = &[Self::Tag, Self::Workspace];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::LayoutMode as fn(LayoutMode) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            LayoutMode::Tag => "Tag",
            LayoutMode::Workspace => "Workspace",
        }
    }

    fn name<'a>() -> &'a str {
        "Layout Mode"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.layout_mode == *self
    }
}
