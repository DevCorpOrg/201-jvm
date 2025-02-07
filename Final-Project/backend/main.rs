## **📄 File: `Final-Project/backend/main.rs`**
📌 **Rust API using Axum**
```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn get_balance() -> &'static str {
    "Balance: 1000 SUI"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/balance", get(get_balance));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Backend running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
```
✅ **Run the backend API:**
```sh
cargo run
```
📌 **Next Step:** Deploy the Move smart contract.
