# GTK Template

**Supported modes:** `include`

> **Note:** You need the GTK Material theme installed for this to work.

## Example target

```toml
[[targets]]
name = "gtk"
template = "gtk.css"
output = ""
mode = "include"
reload_cmd = ""
```

## On program configuration

!Note: Need gtk Material theme installed to work.

```css
/* ./Material/gtk-3.0/gtk.css */
@import url("../../../.cache/themer/gtk.css"); /* cache location */

/* ./Material/gtk-4.0/gtk.css */
@import url("../../../.cache/themer/gtk.css"); /* cache location */
```
