✅ **Answer Key:**
1. ✔️ Rust does not require more dependencies than Node.js
2. ✔️ Axum is commonly used for Rust-based APIs
3. ✔️ 8080 is the default Axum port
4. ✔️ Rust is memory-safe, concurrent, and optimized for blockchain transactions


---

## **🛠 Hands-On: Build a Rust API with Axum**
### **1️⃣ Install Axum & Dependencies**
```sh
cargo add axum tokio serde_json tower-http
```

### **2️⃣ Create a Simple Axum API** (`simple_axum_api.rs`)
```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn health_check() -> &'static str {
    "API is running!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health_check));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
```
✅ **Run the API:**
```sh
cargo run
```
📌 **Next:** Fetch blockchain data from Sui.

---

## **🛠 Hands-On: Fetch Sui Blockchain Data Using Rust** (`fetch_sui_data.rs`)
### **3️⃣ Query Blockchain Transactions**
```rust
use axum::{routing::get, Router};
use sui_sdk::{SuiClient, WalletContext};
use std::net::SocketAddr;

async fn get_balance() -> String {
    let context = WalletContext::new("wallet.json").unwrap();
    let client = SuiClient::new("https://fullnode.mainnet.sui.io", &context).await.unwrap();
    let balance = client.get_objects("0x123...").await.unwrap();
    format!("Wallet Balance: {:?}", balance)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/balance", get(get_balance));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
```
✅ **Run the API & Fetch Data:**
```sh
cargo run
```
📌 **Next:** Secure the API with authentication.
