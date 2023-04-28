use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::{Callback, InputEvent, UseStateHandle};

pub fn get_input_handler(input: &UseStateHandle<String>) -> Callback<InputEvent> {
    let input = input.clone();
    Callback::from(move |event: InputEvent| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        input.set(value);
    })
}
