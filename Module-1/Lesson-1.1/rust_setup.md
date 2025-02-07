### **📄 File: Module-1/Lesson-1.1/rust_setup.md**  
#### **📌 Setting Up Rust & Sui CLI**  

### **1️⃣ Install Rust**  
Rust is needed to interact with the **Sui blockchain**.  
**Run the following command to install Rust:**
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
After installation, restart your terminal and verify:
```sh
rustc --version
cargo --version
```
✅ If Rust is installed, you should see version numbers.

---

### **2️⃣ Install Sui CLI**  
We need **Sui CLI** to deploy Move smart contracts and interact with the blockchain.

**Run:**
```sh
curl -fsSL https://install.sui.io | sh
```
After installation, verify:
```sh
sui --version
```

---

### **3️⃣ Set Up Local Sui Testnet**
We can test our Move contracts on a **local blockchain instance**.

**Start a Local Sui Validator:**
```sh
sui start
```

📌 **Next Lesson**: Writing our first **Rust program** in `hello_sui.rs`.

---
