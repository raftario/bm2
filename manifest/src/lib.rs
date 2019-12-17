use lazy_static::lazy_static;
use regex::Regex;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{self, Display, Formatter},
    fs,
    io::{Read, Write},
    path::PathBuf,
    str::FromStr,
};
use url::Url;

lazy_static! {
    pub static ref ID_REGEX: Regex =
        Regex::new(r#"^([A-Z][0-9a-z]*)+(\.([A-Z][0-9a-z]*)+)*$"#).unwrap();
    pub static ref NAME_REGEX: Regex = Regex::new(r#"^[^\n\r\t]+$"#).unwrap();
    pub static ref DESCRIPTION_REGEX: Regex = Regex::new(r#"^[^\n\r]*$"#).unwrap();
}

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

/// Manifest validity error
#[derive(Debug)]
pub enum ValidityError {
    InvalidId,
    InvalidName,
    InvalidDescription,
    InvalidLinks,
    InvalidLicense,
}

impl Display for ValidityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ValidityError::InvalidId => write!(f, "Invalid manifest ID, it should follow the C# namespace naming convention"),
            ValidityError::InvalidName => write!(f, "Invalid manifest name, it should not contain tabs or newlines"),
            ValidityError::InvalidDescription => write!(f, "Invalid manifest description, it should not contain newlines"),
            ValidityError::InvalidLinks => write!(f, "Invalid manifest links, at least `project-home` or `project-source` should be specified"),
            ValidityError::InvalidLicense => write!(f, "Invalid manifest license, the license file should exist"),
        }
    }
}

impl Error for ValidityError {}

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

    /// Validates the manifest against its schema's regexps
    pub fn validate(&self) -> Result<(), ValidityError> {
        if !ID_REGEX.is_match(&self.id) {
            return Err(ValidityError::InvalidId);
        }
        if !NAME_REGEX.is_match(&self.name) {
            return Err(ValidityError::InvalidName);
        }
        if !&self
            .description
            .iter()
            .all(|l| DESCRIPTION_REGEX.is_match(l))
        {
            return Err(ValidityError::InvalidDescription);
        }
        if self.links.project_home.is_none() && self.links.project_source.is_none() {
            return Err(ValidityError::InvalidLinks);
        }
        if self.license.starts_with("SEE LICENSE IN ") {
            let license_file = self.license.replace("SEE LICENSE IN ", "");
            if fs::metadata(license_file).is_err() {
                return Err(ValidityError::InvalidLicense);
            }
        }
        Ok(())
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
          "$schema": "https://raw.githubusercontent.com/raftario/BSIPA-MetadataFileSchema/master/Schema.json",
          "name": "Example Mod",
          "id": "ExampleMod",
          "description": [
            "This is an example mod.",
            "",
            "It has a multiline description."
          ],
          "version": "1.2.3",
          "gameVersion": "0.13.2",
          "author": "DaNike",
          "license": "MIT",
          "dependsOn": {
            "SongCore": "^2.5.1"
          },
          "conflictsWith": {
            "CameraPlus": "^3.5.7"
          },
          "loadAfter": ["SongCore"],
          "loadBefore": ["ScoreSaber"],
          "features": [],
          "links": {
            "project-source": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Schema.json",
            "project-home": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Example.json"
          },
          "publish": {
            "script": ["msbuild ExampleMod/ExampleMod.csproj"],
            "resource": "ExampleMod/bin/"
          },
          "readme": "README.md",
          "icon": "ExampleMod/icon.png"
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
          "$schema": "https://raw.githubusercontent.com/raftario/BSIPA-MetadataFileSchema/master/Schema.json",
          "name": "Example Mod",
          "id": "ExampleMod",
          "description": [
            "This is an example mod.",
            "",
            "It has a multiline description."
          ],
          "version": "1.2.3",
          "gameVersion": "0.13.2",
          "author": "DaNike",
          "license": "MIT",
          "dependsOn": {
            "SongCore": "^2.5.1"
          },
          "conflictsWith": {
            "CameraPlus": "^3.5.7"
          },
          "loadAfter": ["SongCore"],
          "loadBefore": ["ScoreSaber"],
          "features": [],
          "links": {
            "project-source": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Schema.json",
            "project-home": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Example.json"
          },
          "publish": {
            "script": ["msbuild ExampleMod/ExampleMod.csproj"],
            "resource": "ExampleMod/bin/"
          },
          "readme": "README.md",
          "icon": "ExampleMod/icon.png"
        }
        "#;
        let deserialised = str_source
            .parse::<Manifest>()
            .expect("Can't deserialise manifest");
        println!("{:#?}", deserialised);
        let serialised = deserialised.to_string().expect("Can't serialise manifest");
        println!("{}", serialised);
    }

    #[test]
    fn validation() {
        let valid_source = r#"
        {
          "$schema": "https://raw.githubusercontent.com/raftario/BSIPA-MetadataFileSchema/master/Schema.json",
          "name": "Example Mod",
          "id": "ExampleMod",
          "description": [
            "This is an example mod.",
            "",
            "It has a multiline description."
          ],
          "version": "1.2.3",
          "gameVersion": "0.13.2",
          "author": "DaNike",
          "license": "MIT",
          "dependsOn": {
            "SongCore": "^2.5.1"
          },
          "conflictsWith": {
            "CameraPlus": "^3.5.7"
          },
          "loadAfter": ["SongCore"],
          "loadBefore": ["ScoreSaber"],
          "features": [],
          "links": {
            "project-source": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Schema.json",
            "project-home": "https://github.com/raftario/BSIPA-MetadataFileSchema/blob/master/Example.json"
          },
          "publish": {
            "script": ["msbuild ExampleMod/ExampleMod.csproj"],
            "resource": "ExampleMod/bin/"
          },
          "readme": "README.md",
          "icon": "ExampleMod/icon.png"
        }
        "#;
        let valid_deserialised = valid_source
            .parse::<Manifest>()
            .expect("Can't deserialise manifest");
        assert!(valid_deserialised.validate().is_ok());

        let invalid_source = r#"
        {
          "$schema": "https://raw.githubusercontent.com/raftario/BSIPA-MetadataFileSchema/master/Schema.json",
          "name": "Example Mod\n",
          "id": "example-mod",
          "description": [
            "This is an example mod.",
            "\n",
            "It has a multiline description."
          ],
          "version": "1.2.3",
          "gameVersion": "0.13.2",
          "author": "DaNike",
          "license": "SEE LICENSE IN LICENSE.txt",
          "dependsOn": {
            "SongCore": "^2.5.1"
          },
          "conflictsWith": {
            "CameraPlus": "^3.5.7"
          },
          "loadAfter": ["SongCore"],
          "loadBefore": ["ScoreSaber"],
          "features": [],
          "links": {},
          "publish": {
            "script": ["msbuild ExampleMod/ExampleMod.csproj"],
            "resource": "ExampleMod/bin/"
          },
          "readme": "README.md",
          "icon": "ExampleMod/icon.png"
        }
        "#;
        let invalid_deserialised = invalid_source
            .parse::<Manifest>()
            .expect("Can't deserialise manifest");
        assert!(invalid_deserialised.validate().is_err());
    }
}
