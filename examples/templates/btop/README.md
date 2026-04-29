# Btop Template

**Supported mode:** `replace`

## Example target

```toml
[[targets]]
name = "btop"
template = "btop.theme"
output = "~/.config/btop/themes/themer.theme"
mode = "replace"
reload_cmd = "pkill -USR2 btop &"
```

## On program configuration

```conf
color_theme = "themer.theme"
```
