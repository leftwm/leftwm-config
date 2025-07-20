use crate::{
    config::{values::InsertBehavior, Config},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for InsertBehavior {
    const ALL_VARIANTS: &'static [Self] = &[
        Self::Top,
        Self::Bottom,
        Self::BeforeCurrent,
        Self::AfterCurrent,
    ];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::InsertBehavior as fn(InsertBehavior) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            InsertBehavior::Top => "Top",
            InsertBehavior::Bottom => "Bottom",
            InsertBehavior::BeforeCurrent => "BeforeCurrent",
            InsertBehavior::AfterCurrent => "AfterCurrent",
        }
    }

    fn name<'a>() -> &'a str {
        "Insert Behavior"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.insert_behavior == *self
    }
}
