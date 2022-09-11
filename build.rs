fn main() {
    #[cfg(debug_assertions)]
    println!("cargo:warning=leftwm-config is being build in debug mode, if build in debug mode leftwm-config will work with a `test_config.ron` (or .toml) file, build in release mode to work with `config.ron`");
}
