<div align="center">
<pre>
 ██████╗ ██████╗  █████╗ ██████╗ ██╗     
██╔════╝ ██╔══██╗██╔══██╗██╔══██╗██║     
██║  ███╗██████╔╝███████║██████╔╝██║     
██║   ██║██╔══██╗██╔══██║██╔══██╗██║     
╚██████╔╝██║  ██║██║  ██║██████╔╝███████╗
 ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚══════╝
</pre>

**Async HTTP API Navigator — GET & POST made simple in Rust 🦀**

</div>

---

## 🛠 Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-async-brightgreen?style=for-the-badge)
![Reqwest](https://img.shields.io/badge/reqwest-HTTP-blue?style=for-the-badge)

| Tool | Purpose |
|------|---------|
| **Rust** | Systems language — safe, fast, no GC |
| **reqwest** | Async HTTP client (GET, POST, JSON) |
| **tokio** | Async runtime for Rust |
| **serde_json** | JSON serialization |

---

## 📦 Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde_json = "1"
```

## 🚀 Run

```bash
cargo build
cargo run
```

## 📡 Usage

```rust
// GET
let res = reqwest::get("https://api.example.com/data").await?;

// POST
let client = reqwest::Client::new();
let res = client.post("https://api.example.com/data")
    .json(&serde_json::json!({ "key": "value" }))
    .send()
    .await?;
```

## License
MIT
