### **📄 File: Module-1/Lesson-1.1/exercises/move_basics.move**  
#### **📌 Simple Move Smart Contract**
```move
module sui::example::MyContract {
    use sui::transfer;

    struct MyToken has key, store {
        value: u64
    }

    public entry fun mint(recipient: address, amount: u64) {
        let token = MyToken { value: amount };
        transfer::transfer(token, recipient);
    }
}
```
✅ **Compile the contract:**
```sh
sui move build
```
✅ **Run tests:**
```sh
sui move test
```

📌 **Next Module**: **JVM & GraalVM for JavaScript Execution (Module 2)**.

---

🚀 **Next Step: Writing Module-2!**