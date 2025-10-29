# Alacritty template

**Supported modes:** `include`

## Example target

```toml
[[targets]]
name = "alacritty"
template = "alacritty.toml"
mode = "include"
output = ""
reload_cmd = ""
```

## On program configuration

```toml
# ~/.config/alacritty/alacritty.toml
[general]
import = [
   "~/.cache/themer/alacritty.toml"
]
```
