## **📄 File: Module-6/Lesson-6.1/exercises/connect_yew_api.rs**
📌 **Exercise: Connect Yew Frontend to a Rust API**
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
                let response = Request::get("http://127.0.0.1:8080/status")
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
            <h1>{"Blockchain Status"}</h1>
            <p>{(*state).clone()}</p>
            <button onclick={fetch_data}>{"Fetch Status"}</button>
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
📌 **Next:** Deploy the full blockchain app!