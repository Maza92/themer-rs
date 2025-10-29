# Waybar Template

**Supported modes:** `include`

## Example target

```toml
[[targets]]
name = "Waybar"
template = "waybar.css"
output = ""
mode = "include"
reload_cmd = ""
```

## On program configuration

```css
/* ~/.config/waybar/style.css */
@import "../../.cache/themer/waybar.css"; /* cache location */
```
