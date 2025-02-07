## **📄 File: Module-6/Lesson-6.1/exercises/simple_yew_app.rs**
📌 **Exercise: Create a Basic Yew Component**
```rust
use yew::prelude::*;

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    html! {
        <h1>{"Hello, WebAssembly from Yew!"}</h1>
    }
}

fn main() {
    yew::Renderer::<HelloWorld>::new().render();
}
```
✅ **Run the Yew app:**
```sh
trunk serve
```
📌 **Next Exercise:** Connect Yew to an API.