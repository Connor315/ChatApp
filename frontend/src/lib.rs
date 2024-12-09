// use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Event};
use yew::prelude::*;
use gloo::utils::window;

// #[derive(Serialize, Deserialize, Clone, Debug)]
// struct ApiResponse<T> {
//     success: bool,
//     data: Option<T>,
//     message: Option<String>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// struct User {
//     id: i32,
//     username: String,
//     #[serde(skip_serializing)]
//     password: String,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// struct Channel {
//     id: i32,
//     name: String,
//     owner: String,
// }

#[function_component(Welcome)]
fn welcome() -> Html {
    let on_login_click = Callback::from(move |_| {
        window().location().set_href("/login").unwrap();
    });

    let on_register_click = Callback::from(move |_| {
        window().location().set_href("/register").unwrap();
    });

    html! {
        <div class="form">
            <div id="welcome-message">
                <h2>{"Welcome to Chat App"}</h2>
                <p>{"Please log in/register to continue."}</p>
            </div>
            <button onclick={on_login_click} class="button">
                {"Login"}
            </button>
            <button onclick={on_register_click} class="button">
                {"Register"}
            </button>
        </div>
    }
}

#[function_component(Login)]
fn login_form() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(String::new);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let error = error.clone();
        })
    };

    html! {
        <div>
            <div class="form">
                <h2>{"Login"}</h2>
                {if !(*error).is_empty() {
                    html! { <div class="error-message">{&*error}</div> }
                } else {
                    html! {}
                }}
                <form {onsubmit}>
                    <input
                        type="text"
                        placeholder="Username"
                        value={(*username).clone()}
                        onchange={
                            let username = username.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    username.set(input.value());
                                }
                            })
                        }
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={(*password).clone()}
                        onchange={
                            let password = password.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    password.set(input.value());
                                }
                            })
                        }
                    />
                    <button type="submit">{"Login"}</button>
                </form>
            </div>
        </div>
    }
}

// #[function_component(ChannelList)]
// fn channel_list() -> Html {
//     let channels = use_state(Vec::new);
//     let error = use_state(String::new);

//     {
//         let channels = channels.clone();
//         let error = error.clone();

//         use_effect_with_deps(
//             move |_| {
//                 spawn_local(async move {
//                     let resp = Request::get("/api/channels")
//                         .send()
//                         .await;

//                     match resp {
//                         Ok(response) => {
//                             let api_response: ApiResponse<Vec<Channel>> = response.json().await.unwrap();
//                             if api_response.success {
//                                 channels.set(api_response.data.unwrap_or_default());
//                             } else {
//                                 error.set(api_response.message.unwrap_or_else(|| "Failed to load channels".to_string()));
//                             }
//                         }
//                         Err(_) => {
//                             error.set("Network error".to_string());
//                         }
//                     }
//                 });
//                 || ()
//             },
//             (),
//         );
//     }

//     html! {
//         <div class="channel-list-container">
//             <h2>{"Available Channels"}</h2>
//             {if !(*error).is_empty() {
//                 html! { <div class="error-message">{&*error}</div> }
//             } else {
//                 html! {
//                     <div class="channel-list">
//                         {channels.iter().map(|channel| {
//                             html! {
//                                 <div class="channel-item">
//                                     <div class="channel-info">
//                                         <span class="channel-name">{&channel.name}</span>
//                                         <span class="channel-owner">{format!("Owner: {}", &channel.owner)}</span>
//                                     </div>
//                                     <button class="enter-button">{"Enter"}</button>
//                                 </div>
//                             }
//                         }).collect::<Html>()}
//                     </div>
//                 }
//             }}
//         </div>
//     }
// }

#[function_component(Index)]
fn index() -> Html {
    let location = window().location();
    let path = location.pathname().unwrap_or_else(|_| "/".to_string());

    let content = match path.as_str() {
        "/" => html! { <Welcome /> },
        "/login" => html! { <Login /> },
        // "/register.html" => html! { <RegisterPage /> },
        _ => html! { <h1>{ "404 Not Found" }</h1> },
    };

    html! {
        { content }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Index>::new().render();
}