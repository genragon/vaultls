use crate::format::{diff_to_json, normalize_path, secrets_to_json};
use crate::output::{OutputFormat, OutputWriter};
use std::collections::HashMap;

/// Render a list of secrets for a given path.
pub fn render_secrets(
    writer: &OutputWriter,
    path: &str,
    secrets: &HashMap<String, String>,
) {
    let display_path = normalize_path(path);

    match writer.format {
        OutputFormat::Json => {
            println!("{}", secrets_to_json(&display_path, secrets));
            return;
        }
        _ => {}
    }

    writer.print_header(&format!("Secrets at {}", display_path));

    if secrets.is_empty() {
        writer.print_info("No secrets found.");
        return;
    }

    let mut keys: Vec<&String> = secrets.keys().collect();
    keys.sort();

    for key in keys {
        let value = &secrets[key];
        writer.print_key_value(key, value);
    }
}

/// Render a diff between two secret paths.
pub fn render_diff(
    writer: &OutputWriter,
    path_a: &str,
    path_b: &str,
    added: &HashMap<String, String>,
    removed: &HashMap<String, String>,
    changed: &HashMap<String, (String, String)>,
) {
    let pa = normalize_path(path_a);
    let pb = normalize_path(path_b);

    match writer.format {
        OutputFormat::Json => {
            println!("{}", diff_to_json(&pa, &pb, added, removed, changed));
            return;
        }
        _ => {}
    }

    writer.print_header(&format!("Diff: {} vs {}", pa, pb));

    if added.is_empty() && removed.is_empty() && changed.is_empty() {
        writer.print_info("No differences found.");
        return;
    }

    let mut added_keys: Vec<&String> = added.keys().collect();
    added_keys.sort();
    for key in added_keys {
        writer.print_added(key, &added[key]);
    }

    let mut removed_keys: Vec<&String> = removed.keys().collect();
    removed_keys.sort();
    for key in removed_keys {
        writer.print_removed(key, &removed[key]);
    }

    let mut changed_keys: Vec<&String> = changed.keys().collect();
    changed_keys.sort();
    for key in changed_keys {
        let (old, new) = &changed[key];
        writer.print_changed(key, old, new);
    }
}
