use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Event, SubmitEvent}; 
use yew::prelude::*;
use gloo::utils::window;

#[derive(Clone, PartialEq)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub owner: String,
}

// #[derive(Serialize, Deserialize, Clone, Debug)]
// struct ApiResponse<T> {
//     success: bool,
//     data: Option<T>,
//     message: Option<String>,
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
fn login() -> Html {
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

            spawn_local(async move {
                let response = Request::post("http://localhost:8080/user/login")
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({ "username": username, "password": password }))
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        gloo::console::log!("Login successful!");
                        error.set(String::new());
                        window().location().set_href("/channel_list").unwrap();
                    }
                    Ok(resp) if resp.status() == 400 => {
                        error.set("Already logged in!".to_string());
                    }
                    Ok(resp) if resp.status() == 401 => {
                        error.set("Invalid username or password.".to_string());
                    }
                    _ => {
                        error.set("Error Occurred.".to_string());
                    }
                }
            })
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
                        class="input"
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
                        class="input"
                    />
                    <button type="submit" class="button">{"Login"}</button>
                </form>
            </div>
        </div>
    }
}

#[function_component(Register)]
fn register() -> Html {
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

            spawn_local(async move {
                let response = Request::post("http://localhost:8080/user/register")
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({ "username": username, "password": password }))
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        gloo::console::log!("Registration successful!");
                        error.set(String::new());
                        window().location().set_href("/login").unwrap();
                    }
                    Ok(resp) if resp.status() == 400 => {
                        error.set("Already logged in!".to_string());
                    }
                    Ok(resp) if resp.status() == 409 => {
                        error.set("Username already exists.".to_string());
                    }
                    _ => {
                        error.set("Error occurred.".to_string());
                    }
                }
            })
        })
    };

    html! {
        <div>
            <div class="form">
                <h2>{"Register"}</h2>
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
                        class="input"
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
                        class="input"
                    />
                    <button type="submit" class="button">{"Register"}</button>
                </form>
            </div>
        </div>
    }
}

#[function_component(ChannelList)]
fn channel_list() -> Html {
    let channels = vec![
        Channel { id: 1, name: String::from("Channel 1"), owner: String::from("Concer") },
        Channel { id: 2, name: String::from("Channel 2"), owner: String::from("Torin") },
        Channel { id: 3, name: String::from("Channel 3"), owner: String::from("Chen") },
        Channel { id: 4, name: String::from("Channel 4"), owner: String::from("Mr.Gao") },
        Channel { id: 5, name: String::from("Channel 5"), owner: String::from("Mr.Wang") },
        Channel { id: 6, name: String::from("Channel 6"), owner: String::from("MissTuo") },
    ];
    
    let selected_channel = use_state(|| None::<i32>);
    let error = use_state(String::new);

    let on_channel_select = {
        let selected_channel = selected_channel.clone();
        Callback::from(move |id: i32| {
            selected_channel.set(Some(id));
        })
    };

    let on_enter = {
        let selected_channel = selected_channel.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            if let Some(id) = *selected_channel {
                gloo::console::log!("Entering channel:", id);
                window().location().set_href(&format!("/channel/{}", id)).unwrap();
            } else {
                error.set("Please select a channel first".to_string());
            }
        })
    };

    html! {
        <div class="channel-container">
            <h2>{"Available Channels"}</h2>
            {if !(*error).is_empty() {
                html! { <div class="error-message">{&*error}</div> }
            } else {
                html! {}
            }}
            <div class="channel-list">
                { channels.iter().map(|channel| {
                    let is_selected = *selected_channel == Some(channel.id);
                    let channel_id = channel.id;
                    let on_select = {
                        let on_channel_select = on_channel_select.clone();
                        Callback::from(move |_| on_channel_select.emit(channel_id))
                    };
                    
                    html! {
                        <div class={classes!("channel-item", is_selected.then(|| "selected"))}>
                            <div class="channel-selector" onclick={on_select}>
                                <div class={classes!("radio-circle", is_selected.then(|| "checked"))} />
                            </div>
                            <div class="channel-info">
                                <span class="channel-name">{&channel.name}</span>
                                <span class="channel-owner">{format!("Owner: {}", &channel.owner)}</span>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            <button onclick={on_enter} class="button enter-button">
                {"Enter Channel"}
            </button>
        </div>
    }
}

#[function_component(CreateChannel)]
fn channel_create() -> Html {
    let name = use_state(String::new);
    let error = use_state(String::new);

    let onsubmit = {
        let name = name.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = (*name).clone();
            let error = error.clone();

            spawn_local(async move {
                let response = Request::post("http://localhost:8080/channel/create")
                    .header("Content-Type", "application/json")
                    .json(&serde_json::json!({ "name": name}))
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        gloo::console::log!("Channel Create successful!");
                        error.set(String::new());
                        window().location().set_href("/channel_list").unwrap();
                    }
                    Ok(resp) if resp.status() == 401 => {
                        error.set("Unauthorized!".to_string());
                    }
                    Ok(resp) if resp.status() == 409 => {
                        error.set("Channel name already exists.".to_string());
                    }
                    _ => {
                        error.set("Error occurred.".to_string());
                    }
                }
            })
        })
    };

    html! {
        <div>
            <div class="form">
                <h2>{"Create A Channel"}</h2>
                {if !(*error).is_empty() {
                    html! { <div class="error-message">{&*error}</div> }
                } else {
                    html! {}
                }}
                <form {onsubmit}>
                    <input
                        type="text"
                        placeholder="Channel Name"
                        value={(*name).clone()}
                        onchange={
                            let name = name.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                                    name.set(input.value());
                                }
                            })
                        }
                        class="input"
                    />
                    <button type="submit" class="button">{"Create"}</button>
                </form>
            </div>
        </div>
    }
}

