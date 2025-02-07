## **📄 File: Module-4/Lesson-4.1/exercises/simple_contract.move**
📌 **Exercise: Basic Move Contract**
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

📌 **Next Exercise:** Deploy using Rust.