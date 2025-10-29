# Gowall Template

**Supported modes:** `replace`

[Gowall](https://github.com/Achno/gowall) is an image converter for any color scheme.  
This example is for dynamically changing the wallpaper.

## Example target

```toml
[[targets]]
name = "gowall"
template = "gowall.yml"
mode = "replace"
output = "~/.config/gowall/config.yml"
reload_cmd = "~/.local/bin/change-wallpaper.sh {theme}"
```

## Program configuration

No configuration is needed for this template.

## Script for changing wallpaper

```bash
#!/bin/bash

WALLPAPER_DIR="$HOME/Wallpaper"

if [ -z "$1" ]; then
  exit 1
fi

THEME_NAME="$1"

RANDOM_WALLPAPER=$(find "$WALLPAPER_DIR" -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" -o -iname "*.webp" \) | shuf -n 1)

if [ -z "$RANDOM_WALLPAPER" ]; then
  exit 1
fi

gowall convert "$RANDOM_WALLPAPER" -t "$THEME_NAME" --output "$HOME/.cache/themer/wallpaper.png"
```

## Window manager configuration

Before reloading your window manager or setting the wallpaper, make sure to run the script to generate the new wallpaper with Gowall.

### sway

```conf
# ~/.config/sway/config
output * bg ~/.cache/themer/wallpaper.png fill
```
