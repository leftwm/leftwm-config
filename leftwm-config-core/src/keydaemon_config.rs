use lefthk_core::config::Keybind;

pub trait KeyDaemonConfig {
    fn modkey(&self) -> String;

    fn mapped_bindings(&self) -> Vec<Keybind>;
}
