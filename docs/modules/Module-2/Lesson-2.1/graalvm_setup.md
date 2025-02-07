## **📄 File: `Module-2/Lesson-2.1/graalvm_setup.md`**
📌 **Lesson: Installing GraalVM for JavaScript Execution**  

### **Why GraalVM?**
- GraalVM allows **running JavaScript inside Java** without needing **Node.js**.
- It supports **multiple languages** in a single runtime (Java, JavaScript, Python, Rust).
- Ideal for **blockchain**, **polyglot programming**, and **performance optimization**.

---

### **1️⃣ Install GraalVM**
#### **Linux & macOS:**
```sh
curl -L https://github.com/graalvm/graalvm-ce-builds/releases/latest/download/graalvm-ce-java17-linux-amd64.tar.gz -o graalvm.tar.gz
mkdir -p $HOME/graalvm
tar -xzf graalvm.tar.gz -C $HOME/graalvm
export PATH=$HOME/graalvm/graalvm-ce-java17/bin:$PATH
```
#### **Windows (via SDKMAN):**
```sh
sdk install java 22.0.r17-grl
```
✅ **Verify Installation:**
```sh
java -version
```
Expected output:
```
openjdk version "17"
GraalVM CE 22.0.0
```

📌 **Next Lesson:** Run JavaScript inside Java.

---

