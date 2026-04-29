use crate::config::Config;
use crate::diff::diff_secret_keys;
use crate::vault::VaultClient;
use anyhow::{Context, Result};

pub async fn run_diff(
    client: &VaultClient,
    config: &Config,
    left_path: &str,
    right_path: &str,
    show_common: bool,
) -> Result<()> {
    let left_ns = config
        .namespace
        .as_deref()
        .unwrap_or("default");

    println!("Fetching keys from: {}", left_path);
    let left_keys = client
        .list_secrets(left_path)
        .await
        .with_context(|| format!("Failed to list secrets at '{}'", left_path))?;

    println!("Fetching keys from: {}", right_path);
    let right_keys = client
        .list_secrets(right_path)
        .await
        .with_context(|| format!("Failed to list secrets at '{}'", right_path))?;

    let result = diff_secret_keys(&left_keys, &right_keys, left_path, right_path);

    if !show_common {
        let filtered_entries = result
            .entries
            .into_iter()
            .filter(|e| !matches!(e, crate::diff::DiffEntry::InBoth(_)))
            .collect();
        let filtered_result = crate::diff::DiffResult {
            entries: filtered_entries,
            left_ns: result.left_ns,
            right_ns: result.right_ns,
        };
        filtered_result.print();
    } else {
        result.print();
    }

    Ok(())
}
