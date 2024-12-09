use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Event};
use yew::prelude::*;

// API响应的数据结构
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: i32,
    username: String,
    #[serde(skip_serializing)]
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Channel {
    id: i32,
    name: String,
    owner: String,
}

#[derive(Clone, PartialEq)]
enum AuthStep {
    SelectType,
    Login,
    Register,
    ChannelList,
}

#[derive(Properties, PartialEq, Clone)]
struct WelcomeProps {
    on_select: Callback<AuthStep>,
}

// 欢迎页面组件
#[function_component(Welcome)]
fn welcome(props: &WelcomeProps) -> Html {
    let on_login_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| {
            on_select.emit(AuthStep::Login);
        })
    };

    let on_register_click = {
        let on_select = props.on_select.clone();
        Callback::from(move |_| {
            on_select.emit(AuthStep::Register);
        })
    };

    html! {
        <div class="auth-container">
            <div class="auth-type-selection">
                <h2>{"Welcome to Chat App"}</h2>
                <div class="auth-buttons">
                    <button onclick={on_login_click} class="auth-button">
                        {"Login"}
                    </button>
                    <button onclick={on_register_click} class="auth-button">
                        {"Register"}
                    </button>
                </div>
            </div>
        </div>
    }
}

// 登录表单组件
#[function_component(LoginForm)]
fn login_form() -> Html {
    let username = use_state(String::new);
    let password = use_state(String::new);
    let error = use_state(String::new);
    let history = use_navigator().unwrap();

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let password = (*password).clone();
            let error = error.clone();

            spawn_local(async move {
                let resp = Request::post("/api/login")
                    .json(&serde_json::json!({
                        "username": username,
                        "password": password,
                    }))
                    .unwrap()
                    .send()
                    .await;

                match resp {
                    Ok(response) => {
                        let api_response: ApiResponse<User> = response.json().await.unwrap();
                        if api_response.success {
                            history.push(&Route::ChannelList);
                        } else {
                            error.set(api_response.message.unwrap_or_else(|| "Login failed".to_string()));
                        }
                    }
                    Err(_) => {
                        error.set("Network error".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="auth-container">
            <div class="auth-form">
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

// 频道列表组件
#[function_component(ChannelList)]
fn channel_list() -> Html {
    let channels = use_state(Vec::new);
    let error = use_state(String::new);

    {
        let channels = channels.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let resp = Request::get("/api/channels")
                        .send()
                        .await;

                    match resp {
                        Ok(response) => {
                            let api_response: ApiResponse<Vec<Channel>> = response.json().await.unwrap();
                            if api_response.success {
                                channels.set(api_response.data.unwrap_or_default());
                            } else {
                                error.set(api_response.message.unwrap_or_else(|| "Failed to load channels".to_string()));
                            }
                        }
                        Err(_) => {
                            error.set("Network error".to_string());
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="channel-list-container">
            <h2>{"Available Channels"}</h2>
            {if !(*error).is_empty() {
                html! { <div class="error-message">{&*error}</div> }
            } else {
                html! {
                    <div class="channel-list">
                        {channels.iter().map(|channel| {
                            html! {
                                <div class="channel-item">
                                    <div class="channel-info">
                                        <span class="channel-name">{&channel.name}</span>
                                        <span class="channel-owner">{format!("Owner: {}", &channel.owner)}</span>
                                    </div>
                                    <button class="enter-button">{"Enter"}</button>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            }}
        </div>
    }
}

#[function_component(Index)]
fn index() -> Html {
    let current_view = use_state(|| AuthStep::SelectType);

    let handle_auth_select = {
        let current_view = current_view.clone();
        Callback::from(move |step: AuthStep| {
            current_view.set(step);
        })
    };

    html! {
        <div>
            {
                match *current_view {
                    AuthStep::SelectType => {
                        html! { <Welcome on_select={handle_auth_select} /> }
                    }
                    AuthStep::Login => {
                        html! { <LoginForm /> }
                    }
                    AuthStep::Register => {
                        html! { <div>{"Register form will be here"}</div> }
                    }
                    AuthStep::ChannelList => {
                        html! { <ChannelList /> }
                    }
                }
            }
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Index>::new().render();
}