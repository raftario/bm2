use crate::globals::CONFIG_PATH;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};

fn is_default<T: Default + PartialEq>(arg: &T) -> bool {
    arg == &Default::default()
}
fn schema() -> String {
    concat!(
        "https://raw.githubusercontent.com/raftario/bm2/v",
        env!("CARGO_PKG_VERSION"),
        "/config.schema.json"
    )
    .to_owned()
}

/// bm2 config
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "schema")]
    #[serde(rename = "$schema")]
    schema: String,

    /// Enable automatic updates
    pub auto_update: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub defaults: Defaults,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub credentials: Credentials,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema: schema(),
            auto_update: true,
            defaults: Defaults::default(),
            credentials: Credentials::default(),
        }
    }
}

impl Config {
    /// Read config from disk or creates a default one if it doesn't exist
    pub fn read() -> Result<Self> {
        if !CONFIG_PATH.exists() {
            let config = Self::default();
            config.write()?;
            return Ok(config);
        }

        let f = File::open(&*CONFIG_PATH)?;
        Ok(serde_json::from_reader(f)?)
    }

    /// Writes config to disk
    pub fn write(&self) -> Result<()> {
        let mut config_dir = CONFIG_PATH.clone();
        config_dir.pop();
        fs::create_dir_all(config_dir)?;

        let f = File::create(&*CONFIG_PATH)?;
        serde_json::to_writer_pretty(f, self)?;
        Ok(())
    }
}

/// Default values for manifest initialisation
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Defaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

/// BeatMods1 credentials (legacy)
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Credentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
