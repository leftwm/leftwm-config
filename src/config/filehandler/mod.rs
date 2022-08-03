#[cfg(feature = "guess-config")]
mod guess;
#[cfg(feature = "guess-config")]
pub use guess::generate_new_config;
#[cfg(feature = "guess-config")]
pub use guess::get_config_file;
#[cfg(feature = "guess-config")]
pub use guess::load;
#[cfg(feature = "guess-config")]
pub use guess::load_from_file;
#[cfg(feature = "guess-config")]
pub use guess::save_to_file;

#[cfg(feature = "ron-config")]
mod ron;
#[cfg(feature = "ron-config")]
pub use self::ron::generate_new_config;
#[cfg(feature = "ron-config")]
pub use self::ron::get_config_file;
#[cfg(feature = "ron-config")]
pub use self::ron::load;
#[cfg(feature = "ron-config")]
pub use self::ron::load_from_file;
#[cfg(feature = "ron-config")]
pub use self::ron::save_to_file;
