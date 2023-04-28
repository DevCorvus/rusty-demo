use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api::api_login,
    router::Route,
    store::{set_token, Store},
    utils::get_input_handler,
};

#[function_component(Login)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<Store>();
    let email = use_state(String::new);
    let password = use_state(String::new);

    let on_email_input = get_input_handler(&email);
    let on_password_input = get_input_handler(&password);

    let on_submit = {
        let navigator = navigator.clone();
        let dispatch = dispatch.clone();
        let email = email.clone();
        let password = password.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();

            event.prevent_default();

            let data = json!({
                "email": email.to_string(),
                "password": password.to_string(),
            });

            spawn_local(async move {
                let res = api_login(data.to_string().as_str()).await.unwrap();
                set_token(Some(res.token), dispatch);
                navigator.push(&Route::Home);
            });
        })
    };

    html! {
        <div class="flex flex-col max-w-md mx-auto">
            <header class="mb-6 text-2xl font-bold text-orange-500">
                <h1>{ "Login" }</h1>
            </header>
            <form onsubmit={on_submit} class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <label for="email" class="text-slate-500">{ "Email" }</label>
                    <input type="email" oninput={on_email_input} class="p-2 rounded-md" name="email" id="email" placeholder="Enter your email" />
                </div>
                <div class="flex flex-col gap-1">
                    <label for="password" class="text-slate-500">{ "Password" }</label>
                    <input type="password" oninput={on_password_input} class="p-2 rounded-md" name="password" id="password" placeholder="Enter your password" />
                </div>
                <div class="mt-2">
                    <button type="submit" class="p-2 bg-orange-500 rounded-md">{ "Submit" }</button>
                </div>
            </form>
        </div>
    }
}
