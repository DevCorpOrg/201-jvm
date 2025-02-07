# Module 6: Deploying WASM-Based Sui dApps With Yew

## 📌 Overview
This module covers **how to build a WASM-based frontend for Sui dApps using Yew**, a Rust-based alternative to React. We will explore the advantages of Yew, build a simple blockchain-connected frontend, and compare it to JavaScript frameworks.

---

## **📖 Concept: Why Yew is Better Than React for Blockchain dApps**
### **What is Yew?**
- A **Rust-based frontend framework** for building WebAssembly (WASM) applications.
- Allows developers to use **Rust instead of JavaScript** for UI development.
- **Compiles to WASM**, making it more efficient and secure than React.

### **Why Use Yew Instead of React?**
✅ **Faster Execution:** Yew compiles to **WASM**, running at near-native speeds.
✅ **Memory Safety:** Rust’s ownership model prevents memory leaks and runtime errors.
✅ **Better Security:** Unlike JavaScript, Rust eliminates many **XSS and supply-chain attacks**.
✅ **No Node.js Dependencies:** Unlike React, Yew does **not require npm packages**.

📌 **Next:** Build a Yew-based dApp frontend.

## **📝 Quiz: What Are the Advantages of Rust-Based Frontends?**
### **Question 1:**
What is Yew?
- [ ] A JavaScript frontend framework
- [ ] A Rust framework for building WebAssembly frontends
- [ ] A replacement for Vue.js
- [ ] A database system

### **Question 2:**
Which command starts a Yew application?
- [ ] `cargo serve`
- [ ] `yarn start`
- [ ] `trunk serve`
- [ ] `sui start`

### **Question 3:**
Why is Yew preferred over React for blockchain applications?
- [ ] It has more third-party libraries
- [ ] It eliminates JavaScript dependencies and improves security
- [ ] It is maintained by Facebook
- [ ] It uses JSX syntax

### **Question 4:**
What makes Rust-based frontends more secure?
- [ ] They require more configuration
- [ ] They use JavaScript for memory management
- [ ] They prevent common web vulnerabilities like XSS
- [ ] They are only compatible with the Sui blockchain