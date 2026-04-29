use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct VaultListResponse {
    data: VaultListData,
}

#[derive(Debug, Deserialize)]
struct VaultListData {
    keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct VaultReadResponse {
    data: HashMap<String, serde_json::Value>,
}

pub struct VaultClient {
    client: Client,
    base_url: String,
    token: String,
}

impl VaultClient {
    pub fn new(base_url: &str, token: &str) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            token: token.to_string(),
        })
    }

    pub fn list_secrets(&self, path: &str, namespace: Option<&str>) -> Result<Vec<String>> {
        let url = format!("{}/v1/{}", self.base_url, path.trim_start_matches('/'));
        let mut req = self
            .client
            .get(&url)
            .query(&[("list", "true")])
            .header("X-Vault-Token", &self.token);

        if let Some(ns) = namespace {
            req = req.header("X-Vault-Namespace", ns);
        }

        let resp = req.send().context("Failed to send request to Vault")?;
        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("Vault returned status {}: {}", status, path);
        }

        let body: VaultListResponse = resp.json().context("Failed to parse Vault list response")?;
        Ok(body.data.keys)
    }

    pub fn read_secret(
        &self,
        path: &str,
        namespace: Option<&str>,
    ) -> Result<HashMap<String, serde_json::Value>> {
        let url = format!("{}/v1/{}", self.base_url, path.trim_start_matches('/'));
        let mut req = self
            .client
            .get(&url)
            .header("X-Vault-Token", &self.token);

        if let Some(ns) = namespace {
            req = req.header("X-Vault-Namespace", ns);
        }

        let resp = req.send().context("Failed to send request to Vault")?;
        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("Vault returned status {}: {}", status, path);
        }

        let body: VaultReadResponse = resp.json().context("Failed to parse Vault read response")?;
        Ok(body.data)
    }
}
