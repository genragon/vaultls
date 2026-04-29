use crate::list::{print_keys, run_list, ListOptions};
use crate::vault::VaultClient;

// Unit tests for list module logic (non-network parts)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_keys_no_panic_empty() {
        // Should not panic on empty input
        print_keys(&[], None);
        print_keys(&[], Some("test/ns"));
    }

    #[test]
    fn test_print_keys_with_values() {
        let keys = vec![
            "secret/data/app/db".to_string(),
            "secret/data/app/".to_string(),
            "secret/data/app/api".to_string(),
        ];
        // Should not panic with valid input
        print_keys(&keys, Some("team-a"));
    }

    #[test]
    fn test_list_options_fields() {
        let opts = ListOptions {
            path: "secret/data",
            namespace: Some("my-ns"),
            recursive: true,
        };
        assert_eq!(opts.path, "secret/data");
        assert_eq!(opts.namespace, Some("my-ns"));
        assert!(opts.recursive);
    }

    #[test]
    fn test_list_options_no_namespace() {
        let opts = ListOptions {
            path: "kv/",
            namespace: None,
            recursive: false,
        };
        assert!(opts.namespace.is_none());
        assert!(!opts.recursive);
    }
}
