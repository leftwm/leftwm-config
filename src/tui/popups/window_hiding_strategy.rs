use crate::{
    config::{Config, WindowHidingStrategy},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for WindowHidingStrategy {
    const ALL_VARIANTS: &'static [Self] = &[Self::Unmap, Self::MoveOnly, Self::MoveMinimize];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::WindowHidingStrategy as fn(WindowHidingStrategy) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            WindowHidingStrategy::Unmap => "Unmap",
            WindowHidingStrategy::MoveMinimize => "MoveMinimize",
            WindowHidingStrategy::MoveOnly => "MoveOnly",
        }
    }

    fn name<'a>() -> &'a str {
        "Window Hiding Strategy"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.window_hiding_strategy == *self
    }
}
