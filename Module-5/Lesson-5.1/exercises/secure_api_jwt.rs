## **📄 File: Module-5/Lesson-5.1/exercises/secure_api_jwt.rs**
📌 **Exercise: Secure API with JWT Authentication**
```rust
use axum::{extract::TypedHeader, routing::get, Router};
use axum::headers::Authorization;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::net::SocketAddr;

async fn protected_route(TypedHeader(auth): TypedHeader<Authorization<String>>) -> &'static str {
    let key = DecodingKey::from_secret(b"my-secret-key");
    let token = auth.token();
    let validation = Validation::new(Algorithm::HS256);
    match decode::<serde_json::Value>(token, &key, &validation) {
        Ok(_) => "Access Granted!",
        Err(_) => "Invalid Token",
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/secure", get(protected_route));
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Secure API running on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
```
✅ **Run the Secure API:**
```sh
cargo run
```
📌 **Next Module:** **Deploying Rust-Based dApps with Yew (Module 6)**