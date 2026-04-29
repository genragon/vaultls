use serde_json::{json, Value};
use std::collections::HashMap;

/// Serialize a flat key-value map to a JSON string.
pub fn secrets_to_json(path: &str, secrets: &HashMap<String, String>) -> String {
    let obj: Value = json!({
        "path": path,
        "secrets": secrets
    });
    serde_json::to_string_pretty(&obj).unwrap_or_else(|_| "{}".to_string())
}

/// Serialize a diff result to JSON.
pub fn diff_to_json(
    path_a: &str,
    path_b: &str,
    added: &HashMap<String, String>,
    removed: &HashMap<String, String>,
    changed: &HashMap<String, (String, String)>,
) -> String {
    let changed_obj: HashMap<String, Value> = changed
        .iter()
        .map(|(k, (old, new))| {
            (
                k.clone(),
                json!({ "old": old, "new": new }),
            )
        })
        .collect();

    let obj = json!({
        "path_a": path_a,
        "path_b": path_b,
        "added": added,
        "removed": removed,
        "changed": changed_obj,
    });

    serde_json::to_string_pretty(&obj).unwrap_or_else(|_| "{}".to_string())
}

/// Format a path for display, normalizing trailing slashes.
pub fn normalize_path(path: &str) -> String {
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("secret/data/foo/"), "secret/data/foo");
        assert_eq!(normalize_path("secret/data/foo"), "secret/data/foo");
        assert_eq!(normalize_path("/"), "/");
        assert_eq!(normalize_path(""), "/");
    }

    #[test]
    fn test_secrets_to_json() {
        let mut map = HashMap::new();
        map.insert("KEY".to_string(), "value".to_string());
        let result = secrets_to_json("secret/foo", &map);
        assert!(result.contains("secret/foo"));
        assert!(result.contains("KEY"));
    }
}
