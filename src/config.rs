use anyhow::{Context, Result};
use std::env;

/// Runtime configuration loaded from environment variables or CLI flags.
#[derive(Debug, Clone)]
pub struct Config {
    /// Base URL of the Vault server, e.g. `https://vault.example.com`
    pub vault_addr: String,
    /// Vault token used for authentication
    pub vault_token: String,
    /// Optional namespace prefix (Vault Enterprise)
    pub vault_namespace: Option<String>,
}

impl Config {
    /// Build a [`Config`] from environment variables.
    ///
    /// Required vars: `VAULT_ADDR`, `VAULT_TOKEN`
    /// Optional vars: `VAULT_NAMESPACE`
    pub fn from_env() -> Result<Self> {
        // Load a `.env` file if present (best-effort)
        let _ = dotenvy::dotenv();

        let vault_addr = env::var("VAULT_ADDR")
            .context("VAULT_ADDR environment variable is not set")?;
        let vault_token = env::var("VAULT_TOKEN")
            .context("VAULT_TOKEN environment variable is not set")?;
        let vault_namespace = env::var("VAULT_NAMESPACE").ok();

        Ok(Self {
            vault_addr: vault_addr.trim_end_matches('/').to_owned(),
            vault_token,
            vault_namespace,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_vault_addr_returns_error() {
        // Ensure vars are absent for this test
        env::remove_var("VAULT_ADDR");
        env::remove_var("VAULT_TOKEN");
        let result = Config::from_env();
        assert!(result.is_err());
        let msg = format!("{}", result.unwrap_err());
        assert!(msg.contains("VAULT_ADDR"));
    }

    #[test]
    fn trailing_slash_is_stripped() {
        env::set_var("VAULT_ADDR", "https://vault.example.com/");
        env::set_var("VAULT_TOKEN", "s.test");
        env::remove_var("VAULT_NAMESPACE");
        let cfg = Config::from_env().unwrap();
        assert_eq!(cfg.vault_addr, "https://vault.example.com");
        // cleanup
        env::remove_var("VAULT_ADDR");
        env::remove_var("VAULT_TOKEN");
    }

    #[test]
    fn namespace_is_optional() {
        env::set_var("VAULT_ADDR", "https://vault.example.com");
        env::set_var("VAULT_TOKEN", "s.test");
        env::remove_var("VAULT_NAMESPACE");
        let cfg = Config::from_env().unwrap();
        assert!(cfg.vault_namespace.is_none());
        env::remove_var("VAULT_ADDR");
        env::remove_var("VAULT_TOKEN");
    }
}
