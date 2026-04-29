use crate::output::{OutputFormat, OutputWriter};
use crate::render::{render_diff, render_secrets};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_writer(fmt: &str) -> OutputWriter {
        OutputWriter::new(OutputFormat::from_str(fmt))
    }

    #[test]
    fn test_output_format_from_str() {
        matches!(OutputFormat::from_str("json"), OutputFormat::Json);
        matches!(OutputFormat::from_str("plain"), OutputFormat::Plain);
        matches!(OutputFormat::from_str("pretty"), OutputFormat::Pretty);
        matches!(OutputFormat::from_str("unknown"), OutputFormat::Pretty);
    }

    #[test]
    fn test_render_secrets_empty() {
        let writer = make_writer("plain");
        let secrets = HashMap::new();
        // Should not panic
        render_secrets(&writer, "secret/foo", &secrets);
    }

    #[test]
    fn test_render_secrets_with_values() {
        let writer = make_writer("plain");
        let mut secrets = HashMap::new();
        secrets.insert("API_KEY".to_string(), "abc123".to_string());
        secrets.insert("DB_PASS".to_string(), "secret".to_string());
        // Should not panic
        render_secrets(&writer, "secret/myapp", &secrets);
    }

    #[test]
    fn test_render_diff_no_changes() {
        let writer = make_writer("plain");
        let added = HashMap::new();
        let removed = HashMap::new();
        let changed = HashMap::new();
        render_diff(&writer, "secret/a", "secret/b", &added, &removed, &changed);
    }

    #[test]
    fn test_render_diff_with_changes() {
        let writer = make_writer("plain");
        let mut added = HashMap::new();
        added.insert("NEW_KEY".to_string(), "newval".to_string());
        let mut removed = HashMap::new();
        removed.insert("OLD_KEY".to_string(), "oldval".to_string());
        let mut changed = HashMap::new();
        changed.insert("CHANGED".to_string(), ("before".to_string(), "after".to_string()));
        render_diff(&writer, "secret/a", "secret/b", &added, &removed, &changed);
    }

    #[test]
    fn test_render_secrets_json_format() {
        let writer = make_writer("json");
        let mut secrets = HashMap::new();
        secrets.insert("FOO".to_string(), "bar".to_string());
        // Should not panic
        render_secrets(&writer, "secret/test", &secrets);
    }
}
