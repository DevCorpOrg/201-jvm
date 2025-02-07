# Module 5: Rust-Based APIs for Blockchain

## 📌 Overview
In this module, we will learn how to **build, secure, and deploy Rust-based APIs** for blockchain interactions using **Axum** instead of JavaScript frameworks like Express.js.

### **🎯 Learning Objectives**
By the end of this module, you will:
✅ Understand why **Rust APIs** are better than Node.js-based solutions for blockchain
✅ Build a **REST API using Axum** for querying the Sui blockchain
✅ Secure API endpoints with **JWT authentication**
✅ Replace **JavaScript-based blockchain APIs (`express.js`)** with Rust

## 📂 Module Structure
```plaintext
Module-5/
│── Lesson-5.1/                # Rust-Based APIs for Blockchain
│   ├── intro_to_rust_apis.md   # Why Use Rust for Blockchain APIs?
│   ├── build_axum_api.md       # Building a REST API with Axum
│   ├── exercises/           
│   │   ├── simple_axum_api.rs  # Basic API using Axum
│   │   ├── secure_api_jwt.rs   # Secure API with JWT Authentication
```

---

## **📄 File: Module-5/Lesson-5.1/intro_to_rust_apis.md**
📌 **Lesson: Why Use Rust for Blockchain APIs?**

### **Why Rust Instead of Node.js for Blockchain APIs?**
❌ **Node.js (Express.js) Issues:**
- Single-threaded, **not ideal for high-load blockchain APIs**
- Security vulnerabilities from **NPM packages**
- Unnecessary **JavaScript runtime dependencies**

✅ **Why Rust is Better?**
- **Multi-threaded**, high-performance request handling
- **Memory-safe**, reducing attack surfaces
- **No runtime dependencies**—compiled to efficient binaries
- **Direct blockchain integration** with `sui-sdk-rs`

📌 **Next Lesson:** Building a **REST API using Axum**.
