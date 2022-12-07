use std::{env, path::PathBuf};

use anyhow::Context;

pub fn get_main_directory() -> anyhow::Result<PathBuf> {
    // Assuming we will always run the archiver via `cargo run`, the current directory will always be `archiver`.
    let current_dir = env::current_dir().context("Failed to get current dir")?;

    let main_dir = current_dir
        .parent()
        .context("Failed to get parent of current dir")?;

    Ok(main_dir.to_owned())
}
