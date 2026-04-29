use crate::diff::{diff_secret_keys, DiffEntry};

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> String {
        val.to_string()
    }

    #[test]
    fn test_diff_identical_keys() {
        let left = vec![s("api_key"), s("db_pass")];
        let right = vec![s("api_key"), s("db_pass")];
        let result = diff_secret_keys(&left, &right, "ns/left", "ns/right");
        assert!(!result.has_differences());
        assert!(result.entries.iter().all(|e| matches!(e, DiffEntry::InBoth(_))));
    }

    #[test]
    fn test_diff_only_in_left() {
        let left = vec![s("secret_a"), s("shared")];
        let right = vec![s("shared")];
        let result = diff_secret_keys(&left, &right, "ns/left", "ns/right");
        assert!(result.has_differences());
        assert!(result.entries.iter().any(|e| matches!(e, DiffEntry::OnlyInLeft(k) if k == "secret_a")));
        assert!(result.entries.iter().any(|e| matches!(e, DiffEntry::InBoth(k) if k == "shared")));
    }

    #[test]
    fn test_diff_only_in_right() {
        let left = vec![s("shared")];
        let right = vec![s("shared"), s("new_secret")];
        let result = diff_secret_keys(&left, &right, "ns/left", "ns/right");
        assert!(result.has_differences());
        assert!(result.entries.iter().any(|e| matches!(e, DiffEntry::OnlyInRight(k) if k == "new_secret")));
    }

    #[test]
    fn test_diff_completely_different() {
        let left = vec![s("alpha"), s("beta")];
        let right = vec![s("gamma"), s("delta")];
        let result = diff_secret_keys(&left, &right, "ns/left", "ns/right");
        assert!(result.has_differences());
        assert_eq!(result.entries.len(), 4);
    }

    #[test]
    fn test_diff_empty_both() {
        let result = diff_secret_keys(&[], &[], "ns/left", "ns/right");
        assert!(!result.has_differences());
        assert!(result.entries.is_empty());
    }

    #[test]
    fn test_diff_result_sorted() {
        let left = vec![s("zebra"), s("apple")];
        let right = vec![s("mango"), s("apple")];
        let result = diff_secret_keys(&left, &right, "ns/left", "ns/right");
        let keys: Vec<&str> = result.entries.iter().map(|e| match e {
            crate::diff::DiffEntry::OnlyInLeft(k) => k.as_str(),
            crate::diff::DiffEntry::OnlyInRight(k) => k.as_str(),
            crate::diff::DiffEntry::InBoth(k) => k.as_str(),
        }).collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}
