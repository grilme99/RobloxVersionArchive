use anyhow::Context;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveDeploymentInfo {
    pub version: String,
    pub client_version_upload: String,
    pub bootstrapper_version: String,
}

pub async fn get_deployment_info(
    client: &Client,
    base_url: &str,
    binary_type: &str,
    channel: &str,
) -> anyhow::Result<LiveDeploymentInfo> {
    let url = format!("{base_url}/{binary_type}/channel/{channel}");

    let deployment_info = client
        .get(&url)
        .send()
        .await
        .context(format!("Failed to send GET request to {url}"))?
        .json::<LiveDeploymentInfo>()
        .await
        .context(format!("Failed to convert response to JSON for URL {url}"))?;

    Ok(deployment_info)
}
