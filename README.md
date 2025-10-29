<h1 align="center">Themer-rs</h1>

<video src="https://github.com/user-attachments/assets/8f8d4f5e-8019-4193-bc48-fb974548d9e9" autoplay loop muted playsinline width="600"></video>

<p align="center">
  <strong>A Rust-based CLI tool for centralized color palette management across Linux applications.</strong>
</p>

<p align="center">
  🎨 <strong><a href="./examples/palettes/">50+ curated themes included</a></strong>
</p>

## Overview

Themer uses Tera templates to inject color values from JSON palettes into application config files. It supports both base16 and base30 color schemes, with template hot-rendering and optional reload commands.

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
git clone https://github.com/Maza92/themer-rs
cd themer
cargo build --release
sudo cp target/release/themer /usr/local/bin/
```

## Directory Structure

```
~/.config/themer/
├── config.toml           # Main configuration
├── palettes/             # Color palette definitions (JSON)
│   ├── dracula.json
│   ├── gruvbox.json
│   └── nord.json
└── templates/            # Application templates
    ├── alacritty.toml
    ├── waybar.css
    └── kitty.conf
```

## Configuration

### config.toml

```toml
active_palette = "gruvbox"

[[targets]]
name = "Waybar"
template = "waybar.css"
mode = "include"
output = ""
reload_cmd = "pkill -SIGUSR2 waybar"

[[targets]]
name = "Alacritty"
template = "alacritty.toml"
mode = "replace"
output = "~/.config/alacritty/colors.toml"
reload_cmd = ""
```

#### Target Fields

- **name**: Identifier for the target
- **template**: Template filename in `~/.config/themer/templates/`
- **mode**:
  - `include`: Output to `~/.cache/themer/<name>.<ext>` (for apps with import support)
  - `replace`: Write directly to `output` path
- **output**: Required for `mode = "replace"`, ignored for `include`
- **reload_cmd**: Shell command to reload application (optional)
  - Use `&` suffix for background execution
  - `{theme}` placeholder available (replaced with palette name)

### Palette Format (JSON)

```json
{
  "name": "Gruvbox Dark",
  "base_16": {
    "base00": "282828",
    "base01": "3c3836",
    "base02": "504945",
    "base03": "665c54",
    "base04": "bdae93",
    "base05": "d5c4a1",
    "base06": "ebdbb2",
    "base07": "fbf1c7",
    "base08": "fb4934",
    "base09": "fe8019",
    "base0A": "fabd2f",
    "base0B": "b8bb26",
    "base0C": "8ec07c",
    "base0D": "83a598",
    "base0E": "d3869b",
    "base0F": "d65d0e"
  },
  "base_30": {
    "white": "fbf1c7",
    "darker_black": "1d2021",
    "black": "282828",
    "black2": "32302f",
    "one_bg": "3c3836",
    "one_bg2": "504945",
    "one_bg3": "665c54",
    "grey": "928374",
    "grey_fg": "a89984",
    "grey_fg2": "bdae93",
    "light_grey": "d5c4a1",
    "red": "fb4934",
    "baby_pink": "fe8019",
    "pink": "d3869b",
    "line": "504945",
    "green": "b8bb26",
    "vibrant_green": "8ec07c",
    "nord_blue": "83a598",
    "blue": "458588",
    "yellow": "fabd2f",
    "sun": "fe8019",
    "purple": "d3869b",
    "dark_purple": "b16286",
    "teal": "8ec07c",
    "orange": "fe8019",
    "cyan": "8ec07c",
    "lightbg": "504945"
  }
}
```

Both `base_16` and `base_30` are optional. Include whichever your templates require.

## Templates

Templates use Tera syntax with available variables from your palette:

```toml
# alacritty.toml
[colors.primary]
background = "{{ base00 | hex_hash }}"
foreground = "{{ base05 | hex_hash }}"

[colors.normal]
black   = "{{ base00 | hex_hash }}"
red     = "{{ base08 | hex_hash }}"
green   = "{{ base0B | hex_hash }}"
yellow  = "{{ base0A | hex_hash }}"
blue    = "{{ base0D | hex_hash }}"
magenta = "{{ base0E | hex_hash }}"
cyan    = "{{ base0C | hex_hash }}"
white   = "{{ base05 | hex_hash }}"
```

```css
/* waybar.css */
@define-color background {{ base00 | hex_hash }};
@define-color foreground {{ base05 | hex_hash }};
@define-color red {{ red | hex_hash }};
@define-color green {{ green | hex_hash }};
```

### Available Filters

- `hex_hash`: Adds `#` prefix

  ```
  {{ base00 | hex_hash }}  → #282828
  ```

- `rgb`: Converts to RGB/RGBA format
  ```
  {{ base00 | rgb }}        → rgb(40, 40, 40)
  {{ base00 | rgb(a=0.8) }} → rgba(40, 40, 40, 0.80)
  ```

### Template Variables

All colors from your palette are available as variables:

- Base16: `base00` through `base0F`
- Base30: `white`, `black`, `red`, `green`, `blue`, etc.
- Additional: `name` (palette name)

## Commands

```bash
# Apply a palette
themer apply gruvbox

# List available palettes
themer list
themer list --format json   # JSON output
themer list --format plain  # Newline-separated

# List configured targets
themer list-targets
themer list-targets --format json

# Validate templates
themer validate              # All targets
themer validate waybar       # Specific target
```

## Integration Examples

### Rofi Palette Picker

```bash
#!/bin/bash
MENU_LIST=$(themer-rs list --format plain)
SELECTED=$(echo "$MENU_LIST" | rofi -dmenu -p "Select:")

if [[ -z "$SELECTED" ]]; then
    exit 0
fi

themer-rs apply "$SELECTED"
```

### Waybar Include Mode

```toml
# config.toml
[[targets]]
name = "Waybar"
template = "waybar.css"
mode = "include"
output = ""
reload_cmd = "pkill -SIGUSR2 waybar"
```

```css
/* ~/.config/waybar/style.css */
@import "../../.cache/themer/waybar.css";
```

### Hyprland Direct Replace

```toml
[[targets]]
name = "Hyprland"
template = "hyprland.conf"
mode = "replace"
output = "~/.config/hypr/colors.conf"
reload_cmd = "hyprctl reload &"
```

## Error Handling

- Missing templates: Validation catches before apply
- Invalid JSON: Detailed parse errors with line numbers
- Failed reload commands: Logged but don't block apply
- Template syntax errors: Caught during validation

## Notes

- Reload commands with `&` suffix run in background
- Include mode preserves template file extension
- Validation uses dummy palette with all fields populated

## Example Programs

- [Waybar](./examples/templates/waybar/) – supports `include` mode

- [Gowall](./examples/templates/gowall/) – supports `replace` mode

### Terminals

- [Foot](./examples/templates/foot/) – supports `include` mode

- [Alacritty](./examples/templates/alacritty/) – supports `include` mode

- [Kitty](./examples/templates/kitty/) – supports `include` mode

_(More coming soon!)_

## Credits

These color palettes are ported from [NvChad's base46 theme collection](https://github.com/NvChad/base46).

- **Color system**: NvChad's base30 semantic naming
- **Format adaptation**: Converted from Lua to JSON for universal compatibility
