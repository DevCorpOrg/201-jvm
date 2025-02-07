## **📄 File: Lesson-1.2/exercises/simple_contract.move**

## **🛠 Hands-On: Create & Deploy a Move Smart Contract**
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

### **Deploy the Move Contract**
```sh
sui move publish
```
📌 **Next:** Test transactions with the deployed contract.
