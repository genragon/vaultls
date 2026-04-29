use std::collections::{HashMap, HashSet};
use colored::*;

#[derive(Debug, PartialEq)]
pub enum DiffEntry {
    OnlyInLeft(String),
    OnlyInRight(String),
    InBoth(String),
}

pub struct DiffResult {
    pub entries: Vec<DiffEntry>,
    pub left_ns: String,
    pub right_ns: String,
}

impl DiffResult {
    pub fn print(&self) {
        println!(
            "Diff: {} <-> {}",
            self.left_ns.cyan().bold(),
            self.right_ns.cyan().bold()
        );
        println!("{}", "-".repeat(50).dimmed());

        for entry in &self.entries {
            match entry {
                DiffEntry::OnlyInLeft(key) => {
                    println!("{} {}", "-".red().bold(), key.red());
                }
                DiffEntry::OnlyInRight(key) => {
                    println!("{} {}", "+".green().bold(), key.green());
                }
                DiffEntry::InBoth(key) => {
                    println!("{} {}", "=".dimmed(), key.dimmed());
                }
            }
        }
    }

    pub fn has_differences(&self) -> bool {
        self.entries.iter().any(|e| !matches!(e, DiffEntry::InBoth(_)))
    }
}

pub fn diff_secret_keys(
    left_keys: &[String],
    right_keys: &[String],
    left_ns: &str,
    right_ns: &str,
) -> DiffResult {
    let left_set: HashSet<&String> = left_keys.iter().collect();
    let right_set: HashSet<&String> = right_keys.iter().collect();

    let mut all_keys: Vec<&String> = left_set.union(&right_set).collect();
    all_keys.sort();

    let entries = all_keys
        .into_iter()
        .map(|key| {
            let in_left = left_set.contains(key);
            let in_right = right_set.contains(key);
            match (in_left, in_right) {
                (true, false) => DiffEntry::OnlyInLeft(key.clone()),
                (false, true) => DiffEntry::OnlyInRight(key.clone()),
                _ => DiffEntry::InBoth(key.clone()),
            }
        })
        .collect();

    DiffResult {
        entries,
        left_ns: left_ns.to_string(),
        right_ns: right_ns.to_string(),
    }
}
