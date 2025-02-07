## **📄 File: `Final-Project/frontend/src/main.rs`**
📌 **Yew Frontend to Interact with API**
```rust
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| "Fetching balance...".to_string());

    let fetch_balance = {
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
            <h1>{"Sui Blockchain Balance"}</h1>
            <p>{(*state).clone()}</p>
            <button onclick={fetch_balance}>{"Fetch Balance"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```
✅ **Run the frontend:**
```sh
trunk serve
```
📌 **Final Step:** Test the full dApp!

---