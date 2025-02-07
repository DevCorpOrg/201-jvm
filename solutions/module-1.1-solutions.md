

✅ **Answer Key:**
1. ✔️ Rust uses ownership rules instead of garbage collection.
2. ✔️ Move is resource-oriented and prevents asset duplication.
3. ✔️ `sui move build` compiles Move contracts.

---

## **🛠 Hands-On: Install Rust & Sui CLI**
### **1️⃣ Install Rust**
Run the following command:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
✅ **Verify Installation:**
```sh
rustc --version
cargo --version
```
Expected Output:
```
rustc 1.x.x
cargo 1.x.x
```

### **2️⃣ Install Sui CLI**
```sh
curl -fsSL https://install.sui.io | sh
```
✅ **Verify Installation:**
```sh
sui --version
```

---

## **🛠 Hands-On: First Rust Program** (`hello_sui.rs`)
### **Write & Run a Simple Rust Program**
```rust
fn main() {
    println!("🚀 Hello, Sui Blockchain!");
}
```
✅ **Run the program:**
```sh
cargo run
```
Expected Output:
```
🚀 Hello, Sui Blockchain!
```

---

## **🛠 Hands-On: First Move Smart Contract** (`move_basics.move`)
### **Write & Compile a Move Smart Contract**
```move
module sui::example::HelloSui {
    use sui::transfer;
    
    public entry fun hello() {
        let message = b"Hello, Sui!";
        transfer::emit_event(message);
    }
}
```
✅ **Compile Move Contract:**
```sh
sui move build