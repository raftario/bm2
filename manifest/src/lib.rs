use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_home: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_source: Option<Url>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub donate: Option<Url>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Publish {
    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub script: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<PathBuf>,
}

fn is_default<T: Default + PartialEq>(arg: &T) -> bool {
    arg == &Default::default()
}

/// BeatMods2 Manifest (see https://github.com/raftario/BSIPA-MetadataFileSchema)
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub id: String,

    pub name: String,

    pub version: Version,

    pub game_version: String,

    pub description: Vec<String>,

    pub author: String,

    pub license: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<HashMap<String, VersionReq>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts_with: Option<HashMap<String, VersionReq>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_after: Option<HashSet<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_before: Option<HashSet<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HashSet<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<PathBuf>,

    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub links: Links,

    #[serde(skip_serializing_if = "is_default")]
    #[serde(default)]
    pub publish: Publish,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<PathBuf>,
}

impl Manifest {
    /// Reads the manifest from a JSON reader
    pub fn from_reader<R: Read>(reader: R) -> serde_json::Result<Self> {
        serde_json::from_reader(reader)
    }

    /// Writes the prettified JSON manifest to a writer
    pub fn to_writer<W: Write>(&self, writer: W) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(writer, &self)
    }

    /// Converts the manifest to a prettified JSON string
    pub fn to_string(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}

/// Parses the manifest from a JSON string
impl FromStr for Manifest {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> serde_json::Result<Manifest> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::Manifest;

    #[test]
    fn reader_writer() {
        let reader = br#"
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
          "license": "MIT",
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
        let deserialised =
            Manifest::from_reader(reader.as_ref()).expect("Can't deserialise manifest");
        println!("{:#?}", deserialised);
        let mut serialised = Vec::new();
        deserialised
            .to_writer(&mut serialised)
            .expect("Can't serialise manifest");
        println!("{}", String::from_utf8(serialised).unwrap());
    }

    #[test]
    fn str_string() {
        let str_source = r#"
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
          "license": "MIT",
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
        let deserialised = str_source
            .parse::<Manifest>()
            .expect("Can't deserialise manifest");
        println!("{:#?}", deserialised);
        let serialised = deserialised.to_string().expect("Can't serialise manifest");
        println!("{}", serialised);
    }
}
