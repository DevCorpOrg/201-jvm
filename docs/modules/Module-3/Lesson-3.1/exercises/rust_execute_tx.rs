## **📄 File: `Module-3/Lesson-3.1/exercises/rust_execute_tx.rs`**
📌 **Exercise: Execute Sui Transactions Using Rust**

```rust
use sui_sdk::{SuiClient, WalletContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = WalletContext::new("wallet.json")?;
    let client = SuiClient::new("https://fullnode.mainnet.sui.io", &context).await?;
    
    let tx = client.execute_transaction("send_sui", vec!["0xRecipient", "100"]).await?;
    println!("Transaction Hash: {:?}", tx);
    Ok(())
}
```

✅ **Run the transaction:**
```sh
cargo run
```
Expected output:
```
Transaction Hash: 0xabcdef...
```

📌 **Next Module:** **Move Smart Contracts With Rust (Module 4)**

---