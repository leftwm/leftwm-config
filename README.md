# LeftWM Config

This is a little satellite utility to LeftWM.

It serves these main purposes:
- create a default config file in `$HOME/.config/leftwm/config.ron`
- migrate legacy `toml` config files to the new stander `ron`
- edit the config with a TUI

For usage please refer to `leftwm-config --help` for now.

*Note: this tool is BETA software, so expect some kinks and wrinkles here and there.*

To install...

```
git clone https://github.com/leftwm/leftwm-config.git &&
cd leftwm-config &&
cargo build --release &&
sudo ln -s "$(pwd)"/target/release/leftwm-config /usr/bin/leftwm-config ## Dev Install
sudo cp "$(pwd)"/target/release/leftwm-config /usr/bin/leftwm-config ## Normal Install
```
