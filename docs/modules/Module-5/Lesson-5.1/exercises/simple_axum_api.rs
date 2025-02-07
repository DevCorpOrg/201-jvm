## **📄 File: Module-5/Lesson-5.1/exercises/simple_axum_api.rs**
📌 **Exercise: Create a Simple API Using Axum**
```rust
use axum::{routing::get, Router};
use std::net::SocketAddr;

async fn status() -> &'static str {
    "Blockchain API is live!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/status", get(status));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
```
✅ **Run the API:**
```sh
cargo run
```
📌 **Next Exercise:** Secure API endpoints with JWT.
