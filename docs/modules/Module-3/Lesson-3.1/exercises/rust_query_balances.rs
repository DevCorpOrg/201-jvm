## **📄 File: `Module-3/Lesson-3.1/exercises/rust_query_balance.rs`**
📌 **Exercise: Query Wallet Balance Using Rust**

```rust
use sui_sdk::{SuiClient, WalletContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = WalletContext::new("wallet.json")?;
    let client = SuiClient::new("https://fullnode.mainnet.sui.io", &context).await?;
    
    let balance = client.get_objects("0x123...").await?;
    println!("Wallet Balance: {:?}", balance);
    Ok(())
}
```

✅ **Run the script:**
```sh
cargo run
```
Expected output:
```
Wallet Balance: 1000 SUI
```

📌 **Next Exercise:** Execute a Sui transaction using Rust.

---

