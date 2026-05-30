use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::errors::FsdMessageParseError;

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(default)]
pub struct AircraftConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_full_data: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lights: Option<AircraftLightsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<AircraftEnginesConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gear_down: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flaps_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoilers_out: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_ground: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_cg_height: Option<f64>,
}
impl FromStr for AircraftConfig {
    type Err = FsdMessageParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Value = serde_json::from_str(s)
            .map_err(|_| FsdMessageParseError::InvalidAircraftConfig(s.to_string()))?;
        let config_val = value
            .get("config")
            .ok_or_else(|| FsdMessageParseError::InvalidAircraftConfig(s.to_string()))?
            .clone();
        match serde_json::from_value(config_val) {
            Ok(config) => Ok(config),
            Err(_) => Err(FsdMessageParseError::InvalidAircraftConfig(s.to_string())),
        }
    }
}
impl Display for AircraftConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialised = json!({
            "config": self,
        });
        write!(f, "{serialised}")
    }
}
#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AircraftLightsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strobe_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxi_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beacon_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_on: Option<bool>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AircraftEnginesConfig {
    #[serde(rename = "1", skip_serializing_if = "Option::is_none")]
    pub engine_1: Option<AircraftEngine>,
    #[serde(rename = "2", skip_serializing_if = "Option::is_none")]
    pub engine_2: Option<AircraftEngine>,
    #[serde(rename = "3", skip_serializing_if = "Option::is_none")]
    pub engine_3: Option<AircraftEngine>,
    #[serde(rename = "4", skip_serializing_if = "Option::is_none")]
    pub engine_4: Option<AircraftEngine>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AircraftEngine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reversing: Option<bool>,
}
