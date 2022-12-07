use std::{fs, path::Path};

use anyhow::Context;
use archiver::{
    http::{get_deployment_info, LiveDeploymentInfo},
    utils::get_main_directory,
};
use env_logger::Env;
use reqwest::Client;
use roblox_version_archive::prelude::{
    DeployHistory, DeploymentRecord, DeploymentSpace, BINARY_TYPES, PRIMARY_CHANNELS,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let client = Client::new();

    check_deployment_space(&DeploymentSpace::Global, &client)
        .await
        .context("Failed to update archive for deployment space Global")?;

    check_deployment_space(&DeploymentSpace::Luobu, &client)
        .await
        .context("Failed to update archive for deployment space Luobu")?;

    Ok(())
}

async fn check_deployment_space(
    deployment_space: &DeploymentSpace,
    client: &Client,
) -> anyhow::Result<()> {
    log::info!("Checking deployment space {}", deployment_space.to_string());
    let base_url = deployment_space.get_client_version_url();

    let main_dir = get_main_directory().context("Failed to get main directory")?;

    let archive_space_path = main_dir.join(deployment_space.to_string());
    create_dir_at_path_if_new(&archive_space_path)
        .context("Failed to create path for archive space")?;

    // Loop through each binary type and get the latest deploy info for each channel
    for binary_type in BINARY_TYPES {
        let binary_type_path = archive_space_path.join(binary_type.to_string());
        create_dir_at_path_if_new(&binary_type_path)
            .context("Failed to create path for binary type")?;

        // Loop through each channel and get latest deploy info
        for channel_name in PRIMARY_CHANNELS {
            let channel_path = binary_type_path.join(channel_name.to_string());
            create_dir_at_path_if_new(&channel_path)
                .context("Failed to create path for channel")?;

            let deploy_history_path = channel_path.join("DeployHistory.json");
            if !deploy_history_path.exists() {
                fs::write(&deploy_history_path, b"{}\n").context(format!(
                    "Failed to create an empty DeployHistory at {deploy_history_path:?}"
                ))?
            }

            log::info!(
                "Getting deployment info for {} {} at channel {}",
                deployment_space.to_string(),
                binary_type.to_string(),
                channel_name.to_string()
            );

            let latest_deployment =
                get_deployment_info(client, &base_url, &binary_type, &channel_name)
                    .await
                    .context("Failed to get latest deployment")?;

            let latest_deployment_record = get_deployment_record(latest_deployment)
                .context("Failed to parse latest deployment into savable record")?;

            let serialized_latest_deploy = serde_json::to_string_pretty(&latest_deployment_record)
                .context("Failed to convert latest deployment record to string")?;

            let records_content = fs::read_to_string(&deploy_history_path)
                .context(format!("Failed to read {deploy_history_path:?}"))?;

            let mut records: DeployHistory = serde_json::from_str(&records_content)
                .context("Failed to parse records from disk")?;

            if records.contains_key(&latest_deployment_record.change_list) {
                log::info!("Deployment already saved to disk");
                continue;
            }

            records.insert(
                latest_deployment_record.change_list,
                latest_deployment_record,
            );

            let serialized_records =
                serde_json::to_string(&records).context("Failed to convert records to string")?;

            fs::write(&deploy_history_path, &serialized_records)
                .context("Failed to write deploy history to path")?;

            let latest_deploy_path = channel_path.join("LatestDeploy.json");
            fs::write(&latest_deploy_path, &serialized_latest_deploy)
                .context("Failed to write latest deploy to path")?;
        }
    }

    Ok(())
}

fn create_dir_at_path_if_new(path: &Path) -> anyhow::Result<()> {
    if !path.exists() {
        fs::create_dir(&path).context(format!("Failed to create path at {path:?}"))?;
    }

    Ok(())
}

fn get_deployment_record(record: LiveDeploymentInfo) -> anyhow::Result<DeploymentRecord> {
    let mut split_version = record.version.split(".");
    let major_rev = split_version.next().context("No major_rev in version")?;
    let version = split_version.next().context("No version in version")?;
    let patch = split_version.next().context("No patch in version")?;
    let change_list = split_version.next().context("No change_list in version")?;

    Ok(DeploymentRecord {
        client_version: record.client_version_upload,
        bootstrapper_version: record.bootstrapper_version,

        major_rev: major_rev.parse().context("Failed to parse major_rev")?,
        version: version.parse().context("Failed to parse version")?,
        patch: patch.parse().context("Failed to parse patch")?,
        change_list: change_list.parse().context("Failed to parse change_list")?,
    })
}
