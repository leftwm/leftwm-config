use crate::{
    config::{Backend, Config},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for Backend {
    const ALL_VARIANTS: &'static [Self] = &[Self::XLib, Self::X11rb];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::Backend as fn(Backend) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            Backend::XLib => "XLib",
            Backend::X11rb => "X11rb",
        }
    }

    fn name<'a>() -> &'a str {
        "Backend"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.backend == *self
    }
}
