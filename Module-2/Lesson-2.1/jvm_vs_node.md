## **📂 Module-2: JVM & GraalVM for JavaScript Execution**  

```
Module-2/
│── Lesson-2.1/              
│   ├── jvm_vs_node.md       # JVM vs Node.js Execution Models
│   ├── graalvm_setup.md     # Installing GraalVM to Run JavaScript
│   ├── exercises/           
│   │   ├── run_js_in_java.java  # Run JavaScript inside Java using GraalVM
│   │   ├── replace_nodejs.java  # Execute blockchain scripts without Node.js
```

---

## **📄 File: `Module-2/Lesson-2.1/jvm_vs_node.md`**
📌 **Lesson: JVM vs Node.js Execution Models**  

### **What is the JVM?**
- The **Java Virtual Machine (JVM)** allows Java to run **on any OS** by converting **bytecode into machine instructions**.
- Java programs compile into **bytecode** that the **JVM interprets or compiles (JIT - Just In Time Compilation)**.
- The JVM includes:
  - **Class Loader** – Loads Java classes
  - **Just-In-Time Compiler (JIT)** – Optimizes execution
  - **Garbage Collector (GC)** – Manages memory automatically

---

### **How is Node.js Different?**
| **Feature**          | **JVM (Java)**                     | **Node.js (JavaScript)**      |
|----------------------|----------------------------------|------------------------------|
| **Execution Model**  | Compiled → JVM Bytecode         | Interpreted or JIT-compiled |
| **Performance**      | Optimized via JIT + HotSpot     | Slower due to dynamic typing |
| **Use Cases**        | Backend, blockchain, finance   | Frontend, dApps, APIs       |
| **Concurrency**      | Multi-threaded, native threads | Single-threaded event loop  |

📌 **Next Lesson**: Installing **GraalVM** to replace Node.js.

---

