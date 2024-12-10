use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Event}; 
use yew::prelude::*;
use gloo::utils::window;
use gloo_storage::{Storage, LocalStorage};

#[derive(Deserialize, Debug, Clone)]
struct Channel {
    id: i32,
    name: String,
    owner: String,
}

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
    let error = use_state(|| String::new());
    let channels = use_state(|| Vec::new());

    use_effect_with_deps({
        let channels = channels.clone();
        let error = error.clone();

        move |_| {
            spawn_local(async move {
                let response = Request::get("http://localhost:8080/channel/list")
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        match resp.json::<Vec<Channel>>().await {
                            Ok(channels_data) => channels.set(channels_data),
                            Err(_) => error.set("Error".to_string()),
                        }
                    }
                    _ => {
                        error.set("Unauthorized!".to_string());
                    }
                }
            });
            || ()
        }
    }, ());

    let selected_channel = use_state(|| None::<String>);

    let on_channel_select = {
        let selected_channel = selected_channel.clone();
        Callback::from(move |name: String| {
            selected_channel.set(Some(name));
        })
    };

    let on_enter = {
        let selected_channel = selected_channel.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            if let Some(name) = (*selected_channel).clone() {
                match LocalStorage::set("selected_channel", name.clone()) {
                    Ok(_) => gloo::console::log!("Channel saved to local storage"),
                    Err(_) => gloo::console::log!("Error saving to local storage:"),
                }
                window().location().set_href("/channel_room").unwrap();
            } else {
                error.set("Please select a channel first".to_string());
            }
        })
    };

    let logout = {
        let error = error.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let error = error.clone();

            spawn_local(async move {
                let response = Request::post("http://localhost:8080/user/logout")
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        gloo::console::log!("Log Out successful!");
                        window().location().set_href("/login").unwrap();
                    }
                    _ => {
                        error.set("Unauthorized!".to_string());
                    }
                }
            })
        })
    };

    let navigate = {
        Callback::from(move |_| {
            window().location().set_href("/channel_create").unwrap();
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
                { for channels.iter().map(|channel| {
                    let is_selected = *selected_channel == Some(channel.name.clone());
                    let channel_name = channel.name.clone();
                    let on_select = {
                        let on_channel_select = on_channel_select.clone();
                        Callback::from(move |_| on_channel_select.emit(channel_name.clone()))
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
                })}
            </div>
            <button onclick={on_enter} class="button enter-button">
                {"Enter Channel"}
            </button>
            <button onclick={logout} class="button func-btn">
                {"Log Out"}
            </button>
            <button onclick={navigate} class="button func-btn">
                {"Create A Channel"}
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

    let logout = {
        let error = error.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let error = error.clone();

            spawn_local(async move {
                let response = Request::post("http://localhost:8080/user/logout")
                    .send()
                    .await;

                match response {
                    Ok(resp) if resp.ok() => {
                        gloo::console::log!("Log Out successful!");
                        error.set(String::new());
                        window().location().set_href("/login").unwrap();
                    }
                    _ => {
                        error.set("Unauthorized!".to_string());
                    }
                }
            })
        })
    };

    let navigate = {
        Callback::from(move |_| {
            window().location().set_href("/channel_list").unwrap();
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
                    <button onclick={navigate} class="button">{"Available Channel List"}</button>
                    <button onclick={logout} class="button">{"Log Out"}</button>
                </form>
            </div>
        </div>
    }
}

#[function_component(ChatRoom)]
fn chat_room() -> Html {
    let error = use_state(|| String::new());
    let current_channel = use_state(|| None::<String>);
    let message = use_state(String::new);
    let messages = use_state(|| Vec::<String>::new());
        
    {
        let current_channel = current_channel.clone();
        let error = error.clone();
        
        use_effect_with_deps(move |_| {
            let stored_channel = LocalStorage::get("selected_channel").unwrap_or(None);
            current_channel.set(stored_channel);
            spawn_local(async move {
                // let response = Request::get(&format!("http://localhost:8080/channel/{}", channel_id))
                //     .send()
                //     .await;

                // match response {
                //     Ok(resp) if resp.ok() => {
                //         match resp.json::<Channel>().await {
                //             Ok(channel_data) => current_channel.set(Some(channel_data)),
                //             Err(_) => error.set("Error loading channel".to_string()),
                //         }
                //     }
                //     _ => {
                //         error.set("Could not load channel".to_string());
                //     }
                // }
            });
            || ()
        }, ());
    }

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
        let messages = messages.clone();
        
        Callback::from(move |_| {
            let message_state = message.clone();
            let messages_state = messages.clone();
            let msg = (*message).clone();
            
            if !msg.is_empty() {
                spawn_local(async move {
                    // let response = Request::post(&format!("http://localhost:8080/channel/{}/message", channel_id))
                    //     .header("Content-Type", "application/json")
                    //     .json(&serde_json::json!({ "content": msg.clone() }))
                    //     .unwrap()
                    //     .send()
                    //     .await;

                    // if let Ok(resp) = response {
                    //     if resp.ok() {
                    //         let mut new_messages = (*messages_state).clone();
                    //         new_messages.push(msg);
                    //         messages_state.set(new_messages);
                    //         message_state.set(String::new());
                    //     }
                    // }
                });
            }
        })
    };

    let on_exit = Callback::from(move |_| {
        LocalStorage::delete("selected_channel");
        window().location().set_href("/channel_list").unwrap();
    });

    match &*current_channel {
        Some(channel) => html! {
            <div class="chat-container">
                <div class="chat-header">
                    <div class="header-left">
                        <button onclick={on_exit} class="exit-button">{"Exit"}</button>
                        <h2 class="channel-title">
                            {format!(
                                "Channel: {}",
                                match &*current_channel {
                                    Some(channel) => channel.clone(),
                                    None => "None".to_string(), // Default message if no channel is selected
                                }
                            )}
                        </h2>
                    </div>
                </div>
                {if !(*error).is_empty() {
                    html! { <div class="error-message">{&*error}</div> }
                } else {
                    html! {}
                }}
                <div class="chat-messages">
                    {for messages.iter().map(|msg| {
                        html! {
                            <div class="message">{msg}</div>
                        }
                    })}
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
        },
        None => html! {
            <div class="loading-container">
                <h2>{"Loading channel..."}</h2>
                {if !(*error).is_empty() {
                    html! { <div class="error-message">{&*error}</div> }
                } else {
                    html! {}
                }}
            </div>
        }
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
        "/channel_create" => html! { <CreateChannel /> },
        "/channel_room" => html! { <ChatRoom /> },
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