use crate::{config::Config, tui::ConfigUpdate};

use super::SelectorEnum;

#[derive(Debug, Clone, Copy)]
pub struct CreateFollowsCursor(Option<bool>);

impl SelectorEnum for CreateFollowsCursor {
    const ALL_VARIANTS: &'static [Self] = &[Self(None), Self(Some(false)), Self(Some(true))];

    const CONFIG_UPDATE: &'static fn(Self) -> ConfigUpdate =
        &(CreateFollowsCursor::update as fn(CreateFollowsCursor) -> ConfigUpdate);

    fn variant_name(&self) -> &str {
        match self.0 {
            Some(true) => "True",
            Some(false) => "False",
            None => "Unset",
        }
    }

    fn name<'a>() -> &'a str {
        "Create Follows Cursor"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        config.create_follows_cursor == self.0
    }
}

impl CreateFollowsCursor {
    fn update(self) -> ConfigUpdate {
        ConfigUpdate::CreateFollowsCursor(self.0)
    }
}
