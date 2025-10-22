use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base16 {
    pub base00: String,
    pub base01: String,
    pub base02: String,
    pub base03: String,
    pub base04: String,
    pub base05: String,
    pub base06: String,
    pub base07: String,
    pub base08: String,
    pub base09: String,
    #[serde(rename = "base0A")]
    pub base0a: String,
    #[serde(rename = "base0B")]
    pub base0b: String,
    #[serde(rename = "base0C")]
    pub base0c: String,
    #[serde(rename = "base0D")]
    pub base0d: String,
    #[serde(rename = "base0E")]
    pub base0e: String,
    #[serde(rename = "base0F")]
    pub base0f: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Base30 {
    pub white: String,
    pub darker_black: String,
    pub black: String,
    pub black2: String,
    pub one_bg: String,
    pub one_bg2: String,
    pub one_bg3: String,
    pub grey: String,
    pub grey_fg: String,
    pub grey_fg2: String,
    pub light_grey: String,
    pub red: String,
    pub baby_pink: String,
    pub pink: String,
    pub line: String,
    pub green: String,
    pub vibrant_green: String,
    pub nord_blue: String,
    pub blue: String,
    pub yellow: String,
    pub sun: String,
    pub purple: String,
    pub dark_purple: String,
    pub teal: String,
    pub orange: String,
    pub cyan: String,
    pub lightbg: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Palette {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_30: Option<Base30>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_16: Option<Base16>,
}

impl Palette {
    pub fn base16(&self) -> &Base16 {
        self.base_16
            .as_ref()
            .expect("Palette must have either base_16 or unsupported")
    }

    pub fn base30(&self) -> &Base30 {
        self.base_30
            .as_ref()
            .expect("Palette must have either base_30 or unsupported")
    }
}
