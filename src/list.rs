use crate::vault::VaultClient;
use anyhow::Result;
use colored::Colorize;

pub struct ListOptions<'a> {
    pub path: &'a str,
    pub namespace: Option<&'a str>,
    pub recursive: bool,
}

pub fn run_list(client: &VaultClient, opts: &ListOptions) -> Result<Vec<String>> {
    let keys = client.list_secrets(opts.path, opts.namespace)?;
    let mut all_keys = Vec::new();

    for key in &keys {
        let full_path = format!("{}/{}", opts.path.trim_end_matches('/'), key);
        if key.ends_with('/') && opts.recursive {
            let sub_opts = ListOptions {
                path: &full_path,
                namespace: opts.namespace,
                recursive: true,
            };
            let sub_keys = run_list(client, &sub_opts)?;
            all_keys.extend(sub_keys);
        } else {
            all_keys.push(full_path);
        }
    }

    Ok(all_keys)
}

pub fn print_keys(keys: &[String], namespace: Option<&str>) {
    let ns_label = namespace.unwrap_or("root");
    println!(
        "{} {}",
        "Namespace:".bold().cyan(),
        ns_label.yellow()
    );
    println!("{}", "─".repeat(40).dimmed());
    for key in keys {
        if key.ends_with('/') {
            println!("  {}", key.blue().bold());
        } else {
            println!("  {}", key.green());
        }
    }
    println!();
}
