use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Event}; 
use yew::prelude::*;
use gloo::utils::window;
use gloo_storage::{Storage, LocalStorage};
use serde::Serialize;
use web_sys::WebSocket;
use wasm_bindgen::JsCast;
use web_sys::MessageEvent;
use wasm_bindgen::closure::Closure;
use gloo::timers::future::TimeoutFuture;
use chrono;

#[derive(PartialEq, Clone, Debug, Deserialize)] 
struct Channel {
    id: u32,
    name: String,
    owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ChatMessage {
    username: String,
    message: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageRequest {
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserStatus {
    pub username: String,
    pub status: String,
    pub timestamp: String,
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

fn setup_websocket(
    channel_name: String,
    messages: UseStateHandle<Vec<ChatMessage>>,
    ws_state: UseStateHandle<Option<WebSocket>>,
) -> Option<WebSocket> {
    let ws_url = format!("ws://localhost:8080/channel/ws/{}", channel_name);
    
    match WebSocket::new(&ws_url) {
        Ok(websocket) => {
            // Set up open handler
            let onopen = Closure::wrap(Box::new(|| {
                gloo::console::log!("WebSocket connected");
            }) as Box<dyn FnMut()>);
            websocket.set_onopen(Some(onopen.as_ref().unchecked_ref()));
            onopen.forget();

            // Set up close handler
            let ws_url = ws_url.clone();
            let ws_state_reconnect = ws_state.clone();
            
            let onclose = Closure::wrap(Box::new(move |_| {
                gloo::console::log!("WebSocket closed, attempting to reconnect...");
                
                let ws_url = ws_url.clone();  // Clone inside closure to make it FnMut
                let ws_state = ws_state_reconnect.clone();  // Clone inside closure
                
                spawn_local(async move {
                    TimeoutFuture::new(3_000).await;
                    if let Ok(new_ws) = WebSocket::new(&ws_url) {
                        ws_state.set(Some(new_ws));
                    }
                });
            }) as Box<dyn FnMut(JsValue)>);
            websocket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
            onclose.forget();

            // Set up message handler
            // Only clone messages once for the message handler
            let messages_handler = messages.clone();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                gloo::console::log!("onmessage triggered");
                if let Some(text) = event.data().as_string() {
                    if text == "ping" {
                        return;
                    }

                    let mut current_messages = (*messages_handler).clone();
                    gloo::console::log!("Current messages before update:", current_messages.len());

                    gloo::console::log!("Received message (raw):", &text);

                    
                    let new_message = if text.trim().contains(" left the chat") || text.trim().contains(" joined the chat"){
                        
                        ChatMessage {
                            username: "System".to_string(),
                            message: text,
                            timestamp: chrono::Local::now()
                                .format("%Y-%m-%d %H:%M")
                                .to_string(),
                        }
                    } else if let Some((username, msg)) = text.split_once(':') {
                        ChatMessage {
                            username: username.to_string(),
                            message: msg.trim().to_string(),
                            timestamp: chrono::Local::now()
                                .format("%Y-%m-%d %H:%M:%S%.3f")
                                .to_string(),
                        }
                    } else {
                        return;
                    };
                    
                    // gloo::console::log!("set up 1");
                    current_messages.push(new_message);
                    // gloo::console::log!("Messages after update:", current_messages.len());
                    messages_handler.set(current_messages);
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            websocket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();

            Some(websocket)
        }
        Err(err) => {
            gloo::console::log!("WebSocket connection failed:", err);
            None
        }
    }
}

fn set_onmessage(messages: UseStateHandle<Vec<ChatMessage>>) -> Option<Closure<dyn FnMut(MessageEvent)>> {
    let messages_handler = messages.clone();
    let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(text) = event.data().as_string() {
            if text == "ping" {
                return;
            }

            let mut current_messages = (*messages_handler).clone();
            // gloo::console::log!("Current messages before update:", current_messages.len());
            
            let new_message = if text.contains(" joined the chat") || text.contains(" left the chat"){
                ChatMessage {
                    username: "System".to_string(),
                    message: text,
                    timestamp: chrono::Local::now()
                        .format("%Y-%m-%d %H:%M")
                        .to_string(),
                }
            } else if let Some((username, msg)) = text.split_once(':') {
                ChatMessage {
                    username: username.to_string(),
                    message: msg.trim().to_string(),
                    timestamp: chrono::Local::now()
                        .format("%Y-%m-%d %H:%M:%S%.3f")
                        .to_string(),
                }
            } else {
                return;
            };

            // gloo::console::log!("Adding new message");
            current_messages.push(new_message);
            // gloo::console::log!("Messages after update:", current_messages.len());
            messages_handler.set(current_messages);
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    Some(onmessage)
}

#[function_component(ChatRoom)]
fn chat_room() -> Html {
    let error = use_state(|| String::new());
    let current_channel = use_state(|| None::<Channel>);
    let message = use_state(String::new);
    let ws = use_state(|| None::<WebSocket>);
    let messages = use_state(|| Vec::<ChatMessage>::new());
    let history_fetch = use_state(|| false);
    let ws_setup = use_state(|| false);
    let user_statuses = use_state(|| Vec::<UserStatus>::new());

    // Initial channel setup
    {
        let current_channel = current_channel.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let stored_channel_name: Option<String> = LocalStorage::get("selected_channel").ok();
                    
                    if let Some(channel_name) = stored_channel_name {
                        let response = Request::get(&format!("http://localhost:8080/channel/enter/{}", channel_name))
                            .send()
                            .await;

                        match response {
                            Ok(resp) => {
                                match resp.status() {
                                    200 => {
                                        current_channel.set(Some(Channel {
                                            id: 0,
                                            name: channel_name,
                                            owner: String::new()
                                        }));
                                    },
                                    401 => {
                                        error.set("Please log in first".to_string());
                                        window().location().set_href("/login").unwrap();
                                    },
                                    404 => {
                                        error.set(format!("Channel '{}' not found", channel_name));
                                    },
                                    _ => {
                                        error.set("Error connecting to channel".to_string());
                                    }
                                }
                            },
                            Err(e) => {
                                error.set(format!("Network error: {}", e));
                            }
                        }
                    } else {
                        error.set("No channel selected".to_string());
                    }
                });
                || ()
            },
            (),
        );
    }

    // Fetch chat history
    {
        let history_fetch = history_fetch.clone();
        let messages = messages.clone();
        let error = error.clone();
        let channel_state = current_channel.clone();

        // In the history fetch effect
        use_effect_with_deps(
            move |_| {
                if let Some(channel) = (*channel_state).clone() {
                    spawn_local(async move {
                        // gloo::console::log!("=== FETCHING CHAT HISTORY ===");
                        // gloo::console::log!("Channel name:", &channel.name);
                        
                        let response = Request::get(&format!("http://localhost:8080/channel/history/{}", channel.name))
                            .send()
                            .await;

                        match response {
                            Ok(resp) => {
                                // gloo::console::log!("Response status:", resp.status());
                                match resp.json::<Vec<ChatMessage>>().await {
                                    Ok(history) => {
                                        // gloo::console::log!("Raw history count:", history.len());
                                        // for msg in &history {
                                        //     gloo::console::log!("History message:", 
                                        //         format!("User: {}, Content: {}", msg.username, msg.message));
                                        // }
                                        messages.set(history);
                                        history_fetch.set(true);
                                        gloo::console::log!("History set complete");
                                    }
                                    Err(e) => {
                                        gloo::console::log!("Failed to parse history:", e.to_string());
                                        error.set(format!("Failed to parse history: {}", e));
                                    }
                                }
                            }
                            Err(e) => {
                                gloo::console::log!("Failed to fetch history:", e.to_string());
                                error.set(format!("Failed to fetch history: {}", e));
                            }
                        }
                    });
                }
                || ()
            },
            current_channel.clone(),
        );
    }

    // WebSocket setup
    {
        let messages_c1 = messages.clone();
        let history_fetch_clone = history_fetch.clone();
        let ws = ws.clone();
        let channel_state = current_channel.clone();
        let ws_setup_clone = ws_setup.clone();

        use_effect_with_deps(
            move |_| {
                if *history_fetch_clone {
                    if let Some(channel) = (*channel_state).clone() {
                        if let Some(websocket) = setup_websocket(channel.name, messages_c1.clone(), ws.clone()) {
                            // Setup ping
                            let ws_clone = websocket.clone();
                            ws_setup_clone.set(true);
                            spawn_local(async move {
                                loop {
                                    TimeoutFuture::new(30_000).await;
                                    if ws_clone.send_with_str("ping").is_err() {
                                        break;
                                    }
                                }
                            });

                            ws.set(Some(websocket));
                        }
                    }
                }
                
                || ()
            },
            (current_channel.clone(), history_fetch.clone()),
        );
    }

    {
        let messages_c1 = messages.clone();
        let ws_setup_clone = ws_setup.clone();
        let ws_clone = ws.clone();

        use_effect_with_deps(
            move |_| {
                if *ws_setup_clone {
                    if let Some(ws_onmessage) = set_onmessage(messages_c1.clone()) {
                        if let Some(webs) = &*ws_clone {
                            webs.set_onmessage(Some(ws_onmessage.as_ref().unchecked_ref()));
                            ws_onmessage.forget();
                        }
                    }
                }
                || ()
            },
            (ws_setup.clone(), messages.clone()), // Dependencies
        );
    }

    {
        let user_statuses = user_statuses.clone();
        let channel_state = current_channel.clone();

        use_effect_with_deps(
            move |_| {
                if let Some(channel) = (*channel_state).clone() {
                    let channel_name = channel.name.clone();

                    spawn_local(async move {
                        loop {
                            let user_statuses = user_statuses.clone();
                            let url = format!("http://localhost:8080/user/status/{}", channel_name);

                            match Request::get(&url).send().await {
                                Ok(response) => {
                                    if let Ok(status_list) = response.json::<Vec<UserStatus>>().await {
                                        user_statuses.set(status_list);
                                    } else {
                                        gloo::console::log!("Failed to parse user statuses");
                                    }
                                }
                                Err(_) => {
                                    gloo::console::log!("Failed to fetch user statuses");
                                }
                            }

                            TimeoutFuture::new(5_000).await;
                        }
                    });
                }

                || {}
            },
            current_channel.clone(),
        );
    }

    // Message sending
    let send_message = {
        let message = message.clone();
        let ws = ws.clone();
        move || {
            let msg = (*message).clone();
            if !msg.is_empty() {
                if let Some(websocket) = &*ws {
                    // gloo::console::log!("Sending message:", &msg);
                    if websocket.send_with_str(&msg).is_ok() {
                        message.set(String::new());
                    }
                }
            }
        }
    };

    let on_message_change = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                message.set(input.value());
            }
        })
    };

    let on_keypress = {
        let send_message = send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                send_message();
            }
        })
    };

    let on_send = {
        let send_message = send_message.clone();
        Callback::from(move |_| send_message())
    };

    let cur_channel = current_channel.clone();

    let on_exit = Callback::from(move |_| {
        cur_channel.set(None);
        history_fetch.set(false);
        match LocalStorage::set("selected_channel", "") {
            Ok(_) => gloo::console::log!("Channel clear"),
            Err(_) => gloo::console::log!("Error"),
        }
        window().location().set_href("/channel_list").unwrap();
    });

    match &*current_channel {
        Some(channel) => html! {
            <div class="chat-container">
                <div class="chat-header">
                    <button onclick={on_exit} class="exit-button">{"Exit"}</button>
                    <h2 class="channel-title">{format!("Channel: {}", channel.name)}</h2>
                </div>
                <div class="chat-layout">
                    <div class="chat-main">
                        {if !error.is_empty() {
                            html! { <div class="error-message">{&*error}</div> }
                        } else {
                            html! {}
                        }}
                        <div class="chat-messages">
                            {for (*messages).iter()
                                .filter(|msg| !msg.message.contains("ping"))
                                .map(|msg| {
                                    if msg.username == "System" {
                                        html! {
                                            <div class="message system-message">
                                                <div class="content">{&msg.message}</div>
                                                <span class="timestamp">{&msg.timestamp}</span>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <div class="message" key={format!("{}-{}", msg.timestamp, msg.username)}>
                                                <div class="message-header">
                                                    <span class="username">{&msg.username}</span>
                                                    <span class="timestamp">{&msg.timestamp}</span>
                                                </div>
                                                <div class="content">{&msg.message}</div>
                                            </div>
                                        }
                                    }
                                })
                            }
                        </div>
                        <div class="chat-input">
                            <input
                                type="text"
                                placeholder="Type a message..."
                                value={(*message).clone()}
                                oninput={on_message_change}
                                onkeypress={on_keypress}
                                class="message-input"
                            />
                            <button onclick={on_send} class="send-button">{"Send"}</button>
                        </div>
                    </div>
                    <div class="user-list">
                        <h3>{"User Status"}</h3>
                        <div class="user-list-content">
                            {for (*user_statuses).iter().map(|user| {
                                html! {
                                    <div class="user-item">
                                        <span class={classes!("status-indicator", if user.status=="Online" { "online" } else { "offline" })}></span>
                                        <span class="username">{&user.username}</span>
                                    </div>
                                }
                            })}
                        </div>
                    </div>
                </div>
            </div>
        },
        None => html! {
            <div class="loading-container">
                <h2>{"Loading channel..."}</h2>
                {if !error.is_empty() {
                    html! { <div class="error-message">{&*error}</div> }
                } else {
                    html! {}
                }}
            </div>
        },
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
