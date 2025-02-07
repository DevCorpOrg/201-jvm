# Module 1: Lesson 1.2 - Move Basics

## 📌 Overview
In this lesson, we explore **Move**, a **resource-oriented programming language** designed for blockchain smart contracts. We will cover **resources, modules, and transactions**, followed by a hands-on tutorial on writing and deploying a simple Move contract.

---

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

## **📝 Quiz: Understanding Move’s Security Model**
### **Question 1:**
What makes Move different from Solidity?
- [ ] Move allows dynamic typing
- [ ] Move prevents reentrancy attacks with resource ownership
- [ ] Solidity is more secure
- [ ] Move uses garbage collection

### **Question 2:**
What command is used to compile a Move contract?
- [ ] `cargo build`
- [ ] `sui move build`
- [ ] `sui move execute`
- [ ] `rustc build`

### **Question 3:**
Why are **resources** important in Move?
- [ ] They allow smart contracts to modify Ethereum state
- [ ] They prevent unauthorized duplication of assets
- [ ] They are used for garbage collection
- [ ] They speed up Solidity contracts