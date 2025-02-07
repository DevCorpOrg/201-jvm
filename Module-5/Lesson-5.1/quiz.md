# Module 5: Rust-Based APIs for Sui Blockchain

## 📌 Overview
This module covers **how to build a Rust-based API** for interacting with the Sui blockchain using **Axum** instead of JavaScript-based frameworks like Express.js. We will explore API development for fetching blockchain data and executing transactions.

---

## **📖 Concept: Express.js Alternative in Rust (Axum, Actix-web)**
### **Why Use Rust Instead of Node.js for Blockchain APIs?**
✅ **Performance:** Rust is **compiled to machine code**, making it faster than JavaScript.
✅ **Memory Safety:** No garbage collector, reducing unpredictable execution pauses.
✅ **Concurrency:** Rust provides **multi-threaded async execution**, better than Node.js’ single-threaded event loop.
✅ **Security:** No `npm` dependency vulnerabilities, which are common in JavaScript ecosystems.

📌 **Next:** Build a Rust-based blockchain API using Axum.

---

## **📝 Quiz: Why is Rust Better for Blockchain Backends?**
### **Question 1:**
Which of the following is **not** a reason to use Rust over Node.js for blockchain APIs?
- [ ] Rust offers better concurrency
- [ ] Rust does not rely on garbage collection
- [ ] Rust prevents common JavaScript-based security issues
- [ ] Rust requires more dependencies than Node.js

### **Question 2:**
Which Rust framework is commonly used as an alternative to Express.js?
- [ ] Warp
- [ ] Rocket
- [ ] Axum
- [ ] Next.js

### **Question 3:**
What is the default port Axum binds to in a basic API example?
- [ ] 5000
- [ ] 8080
- [ ] 3030
- [ ] 9000

### **Question 4:**
Why is Rust preferred for blockchain transaction APIs?
- [ ] It is interpreted and dynamically typed
- [ ] It executes slower but uses fewer resources
- [ ] It is memory-safe, concurrent, and highly performant
- [ ] It only supports WebAssembly