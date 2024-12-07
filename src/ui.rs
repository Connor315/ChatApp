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

#[derive(Properties, PartialEq, Clone)]
struct ChatProps {
    username: String,
    room: String,
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

    html! {
        <div class="chat-container">
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

#[function_component(App)]
fn app() -> Html {
    let username = use_state(String::new);
    let room = use_state(String::new);
    let joined = use_state(|| false);
    
    let onsubmit = {
        let username = username.clone();
        let room = room.clone();
        let joined = joined.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !(*username).is_empty() && !(*room).is_empty() {
                joined.set(true);
            }
        })
    };

    if *joined {
        html! {
            <ChatRoom username={(*username).clone()} room={(*room).clone()} />
        }
    } else {
        html! {
            <div class="join-form">
                <form {onsubmit}>
                    <input
                        type="text"
                        placeholder="Username"
                        onchange={
                            let username = username.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    username.set(input.value());
                                }
                            })
                        }
                        class="username-input"
                    />
                    <input
                        type="text"
                        placeholder="Room"
                        onchange={
                            let room = room.clone();
                            Callback::from(move |e: Event| {
                                if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                                    room.set(input.value());
                                }
                            })
                        }
                        class="room-input"
                    />
                    <button type="submit" class="join-button">{"Join Chat"}</button>
                </form>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}