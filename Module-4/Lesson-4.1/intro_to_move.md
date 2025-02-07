# Module 4: Move Smart Contracts With Rust

## 📌 Overview
In this module, we will learn how to **write, compile, and deploy Move smart contracts** on the Sui blockchain using Rust tools instead of Node.js.

### **🎯 Learning Objectives**
By the end of this module, you will:
✅ Understand the **Move programming language** for smart contracts
✅ Write and deploy a **Move smart contract** using Rust
✅ Replace **JavaScript-based contract deployment (`sui.js`)** with **Rust CLI**
✅ Test and interact with **Move contracts using Rust SDK**

## 📂 Module Structure
```plaintext
Module-4/
│── Lesson-4.1/                # Move Smart Contracts With Rust
│   ├── intro_to_move.md       # Basics of Move for Smart Contracts
│   ├── deploy_move_rust.md    # Deploying Move Contracts Using Rust
│   ├── exercises/           
│   │   ├── simple_contract.move  # Basic Move contract
│   │   ├── deploy_contract.rs    # Deploy Move contract using Rust
```

---

## **📄 File: Module-4/Lesson-4.1/intro_to_move.md**
📌 **Lesson: Introduction to Move for Smart Contracts**

### **What is Move?**
Move is a **resource-oriented** programming language designed for **secure blockchain transactions**.

### **Why Move?**
✅ **Prevents double-spending** – Uses resource-based logic to enforce security.
✅ **Safe memory management** – No need for garbage collection.
✅ **Ideal for digital assets** – Works natively with tokens and NFTs.

### **Move vs Solidity (Ethereum Smart Contracts)**
| **Feature** | **Move (Sui)** | **Solidity (Ethereum)** |
|------------|---------------|-------------------------|
| **Execution Model** | Bytecode on Sui VM | Bytecode on EVM |
| **Security** | Prevents reentrancy & duplicate assets | Needs external security measures |
| **Concurrency** | Parallel execution | Single-threaded execution |
| **Resource Model** | Ownership-based (similar to Rust) | State-based |

📌 **Next Lesson:** Writing a basic **Move contract**.