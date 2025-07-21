use crate::{
    config::{Config, FocusOnActivationBehaviour},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for FocusOnActivationBehaviour {
    const ALL_VARIANTS: &'static [Self] = &[Self::SwitchTo, Self::DoNothing, Self::MarkUrgent];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::FocusOnActivation as fn(FocusOnActivationBehaviour) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            FocusOnActivationBehaviour::DoNothing => "DoNothing",
            FocusOnActivationBehaviour::MarkUrgent => "MarkUrgent",
            FocusOnActivationBehaviour::SwitchTo => "SwitchTo",
        }
    }

    fn name<'a>() -> &'a str {
        "Focus On Activation Behaviour"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.focus_on_activation == *self
    }

    fn is_enabled_update(&self, update: &ConfigUpdate) -> bool {
        let ConfigUpdate::FocusOnActivation(f) = update else {
            return false;
        };
        f == self
    }

    fn should_update(update: &ConfigUpdate) -> bool {
        matches!(update, ConfigUpdate::FocusOnActivation(_))
    }
}
