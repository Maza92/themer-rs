# NvChad Template

**Supported mode:** `replace`

## Example target

```toml
[[targets]]
name = "neovim"
template = "neovim.lua"
output = "~/.config/nvim/lua/themes/dynamic.lua"
mode = "replace"
reload_cmd = "nvchad-reload-script"
```

To instantly apply the new theme to all running Neovim instances, create an executable shell script and use it as your reload_cmd:

```bash
#!/usr/bin/env dash

for addr in "$XDG_RUNTIME_DIR"/nvim.*; do
  if [ -e "$addr" ]; then
    nvim --server "$addr" --remote-expr 'luaeval("require(\"themer_reload\").reload()")' > /dev/null 2>&1
  fi
done
```

## On program configuration

NvChad's theme engine relies heavily on caching for its fast startup times. To ensure the new theme applies instantly without restarting Neovim, we need a strategy to clear this cache on the fly.

```lua
local M = {}

function M.reload()
  package.loaded["themes.dynamic"] = nil

  local data_path = vim.fn.stdpath "data"
  vim.fn.delete(data_path .. "/nvchad/base46/", "rf")
  vim.fn.delete(data_path .. "/base46/", "rf")

  require("nvconfig").base46.theme = "dynamic"
  require("base46").load_all_highlights()

  pcall(function()
    require("nvchad.utils").reload()
  end)

  vim.cmd "redraw!"
end

return M
```
