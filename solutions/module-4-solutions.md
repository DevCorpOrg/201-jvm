✅ **Answer Key:**
1. ✔️ `cargo-move` removes Node.js dependencies
2. ✔️ `cargo move publish` deploys Move contracts with Rust
3. ✔️ `sui.js` does support Move contracts, but Rust is preferred for security and performance

---

## **🛠 Hands-On: Deploying a Move Smart Contract Using Rust**
### **1️⃣ Install Move CLI & `cargo-move`**
```sh
sui move new my_contract
cargo install cargo-move
```
✅ **Verify Installation:**
```sh
sui move --version
cargo move --version
```

### **2️⃣ Write a Simple Move Contract** (`simple_contract.move`)
```move
module sui::example::MyContract {
    struct MyToken has key, store {
        value: u64
    }

    public entry fun mint(recipient: address, amount: u64) {
        let token = MyToken { value: amount };
        transfer::transfer(token, recipient);
    }
}
```
✅ **Compile the contract:**
```sh
sui move build
```

### **3️⃣ Deploy the Move Contract Using Rust (`deploy_contract.rs`)**
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
✅ **Run the Rust-based deployment script:**
```sh
cargo run
```
Expected output:
```
Contract Deployed at: 0xabcdef...
```
📌 **Next:** Move on to testing and executing transactions with Rust.