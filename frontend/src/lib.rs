use gloo::net::http::Request;
use yew::prelude::*;
use wasm_bindgen::prelude::*; // For #[wasm_bindgen] and related macros
// use wasm_bindgen_futures::spawn_local; // To handle async futures in WASM
use web_sys::{HtmlInputElement, Event}; // For working with DOM elements
// use js_sys::JsCast; // For .dyn_into() conversions
use serde::{Serialize, Deserialize}; // For serializing/deserializing JSON

#[function_component(Index)]
fn index() -> Html {
    html! {
        
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Index>::new().render();
}