✅ **Answer Key:**
1. ✔️ Move prevents reentrancy attacks with resource ownership
2. ✔️ `sui move build` compiles Move contracts
3. ✔️ Resources prevent unauthorized asset duplication

---

## **🛠 Hands-On: Create & Deploy a Move Smart Contract**
### **1️⃣ Install Move CLI**
```sh
sui move new move_contract
```
---

### **2️⃣ Write a Simple Move Contract** (`sources/simple_contract.move`)
```move
module sui::example::SimpleContract {
    struct Counter has key, store {
        value: u64
    }

    public entry fun increment(counter: &mut Counter) {
        counter.value = counter.value + 1;
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

### **3️⃣ Deploy the Move Contract**
```sh
sui move publish
```
📌 **Next:** Test transactions with the deployed contract.

---
