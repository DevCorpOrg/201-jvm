## **📄 File: Module-5/Lesson-5.1/build_axum_api.md**
📌 **Lesson: Building a REST API with Axum**

### **1️⃣ Install Axum & Dependencies**
```sh
cargo add axum tokio serde_json tower-http
```

### **2️⃣ Create a Simple Axum API**
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
📌 **Next Exercise:** Secure the API using JWT authentication.