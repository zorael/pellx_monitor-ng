use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PrintlnConfig {
    pub strings: Option<super::MessageStrings>,
    pub enabled: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SlackConfig {
    pub strings: Option<super::MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BatsignConfig {
    pub strings: Option<super::MessageStrings>,
    pub enabled: Option<bool>,
    pub urls: Option<Vec<String>>,
    pub show_response: Option<bool>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub strings: Option<super::MessageStrings>,
    pub enabled: Option<bool>,
    pub commands: Option<Vec<String>>,
    pub show_response: Option<bool>,
}
