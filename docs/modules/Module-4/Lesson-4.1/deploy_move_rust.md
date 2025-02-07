## **📄 File: Module-4/Lesson-4.1/deploy_move_rust.md**
📌 **Lesson: Deploying Move Contracts Using Rust**

### **1️⃣ Install Move CLI**
```sh
sui move new my_contract
```

### **2️⃣ Write a Move Contract**
Open `sources/MyContract.move` and add:
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

### **3️⃣ Compile the Contract**
```sh
sui move build
```

### **4️⃣ Deploy Using Rust Instead of JavaScript**
Instead of:
```javascript
await client.publishPackage("./move_contract");
```
Use Rust:
```sh
cargo run --bin deploy_contract
```
📌 **Next Exercise:** Deploy a contract using Rust.
