## **📄 File: `Module-2/Lesson-2.1/exercises/run_js_in_java.java`**
📌 **Exercise: Run JavaScript Inside Java Using GraalVM**  

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
Expected output:
```
🚀 Running JavaScript inside Java with GraalVM!
```

📌 **Next Exercise:** Replace **Node.js** with Java.

---
