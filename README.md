# Praemunio Rust SDK

Rust client for the Praemunio public API.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
praemunio-sdk = "1.0.0"
```

## Quick start

```rust
use praemunio_sdk::client::PraemunioClient;

#[tokio::main]
async fn main() {
    let client = PraemunioClient::sandbox("pk_test_...", "sk_test_...");

    // Authenticate (blocking wrapper around async)
    client.authenticate().expect("auth failed");

    println!("Access token: {:?}", client.get_access_token());

    // Create a configured DefaultApi client
    let api = client.create_api();
    // use api to call endpoints...
}
```

## Environments

```rust
let sandbox = PraemunioClient::sandbox(public_key, secret_key);
let live = PraemunioClient::live(public_key, secret_key);
```

## Authentication

The SDK exchanges `public_key` and `secret_key` for an access token and refresh token via `POST /auth/auth-token`.

```rust
client.authenticate()?;
```

## Build

```bash
cargo build
cargo check
```

## Notes

- Generated from `sdks/shared/openapi/praemunio.openapi.json`
- Base URLs:
  - Sandbox: `https://sandbox-api.praemunio.org/api/v1`
  - Live: `https://api.praemunio.org/api/v1`
