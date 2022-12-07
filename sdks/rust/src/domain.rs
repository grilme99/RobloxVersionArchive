use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Represents a Channel supported by Roblox.
///
/// ### `LIVE` vs `zlive`
/// The difference between the `LIVE` and `zlive` channels is not totally clear. `LIVE` is the main release channel that
/// should be used. `z` channels are typically internal, and `zlive` is most likely some kind of internal staging
/// channel.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum PrimaryChannel {
    Live,
    ZLive,
    ZCanary,
    ZIntegration,
}

impl ToString for PrimaryChannel {
    fn to_string(&self) -> String {
        let str = match self {
            PrimaryChannel::Live => "LIVE",
            PrimaryChannel::ZLive => "zlive",
            PrimaryChannel::ZCanary => "zcanary",
            PrimaryChannel::ZIntegration => "zintegration",
        };

        str.to_owned()
    }
}

/// This isn't an exhaustive list of channel names (there's a lot of lesser-known ones), but it's the only ones users
/// will typically care about.
///
/// ### `LIVE` vs `zlive`
/// The difference between the `LIVE` and `zlive` channels is not totally clear. `LIVE` is the main release channel that
/// should be used. `z` channels are typically internal, and `zlive` is most likely some kind of internal staging
/// channel.
pub const PRIMARY_CHANNELS: [PrimaryChannel; 4] = [
    PrimaryChannel::Live,
    PrimaryChannel::ZLive,
    PrimaryChannel::ZCanary,
    PrimaryChannel::ZIntegration,
];

/// All known binary types currently supported by Roblox. Open an issue if there are any more!
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum BinaryType {
    WindowsPlayer,
    WindowsStudio,
    WindowsStudio64,
    MacPlayer,
    MacStudio,
}

impl ToString for BinaryType {
    fn to_string(&self) -> String {
        let str = match self {
            BinaryType::WindowsPlayer => "WindowsPlayer",
            BinaryType::WindowsStudio => "WindowsStudio",
            BinaryType::WindowsStudio64 => "WindowsStudio64",
            BinaryType::MacPlayer => "MacPlayer",
            BinaryType::MacStudio => "MacStudio",
        };

        str.to_owned()
    }
}

/// All known binary types currently supported by Roblox. Open an issue if there are any more!
pub const BINARY_TYPES: [BinaryType; 5] = [
    BinaryType::WindowsPlayer,
    BinaryType::WindowsStudio,
    BinaryType::WindowsStudio64,
    BinaryType::MacPlayer,
    BinaryType::MacStudio,
];

/// Record files that are stored in the archive.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum RecordType {
    LatestDeploy,
    DeployHistory,
}

impl ToString for RecordType {
    fn to_string(&self) -> String {
        let str = match self {
            RecordType::LatestDeploy => "LatestDeploy.json",
            RecordType::DeployHistory => "DeployHistory.json",
        };

        str.to_owned()
    }
}

/// Make sure that Global and Luobu deployments are distinctly different in the archive.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
pub enum DeploymentSpace {
    /// The main deployment space for global users. Unless you are developing an application for China/Luobu, you must
    /// use this channel.
    Global,
    /// **WARNING:** Only use this channel if you are developing an application for Luobu! If global users launch a
    /// LuoBu application then their account will be disabled!
    ///
    /// Use at own risk.
    Luobu,
}

impl DeploymentSpace {
    /// Global and Luobu have completely separate APIs on different domains.
    pub fn get_roblox_domain(&self) -> &str {
        match self {
            Self::Global => "roblox.com",
            // All Luobu APIs live on `roblox.qq.com`.
            Self::Luobu => "roblox.qq.com",
        }
    }

    /// Quick utility for getting the `client-version` API url based on deployment space.
    pub fn get_client_version_url(&self) -> String {
        format!(
            "https://clientsettings.{}/v2/client-version",
            self.get_roblox_domain()
        )
    }
}

impl ToString for DeploymentSpace {
    fn to_string(&self) -> String {
        let str = match self {
            DeploymentSpace::Global => "Global",
            DeploymentSpace::Luobu => "Luobu",
        };

        str.to_string()
    }
}

/// Type of `DeployHistory.json` files.
pub type DeployHistory = BTreeMap<u64, DeploymentRecord>;

/// The actual deployment info that is saved into the DeployHistory.json files.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentRecord {
    pub client_version: String,
    pub bootstrapper_version: String,

    pub major_rev: u32,
    pub version: u32,
    pub patch: u32,
    pub change_list: u64, // Future proofing. Unsure how big this can get.
}
