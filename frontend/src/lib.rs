use gloo::net::http::Request;
use yew::prelude::*;
use wasm_bindgen::prelude::*; // For #[wasm_bindgen] and related macros
// use wasm_bindgen_futures::spawn_local; // To handle async futures in WASM
use web_sys::{HtmlInputElement, Event}; // For working with DOM elements
// use js_sys::JsCast; // For .dyn_into() conversions
use serde::{Serialize, Deserialize}; // For serializing/deserializing JSON

#[function_component(Index)]
fn index() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let username1 = use_state(String::new);
    let password1 = use_state(String::new);
    let login_error = use_state(|| None::<String>);
    let register_error = use_state(|| None::<String>);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let login_error = login_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username = (*username).clone();
            let password = (*password).clone();

            // wasm_bindgen_futures::spawn_local(async move {
            //     let response = Request::post("http://localhost:8080/user/login")
            //         .header("Content-Type", "application/json")
            //         .json(&serde_json::json!({ "username": username, "password": password }))
            //         .unwrap()
            //         .send()
            //         .await;

            //     match response {
            //         Ok(resp) if resp.ok() => {
            //             gloo::console::log!("Login successful!");
            //             login_error.set(None);
            //         }
            //         Ok(_) => {
            //             login_error.set(Some("Invalid username or password.".to_string()));
            //         }
            //         Err(_) => {
            //             login_error.set(Some("Network error occurred.".to_string()));
            //         }
            //     }
            // });
        })
    };

    let onregister = {
        let username = username1.clone();
        let password = password1.clone();
        let register_error = register_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            let username = (*username).clone();
            let password = (*password).clone();

            // wasm_bindgen_futures::spawn_local(async move {
            //     let response = Request::post("http://localhost:8080/user/login")
            //         .header("Content-Type", "application/json")
            //         .json(&serde_json::json!({ "username": username, "password": password }))
            //         .unwrap()
            //         .send()
            //         .await;

            //     match response {
            //         Ok(resp) if resp.ok() => {
            //             gloo::console::log!("Login successful!");
            //             login_error.set(None);
            //         }
            //         Ok(_) => {
            //             login_error.set(Some("Invalid username or password.".to_string()));
            //         }
            //         Err(_) => {
            //             login_error.set(Some("Network error occurred.".to_string()));
            //         }
            //     }
            // });
        })
    };

    html! {
        <div>
            <div class="login-form">
                <form {onsubmit}>
                    <input
                        type="text"
                        placeholder="Username"
                        value={(*username).clone()}
                        oninput={
                            let username = username.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    username.set(input.value());
                                }
                            })
                        }
                        class="username-input"
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={(*password).clone()}
                        oninput={
                            let password = password.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    password.set(input.value());
                                }
                            })
                        }
                        class="password-input"
                    />
                    <button type="submit" class="login-button">{"Log In"}</button>
                </form>
                if let Some(err) = &*login_error {
                    <p class="error">{err}</p>
                }
            </div>

            <div class="register-form">
                <form onsubmit={onregister}>
                    <input
                        type="text"
                        placeholder="Username"
                        value={(*username).clone()}
                        oninput={
                            let username = username.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    username.set(input.value());
                                }
                            })
                        }
                        class="username-input"
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={(*password).clone()}
                        oninput={
                            let password = password.clone();
                            Callback::from(move |e: InputEvent| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    password.set(input.value());
                                }
                            })
                        }
                        class="password-input"
                    />
                    <button type="submit" class="register-button">{"Register"}</button>
                </form>
                if let Some(err) = &*register_error {
                    <p class="error">{err}</p>
                }
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Index>::new().render();
}