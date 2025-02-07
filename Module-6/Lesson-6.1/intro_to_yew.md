# Module 6: WASM-Based dApps with Yew

## 📌 Overview
In this module, we will learn how to **build, compile, and deploy WebAssembly (WASM)-based decentralized applications (dApps) using Yew**, a Rust-based alternative to React for frontend development.

### **🎯 Learning Objectives**
By the end of this module, you will:
✅ Understand **why WebAssembly (WASM) is better than JavaScript for dApps**
✅ Learn the basics of **Yew, Rust’s alternative to React**
✅ Build a **WASM-based frontend for a blockchain dApp**
✅ Connect the frontend to a **Rust-based blockchain API**

## 📂 Module Structure
```plaintext
Module-6/
│── Lesson-6.1/                # WASM-Based dApps with Yew
│   ├── intro_to_yew.md        # Introduction to Yew and WebAssembly
│   ├── build_yew_dapp.md      # Building a dApp frontend using Yew
│   ├── exercises/           
│   │   ├── simple_yew_app.rs  # Basic Yew component
│   │   ├── connect_yew_api.rs # Connecting Yew frontend to a Rust API
```

---

## **📄 File: Module-6/Lesson-6.1/intro_to_yew.md**
📌 **Lesson: Introduction to Yew and WebAssembly**

### **Why Use WebAssembly (WASM) Instead of JavaScript?**
✅ **Faster execution** – Runs at near-native speed.
✅ **Secure memory model** – Unlike JavaScript, WASM prevents unsafe memory access.
✅ **Runs Rust in the browser** – Enables frontend development with Rust instead of JavaScript.

### **What is Yew?**
- **Rust-based frontend framework** (similar to React/Vue but compiled to WASM).
- Uses **Rust instead of JavaScript** for client-side applications.
- Allows **interoperability with Rust-based backend services**.

📌 **Next Lesson:** Building a **Yew dApp frontend**.