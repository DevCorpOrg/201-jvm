## **📂 Module 3: Sui SDKs Without Node.js**
```
Module-3/
│── Lesson-3.1/              
│   ├── sui_sdk_vs_sui_js.md   # Why replace Node.js SDKs?
│   ├── install_sui_sdk.md     # Installing Rust-based Sui SDK
│   ├── exercises/           
│   │   ├── rust_query_balance.rs  # Query wallet balance using Rust
│   │   ├── rust_execute_tx.rs     # Execute transactions using Rust
```

---

## **📄 File: `Module-3/Lesson-3.1/sui_sdk_vs_sui_js.md`**
📌 **Lesson: Why Replace `sui.js` with Rust?**

### **Why Do Blockchain SDKs Use Node.js?**
- The **JavaScript SDK (`sui.js`)** allows developers to interact with **Sui blockchain**.
- It is built for **dApps**, requiring **Node.js** to execute scripts.
- Many **frontend-heavy blockchain projects** rely on `sui.js`.

### **Problems with `sui.js` and Node.js**
❌ **Performance issues** – JavaScript is **single-threaded** and not ideal for high-performance blockchain operations.  
❌ **Security risks** – Node.js introduces **dependency vulnerabilities**.  
❌ **Unnecessary complexity** – Installing and managing **Node.js** is **not needed** for backend blockchain services.

### **Why Rust is Better for Blockchain?**
✅ **Faster Execution** – Rust compiles to **native code**, bypassing Node.js overhead.  
✅ **Memory-Safe** – No garbage collection issues like in JavaScript.  
✅ **Direct Blockchain Integration** – Rust SDK (`sui-sdk-rs`) provides **native blockchain interaction**.  

📌 **Next Lesson:** Installing the **Rust-based Sui SDK**.

---

