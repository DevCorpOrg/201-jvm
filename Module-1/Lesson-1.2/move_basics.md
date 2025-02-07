## **📄 File: Lesson-1.2/move_basics.md**

## **📖 Concept: Move Language Fundamentals**
### **What is Move?**
- A **smart contract language** designed for blockchain security.
- Originally developed for **Facebook's Libra (now Diem, Aptos, and Sui).**
- Designed to prevent **double-spending** and enforce **strict ownership rules.**

### **Key Features of Move**
✅ **Resource-Oriented:** Enforces ownership and prevents duplication.
✅ **Modules-Based:** Contracts are stored in **modules** for security.
✅ **Static Typing:** Like Rust, Move enforces strict **type safety**.
✅ **High Performance:** Optimized for **parallel execution on Sui blockchain**.

### **Move vs Solidity** (Ethereum’s Smart Contract Language)
| Feature         | Move (Sui)       | Solidity (Ethereum) |
|---------------|----------------|-----------------|
| **Execution Model** | Bytecode on **Sui VM** | Bytecode on **EVM** |
| **Security**  | Prevents asset duplication | Needs external security measures |
| **Concurrency** | Parallel transaction execution | Single-threaded execution |
| **Gas Efficiency** | Optimized for **low fees** | High gas costs |

📌 **Next:** Write & deploy a simple Move contract.