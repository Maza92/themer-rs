# NvChad Template

**Supported mode:** `replace`

## Example target

```toml
[[targets]]
name = "neovim"
template = "neovim.lua"
output = "~/.config/nvim/lua/themes/themer_dynamic.lua"
mode = "replace"
reload_cmd = "neovim-reset &"
```

To instantly apply the new theme to all running Neovim instances, create an executable shell script and use it as your reload_cmd:

```bash
#!/usr/bin/env dash

for addr in "$XDG_RUNTIME_DIR"/nvim.*; do
  if [ -S "$addr" ]; then
    timeout 1 nvim --server "$addr" --remote-expr 'luaeval("require(\"nvchad.utils\").reload(\"themes\")")' > /dev/null 2>&1
  fi
done
```

## On program configuration

chadrc.lua
```lua
local M = {}

M.base46 = {
  theme = "themer_dynamic",
}

return M
```
