✅ **Answer Key:**
1. ✔️ Rust avoids Node.js dependencies
2. ✔️ `client.get_objects()` fetches wallet balance
3. ✔️ `cargo install sui-sdk` installs Rust’s Sui SDK
4. ✔️ Rust prevents memory leaks and is faster than JavaScript

---


📌 **Next:** Install `sui-sdk-rs` and interact with Sui blockchain using Rust.

---

## **🛠 Hands-On: Install `sui-sdk-rs` & Query Blockchain Data**
### **1️⃣ Install Rust-Based Sui SDK (`sui-sdk-rs`)**
```sh
cargo install sui-sdk
```

### **2️⃣ Verify Installation**
```sh
cargo --version
sui --version
```
✅ If installed correctly, you should see Rust and Sui CLI versions.

### **3️⃣ Query Blockchain Wallet Balance Using Rust** (`rust_query_balance.rs`)
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
📌 **Next:** Execute a transaction using Rust.

---

## **🛠 Hands-On: Execute Sui Transactions with Rust** (`rust_execute_tx.rs`)
### **4️⃣ Execute a Sui Blockchain Transaction in Rust**
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