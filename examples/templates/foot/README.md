# Foot Template

**Supported modes:** `include`

## Example target

```toml
[[targets]]
name = "foot"
template = "foot.ini"
output = ""
mode = "include"
reload_cmd = ""  # Not hot-reloadable
```

## On program configuration

```ini
# ~/.config/foot/foot.ini
[main]
include=~/.cache/themer/foot.ini
```
