use crate::globals::{TERM_ERR, USER_AGENT};
use anyhow::{Context, Result};
use cfg_if::cfg_if;
use dialoguer::Confirmation;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use reqwest::blocking::ClientBuilder;
use semver::{SemVerError, Version};
use serde::Deserialize;
use std::{
    convert::{TryFrom, TryInto},
    env, fs,
    io::{Cursor, Read},
    process::{self, Command},
    thread,
    time::Duration,
};
use zip::ZipArchive;

const RELEASES_URL: &str = "https://api.github.com/repos/raftario/bm2/releases";
cfg_if! {
    if #[cfg(target_os = "windows")] {
        const RELEASE_ASSET_NAME: &str = "Windows.zip";
    } else if #[cfg(target_os = "macos")] {
        const RELEASE_ASSET_NAME: &str = "macOS.zip";
    } else {
        const RELEASE_ASSET_NAME: &str = "Linux.zip";
    }
}
lazy_static! {
    static ref VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

/// Required GitHub release info
#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

/// Versioned GitHub release info
struct VersionedRelease {
    version: Version,
    assets: Vec<ReleaseAsset>,
}

impl TryFrom<Release> for VersionedRelease {
    type Error = SemVerError;

    fn try_from(value: Release) -> Result<Self, Self::Error> {
        Ok(Self {
            version: Version::parse(&value.tag_name[1..])?,
            assets: value.assets,
        })
    }
}

/// Required GitHub release asset info
#[derive(Deserialize, Debug)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

/// Check for updates and installs them
pub fn update() -> Result<()> {
    let p = ProgressBar::new_spinner();
    p.set_message("Checking for updates");
    p.enable_steady_tick(100);

    let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

    let releases: Vec<Release> = client.get(RELEASES_URL).send()?.json()?;
    let mut new_releases: Vec<VersionedRelease> = releases
        .into_iter()
        .filter_map(|r| r.try_into().ok())
        .filter(|vr: &VersionedRelease| {
            vr.version > *VERSION && vr.assets.iter().any(|a| a.name == RELEASE_ASSET_NAME)
        })
        .collect();
    if new_releases.is_empty() {
        p.finish_and_clear();
        return Ok(());
    }
    new_releases.sort_by(|a, b| b.version.cmp(&a.version));

    let new_version = &new_releases[0].version;
    let url = &new_releases[0]
        .assets
        .iter()
        .find(|a| a.name == RELEASE_ASSET_NAME)
        .unwrap()
        .browser_download_url;

    p.finish_and_clear();
    let install = Confirmation::new()
        .with_text(&format!(
            "A new version is available, do you want to update? (current: {}, new: {})",
            &*VERSION, new_version
        ))
        .interact_on(&*TERM_ERR)?;
    TERM_ERR.clear_last_lines(1)?;
    if !install {
        return Ok(());
    }

    let p = ProgressBar::new_spinner();
    p.set_message("Downloading new update");
    p.enable_steady_tick(100);
    let mut dl = client.get(url).send()?;

    p.set_message("Installing new update");
    let mut asset = Cursor::new(Vec::new());
    dl.copy_to(&mut asset)?;
    let mut zip = ZipArchive::new(asset)?;
    let mut zipfile = None;
    for i in 0..(zip.len()) {
        if zip.by_index(i)?.is_file() {
            zipfile = Some(zip.by_index(i)?);
            break;
        }
    }
    let mut zipfile = zipfile.context("Missing file in GitHub download")?;
    let mut file = Vec::with_capacity(zipfile.size() as usize);
    zipfile.read_to_end(&mut file)?;

    let current_exe = env::current_exe()?;
    cfg_if! {
        if #[cfg(windows)] {
            let old_exe = format!("{}.old.exe", current_exe.display());
            let new_exe = format!("{}.new.exe", current_exe.display());
        } else {
            let old_exe = format!("{}.old", current_exe.display());
            let new_exe = format!("{}.new", current_exe.display());
        }
    }
    fs::copy(&current_exe, &old_exe)?;
    fs::write(&new_exe, file)?;

    p.finish_and_clear();
    Command::new(old_exe).arg("finish_update").spawn()?;
    thread::sleep(Duration::from_millis(250));
    process::exit(0);
}
// This is required cause Windows doesn't let you move an executable if it's running
pub fn finish_update() -> Result<()> {
    let p = ProgressBar::new_spinner();
    p.set_message("Finalizing");
    p.enable_steady_tick(100);

    thread::sleep(Duration::from_millis(500));
    let current_exe = env::current_exe()?;
    let current_exe = current_exe.to_str().context("Invalid current exe path")?;
    cfg_if! {
        if #[cfg(windows)] {
            let old_exe = &current_exe[..(current_exe.len() - 8)];
            let new_exe = format!("{}.new.exe", old_exe);
        } else {
            let old_exe = &current_exe[..(current_exe.len() - 4)];
            let new_exe = format!("{}.new", old_exe);
        }
    }
    fs::remove_file(old_exe)?;
    fs::rename(new_exe, old_exe)?;

    p.finish_and_clear();
    Ok(())
}
