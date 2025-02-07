✅ **Answer Key:**
1. ✔️ Yew is a Rust-based WebAssembly frontend framework
2. ✔️ `trunk serve` starts a Yew application
3. ✔️ Yew eliminates JavaScript dependencies and improves security
4. ✔️ Rust-based frontends prevent common web vulnerabilities like XSS

---

## **🛠 Hands-On: Build a Yew Frontend Using WASM**
### **1️⃣ Install Yew & Dependencies**
```sh
cargo install trunk
cargo add yew wasm-bindgen
```

### **2️⃣ Create a Simple Yew Component** (`simple_yew_app.rs`)
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
📌 **Next:** Connect Yew to a Rust-based blockchain API.

---

## **🛠 Hands-On: Fetch Blockchain Data in Yew** (`connect_yew_api.rs`)
### **3️⃣ Connect Yew Frontend to Rust API**
```rust
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| "Fetching data...".to_string());

    let fetch_data = {
        let state = state.clone();
        Callback::from(move |_| {
            spawn_local(async move {
                let response = Request::get("http://127.0.0.1:8080/balance")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                state.set(response);
            });
        })
    };

    html! {
        <div>
            <h1>{"Blockchain Balance"}</h1>
            <p>{(*state).clone()}</p>
            <button onclick={fetch_data}>{"Fetch Balance"}</button>
        </div>
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
📌 **Next:** Deploy the Yew dApp!

---