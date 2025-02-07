## **📄 File: Module-6/Lesson-6.1/build_yew_dapp.md**
📌 **Lesson: Building a dApp Frontend with Yew**

### **1️⃣ Install Yew & Dependencies**
```sh
cargo install trunk
cargo add yew
cargo add wasm-bindgen
```

### **2️⃣ Create a Simple Yew Component**
```rust
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Hello, Yew!"}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```
✅ **Run the Yew app:**
```sh
trunk serve
```
📌 **Next Exercise:** Connect Yew to a Rust-based API.