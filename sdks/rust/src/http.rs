use reqwest::Client;

use crate::prelude::{
    BinaryType, DeployHistory, DeploymentRecord, DeploymentSpace, PrimaryChannel, RecordType,
};

/// Retrieves the latest deployment for the specified parameters.
pub async fn get_latest_deployment(
    deployment_space: &DeploymentSpace,
    binary_type: &BinaryType,
    channel: &PrimaryChannel,
    http_client: &Client,
) -> Result<DeploymentRecord, reqwest::Error> {
    let archive_url = get_archive_url(
        deployment_space,
        binary_type,
        channel,
        &RecordType::LatestDeploy,
    );

    let response: DeploymentRecord = http_client.get(archive_url).send().await?.json().await?;

    Ok(response)
}

/// Retrieves the deployment history for the specified parameters.
pub async fn get_deployment_history(
    deployment_space: &DeploymentSpace,
    binary_type: &BinaryType,
    channel: &PrimaryChannel,
    http_client: &Client,
) -> Result<DeployHistory, reqwest::Error> {
    let archive_url = get_archive_url(
        deployment_space,
        binary_type,
        channel,
        &RecordType::DeployHistory,
    );

    let response: DeployHistory = http_client.get(archive_url).send().await?.json().await?;

    Ok(response)
}

/// Generate an archive URL for the specified parameters.
pub fn get_archive_url(
    deployment_space: &DeploymentSpace,
    binary_type: &BinaryType,
    channel: &PrimaryChannel,
    record: &RecordType,
) -> String {
    format!(
        "https://raw.githubusercontent.com/grilme99/RobloxVersionArchive/main/{}/{}/{}/{}",
        deployment_space.to_string(),
        binary_type.to_string(),
        channel.to_string(),
        record.to_string()
    )
}

#[cfg(test)]
mod tests {
    use crate::prelude::{BinaryType, DeploymentSpace, PrimaryChannel, RecordType};

    use super::get_archive_url;

    #[test]
    fn archive_url_formatted_correctly() {
        let archive_url = get_archive_url(
            &DeploymentSpace::Global,
            &BinaryType::WindowsPlayer,
            &PrimaryChannel::Live,
            &RecordType::DeployHistory,
        );

        assert_eq!(archive_url, "https://raw.githubusercontent.com/grilme99/RobloxVersionArchive/main/Global/WindowsPlayer/LIVE/DeployHistory.json");
    }
}
