use serde::{Deserialize, Serialize};
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
    password: &'a str,
}

#[component]
pub fn App() -> View {
    let name = create_signal(String::new());
    let password = create_signal(String::new());
    let greet_msg = create_signal(String::new());

    let greet = move |e: SubmitEvent| {
        e.prevent_default();
        spawn_local_scoped(async move {
            let args = serde_wasm_bindgen::to_value(&GreetArgs {
				name: &name.get_clone(),
                password: &password.get_clone(),
			})
			.unwrap();
            let new_msg = invoke("greet", args).await;
            greet_msg.set(new_msg.as_string().unwrap());
        })
    };

    view! {
        main(class="container") {
            h1 {
                "Tauri + Okta"
            }
            form(on:submit=greet) {
                div(class="input-group") {
                    input(id="login-input", bind:value=name, placeholder="Login...")
                }
                div(class="input-group") {
                    input(id="password-input", bind:value=password, r#type="password", placeholder="Password...")
                }
                button(r#type="submit") {
                    "Login"
                }
            }
            p {
                (greet_msg)
            }
        }
    }
}