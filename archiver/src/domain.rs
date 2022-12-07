use std::collections::BTreeMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::http::LiveDeploymentInfo;

/// This isn't an exhaustive list of channel names (there's a lot of lesser-known ones), but it's the only ones users
/// will typically care about.
pub const CHANNEL_NAMES: [&str; 3] = ["zlive", "zcanary", "zintegration"];

/// All the supported binary types we want to archive. At some point this will hopefully grow to support Apple Silicon.
pub const BINARY_TYPES: [&str; 5] = [
    "WindowsPlayer",
    "WindowsStudio",
    "WindowsStudio64",
    "MacPlayer",
    "MacStudio",
];

/// Make sure that Global and China deployments are distinctly different in the archive.
pub enum DeploymentSpace {
    Global,
    China,
}

impl DeploymentSpace {
    pub fn client_version_url(&self) -> &str {
        match self {
            Self::Global => "https://clientsettings.roblox.com/v2/client-version",
            // China APIs live on `roblox.qq.com`.
            Self::China => "https://clientsettings.roblox.qq.com/v2/client-version",
        }
    }
}

impl ToString for DeploymentSpace {
    fn to_string(&self) -> String {
        let str = match self {
            DeploymentSpace::Global => "Global",
            DeploymentSpace::China => "China",
        };

        str.to_string()
    }
}

pub type DeployHistoryContents = BTreeMap<u64, DeploymentRecord>;

/// The actual deployment info that is saved into the DeployHistory.json files.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentRecord {
    pub client_version: String,
    pub bootstrapper_version: String,

    pub major_rev: u32,
    pub version: u32,
    pub patch: u32,
    pub change_list: u64, // Future proofing
}

impl TryFrom<LiveDeploymentInfo> for DeploymentRecord {
    type Error = anyhow::Error;

    fn try_from(record: LiveDeploymentInfo) -> Result<Self, Self::Error> {
        let mut split_version = record.version.split(".");
        let major_rev = split_version.next().context("No major_rev in version")?;
        let version = split_version.next().context("No version in version")?;
        let patch = split_version.next().context("No patch in version")?;
        let change_list = split_version.next().context("No change_list in version")?;

        Ok(Self {
            client_version: record.client_version_upload,
            bootstrapper_version: record.bootstrapper_version,

            major_rev: major_rev.parse().context("Failed to parse major_rev")?,
            version: version.parse().context("Failed to parse version")?,
            patch: patch.parse().context("Failed to parse patch")?,
            change_list: change_list.parse().context("Failed to parse change_list")?,
        })
    }
}
