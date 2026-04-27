# vaultls

A fast CLI for listing and diffing secrets across HashiCorp Vault namespaces with colored output.

---

## Installation

**From source (requires Rust):**

```bash
cargo install vaultls
```

Or clone and build manually:

```bash
git clone https://github.com/yourname/vaultls && cd vaultls && cargo build --release
```

---

## Usage

Set your Vault address and token, then run:

```bash
export VAULT_ADDR="https://vault.example.com"
export VAULT_TOKEN="s.yourtoken"

# List secrets in a namespace
vaultls list secret/data/myapp

# Diff secrets between two namespaces
vaultls diff secret/data/staging secret/data/production
```

**Example output:**

```
  secret/data/myapp/
    ✔  DB_HOST
    ✔  DB_PASS
  + API_KEY        (only in staging)
  - LEGACY_TOKEN   (only in production)
```

Colored output highlights additions, removals, and matches at a glance.

---

## Requirements

- Rust 1.70+
- A running HashiCorp Vault instance
- `VAULT_ADDR` and `VAULT_TOKEN` environment variables

---

## License

This project is licensed under the [MIT License](LICENSE).