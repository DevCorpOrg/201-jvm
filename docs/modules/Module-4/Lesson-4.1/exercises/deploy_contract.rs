## **📄 File: Module-4/Lesson-4.1/exercises/deploy_contract.rs**
📌 **Exercise: Deploy Move Contract Using Rust**
```rust
use sui_sdk::{SuiClient, WalletContext};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = WalletContext::new("wallet.json")?;
    let client = SuiClient::new("https://fullnode.mainnet.sui.io", &context).await?;
    
    let package_id = client.publish_package("./move_contract").await?;
    println!("Contract Deployed at: {:?}", package_id);
    Ok(())
}
```
✅ **Run the deployment:**
```sh
cargo run
```
Expected output:
```
Contract Deployed at: 0xabcdef...
```

📌 **Next Module:** **Rust-Based APIs for Blockchain (Module 5)**