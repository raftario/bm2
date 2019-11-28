use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Links {
    pub project_home: Option<Url>,
    pub project_source: Option<Url>,
    pub donate: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub name: String,
    pub id: String,
    pub version: Version,
    pub game_version: String,
    pub description: Vec<String>,
    pub author: String,
    pub depends_on: Option<HashMap<String, VersionReq>>,
    pub conflicts_with: Option<HashMap<String, VersionReq>>,
    pub load_after: Option<HashSet<String>>,
    pub load_before: Option<HashSet<String>>,
    pub features: Option<HashSet<String>>,
    pub icon: Option<Url>,
    pub links: Option<Links>,
}

impl Manifest {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    pub fn to_string(&self, pretty: bool) -> serde_json::Result<String> {
        if pretty {
            serde_json::to_string_pretty(&self)
        } else {
            serde_json::to_string(&self)
        }
    }
}

impl FromStr for Manifest {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::Manifest;

    #[test]
    fn it_works() {
        let example = r#"
        {
          "$schema": "./Schema.json",
          "name": "Beat Saber IPA",
          "id": "beatsaber-ipa-reloaded",
          "description": [
            "A modified build of IPA for Beat Saber.",
            "",
            "Multiline description."
          ],
          "version": "3.12.13",
          "gameVersion": "0.13.2",
          "author": "DaNike",
          "dependsOn": {
            "ScoreSaber": "^1.7.2"
          },
          "conflictsWith": {
            "Song Loader": "^4.3.2"
          },
          "features": [],
          "links": {
            "project-source": "https://github.com/beat-saber-modding-group/BeatSaber-IPA-Reloaded",
            "project-home": "https://github.com/beat-saber-modding-group/BeatSaber-IPA-Reloaded/wiki"
          }
        }
        "#;

        let deserialised = Manifest::from_str(example).expect("Can't deserialise manifest");
        println!("{:#?}", deserialised);
        let serialised = deserialised
            .to_string(false)
            .expect("Can't serialise manifest");
        println!("{}", serialised);
        let serialised_pretty = deserialised
            .to_string(true)
            .expect("Can't serialise manifest");
        println!("{}", serialised_pretty);
    }
}
