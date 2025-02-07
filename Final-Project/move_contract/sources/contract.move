## **📄 File: `Final-Project/move_contract/sources/contract.move`**
📌 **Move Smart Contract Logic**
```move
module sui::example::TokenContract {
    struct Token has key, store {
        value: u64
    }

    public entry fun mint(recipient: address, amount: u64) {
        let token = Token { value: amount };
        transfer::transfer(token, recipient);
    }
}
```
✅ **Compile and Deploy Contract:**
```sh
sui move build
sui move publish
```
📌 **Next Step:** Build the Yew frontend.