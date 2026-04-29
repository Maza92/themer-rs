# Fish Template

**Supported modes:** `replace`

## Example target

```toml
name = "fish"
template = "fish.fish"
output = "~/.config/fish/conf.d/colors.fish"
mode = "replace"
reload_cmd = "fish -c 'set -U THEME_TRIGGER (date +%s)'"
```

## On program configuration

For a hot reload, we need a listener

```fish
function __reload_themer_colors --on-variable THEME_TRIGGER
    if test -f ~/.config/fish/conf.d/colors.fish
        source ~/.config/fish/conf.d/colors.fish
    end
end
```
