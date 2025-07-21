use crate::{
    config::{values::FocusBehaviour, Config},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for FocusBehaviour {
    const ALL_VARIANTS: &'static [Self] = &[Self::Sloppy, Self::Driven, Self::ClickTo];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::FocusBehaviour as fn(FocusBehaviour) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            FocusBehaviour::Sloppy => "Sloppy",
            FocusBehaviour::ClickTo => "ClickTo",
            FocusBehaviour::Driven => "Driven",
        }
    }

    fn name<'a>() -> &'a str {
        "Focus Behavior"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.focus_behaviour == *self
    }

    fn is_enabled_update(&self, update: &ConfigUpdate) -> bool {
        let ConfigUpdate::FocusBehaviour(f) = update else {
            return false;
        };
        f == self
    }

    fn should_update(update: &ConfigUpdate) -> bool {
        matches!(update, ConfigUpdate::FocusBehaviour(_))
    }
}