#[function_component(ChatRoom)]
fn chat_room() -> Html {
    let location = window().location();
    let path = location.pathname().unwrap_or_else(|_| "/".to_string());
    let channel_id = path.split('/').last()
        .and_then(|id| id.parse::<i32>().ok())
        .unwrap_or(0);

    let channels = vec![
        Channel { id: 1, name: String::from("Channel 1"), owner: String::from("Concer") },
        Channel { id: 2, name: String::from("Channel 2"), owner: String::from("Torin") },
        Channel { id: 3, name: String::from("Channel 3"), owner: String::from("Chen") },
        Channel { id: 4, name: String::from("Channel 4"), owner: String::from("Mr.Gao") },
        Channel { id: 5, name: String::from("Channel 5"), owner: String::from("Mr.Wang") },
        Channel { id: 6, name: String::from("Channel 6"), owner: String::from("MissTuo") },
    ];

    let current_channel = channels.iter()
        .find(|c| c.id == channel_id)
        .cloned()
        .unwrap_or(Channel { 
            id: 0, 
            name: String::from("Unknown Channel"), 
            owner: String::from("Unknown") 
        });

    let message = use_state(String::new);

    let on_message_change = {
        let message = message.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                message.set(input.value());
            }
        })
    };

    let on_send = {
        let message = message.clone();
        Callback::from(move |_| {
            let msg = (*message).clone();
            if !msg.is_empty() {
                gloo::console::log!("Sending message:", msg);
                message.set(String::new());
            }
        })
    };

    let on_exit = Callback::from(move |_| {
        window().location().set_href("/channel_list").unwrap();
    });

    html! {
        <div class="chat-container">
            <div class="chat-header">
                <div class="header-left">
                    <button onclick={on_exit} class="exit-button">{"Exit"}</button>
                    <h2 class="channel-title">
                        {format!("Channel: {} (Owner: {})", current_channel.name, current_channel.owner)}
                    </h2>
                </div>
            </div>
            <div class="chat-messages">
                // Messages will be displayed here
            </div>
            <div class="chat-input">
                <input
                    type="text"
                    placeholder="Type a message..."
                    value={(*message).clone()}
                    onchange={on_message_change}
                    class="message-input"
                />
                <button onclick={on_send} class="send-button">{"Send"}</button>
            </div>
        </div>
    }
}

#[function_component(Index)]
fn index() -> Html {
    let location = window().location();
    let path = location.pathname().unwrap_or_else(|_| "/".to_string());

    let content = match path.as_str() {
        "/" => html! { <Welcome /> },
        "/login" => html! { <Login /> },
        "/register" => html! { <Register /> },
        "/channel_list" => html! { <ChannelList /> },
        path if path.starts_with("/channel/") => html! { <ChatRoom /> },
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