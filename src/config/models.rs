use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub active_pallette: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub template: String,
    pub output: String,
    pub mode: Mode,
    pub reload_cmd: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Replace,
    Include,
}
