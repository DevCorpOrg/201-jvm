# Module 3: Why Sui Blockchain SDKs Depend on Node.js

## 📌 Overview
This module explores why **Sui blockchain SDKs** often rely on **Node.js**, and how we can replace `sui.js` with **Rust-based alternatives (`sui-sdk-rs`)** to improve security, performance, and maintainability.

---

## **📖 Concept: Why `sui.js` Depends on Node.js & Rust-Based Alternatives**
### **Why Do Blockchain SDKs Use Node.js?**
- **JavaScript is widely adopted** in web development.
- Many **blockchain dApps use JavaScript for frontend interactions**.
- `sui.js` is built with **Node.js dependencies** for:
  - Wallet transactions
  - Smart contract deployment
  - Blockchain queries

### **Problems with `sui.js` & Node.js**
❌ **Performance Issues:** Node.js is single-threaded, which is inefficient for blockchain backends.
❌ **Security Concerns:** JavaScript lacks strict memory safety and is prone to supply chain attacks via `npm`.
❌ **Unnecessary Overhead:** Many backend applications do not require JavaScript, yet Node.js is still used.

### **Rust-Based Alternatives (`sui-sdk-rs`)**
✅ **High Performance:** Rust compiles to native binaries, making blockchain interactions faster.
✅ **Memory Safety:** Rust enforces strict ownership models, preventing memory leaks.
✅ **Better Integration:** Rust supports **asynchronous blockchain transactions** without Node.js dependencies.

---

## **📝 Quiz: Why Replace `sui.js` with Rust?**
### **Question 1:**
Why is `sui-sdk-rs` preferred over `sui.js`?
- [ ] Rust is more memory-safe and avoids Node.js dependencies
- [ ] JavaScript executes faster
- [ ] Node.js offers better security
- [ ] Rust doesn’t support async operations

### **Question 2:**
Which function in `sui-sdk-rs` is used to fetch a wallet balance?
- [ ] `client.getTransactionHistory()`
- [ ] `client.get_objects()`
- [ ] `client.fetchSui()`
- [ ] `client.balanceCheck()`

### **Question 3:**
Which command installs `sui-sdk-rs`?
- [ ] `npm install sui-sdk`
- [ ] `cargo install sui-sdk`
- [ ] `rustup install sui-sdk`
- [ ] `brew install sui-sdk`

### **Question 4:**
What is a key advantage of using Rust over Node.js for blockchain applications?
- [ ] Rust requires more dependencies
- [ ] Rust prevents memory leaks and runs faster
- [ ] Rust only supports single-threading
- [ ] Rust cannot interact with Sui blockchain