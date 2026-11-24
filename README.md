# Praemunio Rust SDK

> Client officiel pour l'API Praemunio — Rust

## 📦 Installation

Ajoutez à votre `Cargo.toml` :

```toml
[dependencies]
praemunio-sdk = "1.0"
```

Ou avec Cargo :

```bash
cargo add praemunio-sdk
```

## 🚀 Quick Start (5 minutes)

### 1. Obtenir vos clés API

Récupérez vos clés depuis le [dashboard Praemunio](https://dashboard.praemunio.io) :
- **Public Key** : `pk_test_...` (sandbox) ou `pk_live_...` (production)
- **Secret Key** : `sk_test_...` (sandbox) ou `sk_live_...` (production)

### 2. Initialiser le client

```rust
use praemunio_sdk::PraemunioClient;

// Mode sandbox (tests)
let client = PraemunioClient::sandbox("pk_test_xxx", "sk_test_xxx");

// Mode production
// let client = PraemunioClient::live("pk_live_xxx", "sk_live_xxx");
```

### 3. S'authentifier

```rust
client.authenticate()?;

println!("Access token: {:?}", client.get_access_token());
println!("Refresh token: {:?}", client.get_refresh_token());
```

### 4. Premier appel API

```rust
let api = client.create_api();

// Lister les workspaces
let result = api.workspaces_drive_get().await?;
println!("Workspaces: {:?}", result);
```

### 5. Exemple complet

```rust
use praemunio_sdk::PraemunioClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialiser le client (sandbox)
    let client = PraemunioClient::sandbox("pk_test_xxx", "sk_test_xxx");

    // Authentification
    client.authenticate()?;
    println!("Authentifié avec succès !");

    // Premier appel API
    let api = client.create_api();
    let workspaces = api.workspaces_drive_get().await?;

    println!("Workspaces: {:?}", workspaces);

    Ok(())
}
```

## 🔐 Authentification

Le SDK gère automatiquement l'authentification et le refresh des tokens :

```rust
// Authentification initiale
client.authenticate()?;

// Le token est renouvelé automatiquement lors des appels API
// Vous pouvez aussi forcer le refresh :
client.refresh()?;
```

## 🌍 Environnements

| Environnement | URL | Utilisation |
|---------------|-----|-------------|
| **Sandbox** | `https://sandbox-api.praemunio.org/api/v1` | Tests et développement |
| **Live** | `https://api.praemunio.org/api/v1` | Production |

```rust
// Sandbox (par défaut)
let client = PraemunioClient::sandbox(public_key, secret_key);

// Production
let client = PraemunioClient::live(public_key, secret_key);
```

## 📚 Documentation complète

- [Documentation officielle Rust](https://docs.praemunio.io/sdk/rust)
- [Référence API](https://docs.praemunio.io/api-reference)
- [Guide d'authentification](https://docs.praemunio.io/authentication)
- [Exemples](https://docs.praemunio.io/sdk/rust/examples)

## 📋 Endpoints principaux

| Méthode | Endpoint | Description |
|---------|----------|-------------|
| `POST` | `/auth/auth-token` | Authentification |
| `GET` | `/workspaces/drive` | Lister les workspaces |
| `POST` | `/teams/create` | Créer une équipe |
| `GET` | `/folders/{id}` | Détails d'un dossier |
| `POST` | `/documents/archive` | Archiver un document |

## 🤝 Support

- 📖 [Documentation](https://docs.praemunio.io)
- 🐛 [Signaler un bug](https://github.com/praemunio/sdks/issues)
- 💬 [Discord](https://discord.gg/praemunio)
- 📧 [Email](mailto:support@praemunio.org)