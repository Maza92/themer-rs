# Kitty template

**Supported modes:** `include`

## Example target

```toml
[[targets]]
name = "kitty"
template = "kitty.conf"
mode = "include"
output = ""
reload_cmd = ""
```

## On program configuration

```conf
# ~/.config/kitty/kitty.conf
include ~/.cache/themer/kitty.conf
```
