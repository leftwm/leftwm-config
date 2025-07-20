use crate::{
    config::{values::LogLevel, Config},
    tui::ConfigUpdate,
};

use super::SelectorEnum;

impl SelectorEnum for LogLevel {
    const ALL_VARIANTS: &'static [Self] = &[
        Self::Off,
        Self::Error,
        Self::Warn,
        Self::Info,
        Self::Debug,
        Self::Trace,
    ];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(ConfigUpdate::LogLevel as fn(LogLevel) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self {
            LogLevel::Off => "Off",
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Trace => "Trace",
        }
    }

    fn name<'a>() -> &'a str {
        "Log Level"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        *self == config.log_level
    }
}
