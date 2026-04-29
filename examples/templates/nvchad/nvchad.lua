---@type Base46Table
local M = {}

M.base_30 = {
	white = "{{ white | hex_hash }}",
	darker_black = "{{ darker_black | hex_hash }}",
	black = "{{ black | hex_hash }}",
	black2 = "{{ black2 | hex_hash }}",
	one_bg = "{{ one_bg | hex_hash }}",
	one_bg2 = "{{ one_bg2 | hex_hash }}",
	one_bg3 = "{{ one_bg3 | hex_hash }}",
	grey = "{{ grey | hex_hash }}",
	grey_fg = "{{ grey_fg | hex_hash }}",
	grey_fg2 = "{{ grey_fg2 | hex_hash }}",
	light_grey = "{{ light_grey | hex_hash }}",
	red = "{{ red | hex_hash }}",
	baby_pink = "{{ baby_pink | hex_hash }}",
	pink = "{{ pink | hex_hash }}",
	line = "{{ line | hex_hash }}",
	green = "{{ green | hex_hash }}",
	vibrant_green = "{{ vibrant_green | hex_hash }}",
	nord_blue = "{{ nord_blue | hex_hash }}",
	blue = "{{ blue | hex_hash }}",
	seablue = "{{ blue | hex_hash }}",
	yellow = "{{ yellow | hex_hash }}",
	sun = "{{ sun | hex_hash }}",
	purple = "{{ purple | hex_hash }}",
	dark_purple = "{{ dark_purple | hex_hash }}",
	teal = "{{ teal | hex_hash }}",
	orange = "{{ orange | hex_hash }}",
	cyan = "{{ cyan | hex_hash }}",
	statusline_bg = "{{ one_bg | hex_hash }}",
	lightbg = "{{ base01 | hex_hash }}",
	pmenu_bg = "{{ green | hex_hash }}",
	folder_bg = "{{ blue | hex_hash }}",
}

M.base_16 = {
	base00 = "{{ base00 | hex_hash }}",
	base01 = "{{ base01 | hex_hash }}",
	base02 = "{{ base02 | hex_hash }}",
	base03 = "{{ base03 | hex_hash }}",
	base04 = "{{ base04 | hex_hash }}",
	base05 = "{{ base05 | hex_hash }}",
	base06 = "{{ base06 | hex_hash }}",
	base07 = "{{ base07 | hex_hash }}",
	base08 = "{{ base08 | hex_hash }}",
	base09 = "{{ base09 | hex_hash }}",
	base0A = "{{ base0A | hex_hash }}",
	base0B = "{{ base0B | hex_hash }}",
	base0C = "{{ base0C | hex_hash }}",
	base0D = "{{ base0D | hex_hash }}",
	base0E = "{{ base0E | hex_hash }}",
	base0F = "{{ base0F | hex_hash }}",
}

M.type = "dark"

return M
