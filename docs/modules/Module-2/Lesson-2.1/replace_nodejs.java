## **📄 File: `Module-2/Lesson-2.1/exercises/replace_nodejs.java`**
📌 **Exercise: Replace Node.js for Blockchain Interactions**  

```java
import org.graalvm.polyglot.*;

public class ReplaceNodeJS {
    public static void main(String[] args) {
        try (Context context = Context.create()) {
            context.eval("js", "fetch('https://sui-api.example.com/get-balance')\\n" +
                              "  .then(response => response.json())\\n" +
                              "  .then(data => console.log('Balance:', data.balance));");
        }
    }
}
```

✅ **Run without Node.js:**
```sh
java ReplaceNodeJS
```
Expected output:
```
Balance: 1000 SUI
```

📌 **Next Module:** **Sui SDKs Without Node.js (Module 3)**.

---