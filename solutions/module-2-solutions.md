✅ **Answer Key:**
1. ✔️ JVM translates Java bytecode into machine code
2. ✔️ GraalVM allows execution of JavaScript inside Java
3. ✔️ Correct installation command for GraalVM on Linux/macOS
4. ✔️ GraalVM enables Java to execute JavaScript without Node.js

---


## **🛠 Hands-On: Install GraalVM & Run JavaScript in Java**
### **1️⃣ Install GraalVM**
#### **Linux/macOS:**
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
Expected Output:
```
openjdk version "17"
GraalVM CE 22.0.0
```

### **2️⃣ Run JavaScript Inside Java with GraalVM** (`run_js_in_java.java`)
```java
import org.graalvm.polyglot.*;

public class RunJS {
    public static void main(String[] args) {
        try (Context context = Context.create()) {
            context.eval("js", "console.log('🚀 Running JavaScript inside Java with GraalVM!');");
        }
    }
}
```
✅ **Compile & Run:**
```sh
javac RunJS.java
java RunJS
```
Expected Output:
```
🚀 Running JavaScript inside Java with GraalVM!
```

---

## **🛠 Hands-On: Replacing Node.js with Java** (`replace_nodejs.java`)
### **Problem: Blockchain SDKs Require Node.js**
Most blockchain tools use `sui.js`, requiring **Node.js**. We can replace it with **GraalVM** in Java.

### **Solution: Call an API Without Node.js**
```java
import org.graalvm.polyglot.*;

public class ReplaceNodeJS {
    public static void main(String[] args) {
        try (Context context = Context.create()) {
            context.eval("js", "fetch('https://sui-api.example.com/get-balance')\n" +
                          "  .then(response => response.json())\n" +
                          "  .then(data => console.log('Balance:', data.balance));");
        }
    }
}
```
✅ **Run the script without Node.js:**
```sh
java ReplaceNodeJS
```
Expected output:
```
Balance: 1000 SUI
```

