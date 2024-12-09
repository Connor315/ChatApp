use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsCast;
use js_sys::Date;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
    username: String,
    content: String,
    room: String,
    timestamp: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Channel {
    name: String,
    owner: String,
}

#[derive(Properties, PartialEq, Clone)]
struct ChatProps {
    username: String,
    room: String,
}

#[derive(Clone, PartialEq)]
enum AuthStep {
    SelectType,
    Credentials,
    ChannelList,
}

#[derive(Clone, PartialEq)]
enum AuthType {
    Login,
    Register,
}

#[function_component(ChatRoom)]
fn chat_room(props: &ChatProps) -> Html {
    let messages = use_state(Vec::new);
    let ws = use_state(|| None::<WebSocket>);
    let input_value = use_state(String::new);
    let props = props.clone();
    
    {
        let ws = ws.clone();
        let messages = messages.clone();
        
        use_effect_with_deps(
            move |_| {
                let socket = WebSocket::new("ws://localhost:8080/ws").unwrap();
                
                let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        let txt_str = txt.as_string().unwrap();
                        if let Ok(msg) = serde_json::from_str(&txt_str) {
                            messages.set(
                                messages
                                    .iter()
                                    .cloned()
                                    .chain(std::iter::once(msg))
                                    .collect()
                            );
                        }
                    }
                }) as Box<dyn FnMut(MessageEvent)>);

                socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                
                ws.set(Some(socket));
                
                || ()
            },
            (),
        );
    }

    let onsubmit = {
        let ws = ws.clone();
        let input_value = input_value.clone();
        let props = props.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if let Some(socket) = (*ws).as_ref() {
                let msg = Message {
                    username: props.username.clone(),
                    content: (*input_value).clone(),
                    room: props.room.clone(),
                    timestamp: Date::now(),
                };
                
                if let Ok(msg_str) = serde_json::to_string(&msg) {
                    let _ = socket.send_with_str(&msg_str);
                    input_value.set(String::new());
                }
            }
        })
    };

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                input_value.set(input.value());
            }
        })
    };

    let logout = {
        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            storage.remove_item("current_user").unwrap();
            storage.remove_item("selected_channel").unwrap();
            window.location().reload().unwrap();
        })
    };

    html! {
        <div class="chat-container">
            <div class="chat-header">
                <h2>{format!("Channel: {}", props.room)}</h2>
                <button onclick={logout} class="logout-button">{"Logout"}</button>
            </div>
            <div class="messages">
                {messages.iter().map(|msg: &Message| {
                    let date = Date::new(&JsValue::from(msg.timestamp));
                    let options = js_sys::Object::new();
                    html! {
                        <div class="message">
                            <span class="username">{&msg.username}</span>
                            <span class="content">{&msg.content}</span>
                            <span class="timestamp">
                                {date.to_locale_string("en-US", &options).as_string().unwrap_or_default()}
                            </span>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            
            <form {onsubmit} class="input-form">
                <input
                    type="text"
                    value={(*input_value).clone()}
                    {oninput}
                    placeholder="Type a message..."
                    class="message-input"
                />
                <button type="submit" class="send-button">{"Send"}</button>
            </form>
        </div>
    }
}

#[function_component(AuthForm)]
fn auth_form() -> Html {
    let auth_step = use_state(|| AuthStep::SelectType);
    let auth_type = use_state(|| AuthType::Login);
    let username = use_state(String::new);
    let password = use_state(String::new);
    let error_message = use_state(String::new);
    let channels = use_state(Vec::new);

    {
        let channels = channels.clone();
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                
                // Predefined channels
                let default_channels = vec![
                    Channel { name: "Channel 1".to_string(), owner: "Corner".to_string() },
                    Channel { name: "Channel 2".to_string(), owner: "Torin".to_string() },
                    Channel { name: "Channel 3".to_string(), owner: "Chen".to_string() },
                    Channel { name: "Channel 4".to_string(), owner: "Mr.Gao".to_string() },
                    Channel { name: "Channel 5".to_string(), owner: "Mr.Wang".to_string() },
                    Channel { name: "Channel 6".to_string(), owner: "Miss Tuo".to_string() },
                ];

                let channel_list = if let Ok(Some(stored_channels)) = storage.get_item("channels") {
                    serde_json::from_str(&stored_channels).unwrap_or(default_channels.clone())
                } else {
                    if let Ok(channels_json) = serde_json::to_string(&default_channels) {
                        storage.set_item("channels", &channels_json).unwrap();
                    }
                    default_channels
                };
                
                channels.set(channel_list);

                // Initialize default user if no users exist
                if storage.get_item("users").unwrap_or(None).is_none() {
                    let default_users = vec![
                        User {
                            username: "demo_user".to_string(),
                            password: "password123".to_string(),
                        },
                    ];
                    if let Ok(users_json) = serde_json::to_string(&default_users) {
                        storage.set_item("users", &users_json).unwrap();
                    }
                }
                
                || ()
            },
            (),
        );
    }

    let handle_auth = {
        let username = username.clone();
        let password = password.clone();
        let auth_type = auth_type.clone();
        let error_message = error_message.clone();
        let auth_step = auth_step.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if (*username).is_empty() || (*password).is_empty() {
                error_message.set("Username and password are required".to_string());
                return;
            }

            let window = web_sys::window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            
            let user = User {
                username: (*username).clone(),
                password: (*password).clone(),
            };

            match *auth_type {
                AuthType::Login => {
                    if let Ok(Some(stored_users)) = storage.get_item("users") {
                        if let Ok(users) = serde_json::from_str::<Vec<User>>(&stored_users) {
                            if let Some(stored_user) = users.iter().find(|u| u.username == *username) {
                                if stored_user.password == *password {
                                    storage.set_item("current_user", &user.username).unwrap();
                                    auth_step.set(AuthStep::ChannelList);
                                    return;
                                }
                            }
                        }
                    }
                    error_message.set("Invalid username or password. Please register if you don't have an account.".to_string());
                }
                AuthType::Register => {
                    let mut users = if let Ok(Some(stored_users)) = storage.get_item("users") {
                        serde_json::from_str(&stored_users).unwrap_or_else(|_| Vec::new())
                    } else {
                        Vec::new()
                    };

                    if users.iter().any(|u: &User| u.username == *username) {
                        error_message.set("Username already exists".to_string());
                        return;
                    }

                    users.push(user);
                    if let Ok(users_json) = serde_json::to_string(&users) {
                        storage.set_item("users", &users_json).unwrap();
                        auth_step.set(AuthStep::ChannelList);
                    }
                }
            }
        })
    };

    match *auth_step {
        AuthStep::SelectType => {
            html! {
                <div class="auth-container">
                    <div class="auth-type-selection">
                        <h2>{"Welcome to Chat App"}</h2>
                        <div class="auth-buttons">
                            <button 
                                onclick={
                                    let auth_type = auth_type.clone();
                                    let auth_step = auth_step.clone();
                                    Callback::from(move |_| {
                                        auth_type.set(AuthType::Login);
                                        auth_step.set(AuthStep::Credentials);
                                    })
                                }
                                class="auth-button"
                            >
                                {"Login"}
                            </button>
                            <button 
                                onclick={
                                    let auth_type = auth_type.clone();
                                    let auth_step = auth_step.clone();
                                    Callback::from(move |_| {
                                        auth_type.set(AuthType::Register);
                                        auth_step.set(AuthStep::Credentials);
                                    })
                                }
                                class="auth-button"
                            >
                                {"Register"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
        AuthStep::Credentials => {
            html! {
                <div class="auth-container">
                    <div class="auth-form">
                        <h2>{if *auth_type == AuthType::Login { "Login" } else { "Register" }}</h2>
                        {if !(*error_message).is_empty() {
                            html! {
                                <div class="error-message">{&*error_message}</div>
                            }
                        } else {
                            html! {}
                        }}
                        <form onsubmit={handle_auth}>
                            <input
                                type="text"
                                placeholder="Username"
                                value={(*username).clone()}
                                onchange={
                                    let username = username.clone();
                                    Callback::from(move |e: Event| {
                                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
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
                                        if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                            password.set(input.value());
                                        }
                                    })
                                }
                            />
                            <button type="submit">
                                {if *auth_type == AuthType::Login { "Login" } else { "Register" }}
                            </button>
                            <button 
                                onclick={
                                    let auth_step = auth_step.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        auth_step.set(AuthStep::SelectType);
                                    })
                                }
                                class="back-button"
                            >
                                {"Back"}
                            </button>
                        </form>
                    </div>
                </div>
            }
        }
        AuthStep::ChannelList => {
            html! {
                <div class="channel-list-container">
                    <h2>{"Available Channels"}</h2>
                    <div class="channel-list">
                        {channels.iter().map(|channel| {
                            let username_clone = (*username).clone();
                            html! {
                                <div class="channel-item">
                                    <div class="channel-info">
                                        <span class="channel-name">{&channel.name}</span>
                                        <span class="channel-owner">{format!("Owner: {}", &channel.owner)}</span>
                                    </div>
                                    <button 
                                        onclick={
                                            let channel_name = channel.name.clone();
                                            Callback::from(move |_| {
                                                let window = web_sys::window().unwrap();
                                                let storage = window.local_storage().unwrap().unwrap();
                                                storage.set_item("selected_channel", &channel_name).unwrap();
                                                storage.set_item("current_user", &username_clone).unwrap();
                                                window.location().reload().unwrap();
                                            })
                                        }
                                        class="enter-button"
                                    >
                                        {"Enter"}
                                    </button>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let username = use_state(String::new);
    let room = use_state(String::new);
    let joined = use_state(|| false);

    {
        let username = username.clone();
        let room = room.clone();
        let joined = joined.clone();
        
        use_effect_with_deps(
            move |_| {
                let window = web_sys::window().unwrap();
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(current_user)) = storage.get_item("current_user") {
                        if let Ok(Some(selected_channel)) = storage.get_item("selected_channel") {
                            username.set(current_user);
                            room.set(selected_channel);
                            joined.set(true);
                        }
                    }
                }
                || ()
            },
            (),
        );
    }

    if *joined {
        html! {
            <ChatRoom username={(*username).clone()} room={(*room).clone()} />
        }
    } else {
        html! {
            <AuthForm />
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}