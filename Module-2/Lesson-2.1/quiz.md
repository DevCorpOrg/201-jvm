# Module 2: JVM & GraalVM

## 📌 Overview
This module explores the **Java Virtual Machine (JVM), GraalVM, and their role in executing Java and JavaScript** for blockchain development. We will also see how GraalVM can **replace Node.js dependencies** in blockchain SDKs.

---

## **📖 Concept: JVM vs Node.js & GraalVM**
### **Java Virtual Machine (JVM)**
- **Executes Java bytecode** on any operating system.
- Uses **Just-In-Time (JIT) Compilation** for optimized performance.
- **Manages memory automatically** via **Garbage Collection (GC).**

### **Node.js vs JVM**
| Feature           | JVM (Java)                        | Node.js (JavaScript)  |
|------------------|---------------------------------|----------------------|
| **Execution**   | Runs compiled bytecode          | Interprets JavaScript|
| **Performance** | JIT optimizes execution         | Slower due to dynamic typing |
| **Use Cases**   | Backend systems, blockchain    | dApp frontends, APIs |
| **Concurrency** | Multi-threaded execution       | Single-threaded event loop |

### **What is GraalVM?**
- A **high-performance JVM** that can run Java, JavaScript, Python, and other languages.
- Allows **Java applications to execute JavaScript** without needing Node.js.
- Optimized for **polyglot execution and blockchain applications.**

✅ **Why Use GraalVM?**
- **Runs JavaScript inside Java** without Node.js.
- **Faster execution** of smart contract SDKs.
- **Allows Rust and Move integration** for blockchain interactions.

## **📝 Quiz: JVM vs GraalVM**
### **Question 1:**
What is the primary function of the JVM?
- [ ] Compiles Java code directly into machine code
- [ ] Translates Java bytecode into machine code
- [ ] Executes JavaScript natively
- [ ] Runs Move smart contracts

### **Question 2:**
Which of the following is a key advantage of GraalVM over a traditional JVM?
- [ ] It can only run Java
- [ ] It allows execution of JavaScript inside Java
- [ ] It does not support optimizations
- [ ] It cannot run polyglot applications

### **Question 3:**
Which command installs GraalVM on Linux/macOS?
- [ ] `npm install -g graalvm`
- [ ] `rustup install graalvm`
- [ ] `curl -L https://github.com/graalvm/graalvm-ce-builds/releases/latest/download/graalvm-ce-java17-linux-amd64.tar.gz -o graalvm.tar.gz`
- [ ] `brew install node`

### **Question 4:**
What is a major benefit of using GraalVM instead of Node.js for blockchain applications?
- [ ] GraalVM runs JavaScript faster than Node.js in all cases
- [ ] GraalVM allows Java applications to execute JavaScript without external dependencies
- [ ] GraalVM replaces Move smart contracts
- [ ] GraalVM can only execute Rust code
