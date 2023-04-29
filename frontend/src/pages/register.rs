use common::CreateUserDto;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{api::api_add_user, router::Route, utils::get_input_handler};

#[function_component(Register)]
pub fn register() -> Html {
    let navigator = use_navigator().unwrap();
    let email = use_state(String::new);
    let password = use_state(String::new);
    let err_message = use_state(|| Option::<String>::None);

    let on_email_input = get_input_handler(&email);
    let on_password_input = get_input_handler(&password);

    let on_submit = {
        let navigator = navigator.clone();
        let email = email.clone();
        let password = password.clone();
        let err_message = err_message.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let err_message = err_message.clone();

            event.prevent_default();

            let data = CreateUserDto {
                email: email.to_string(),
                password: password.to_string(),
            };

            spawn_local(async move {
                let res = api_add_user(data).await;

                match res {
                    Ok(_) => {
                        navigator.push(&Route::Login);
                    }
                    Err(e) => err_message.set(Some(e)),
                }
            });
        })
    };

    html! {
        <div class="flex flex-col max-w-md mx-auto">
            <header class="mb-6 text-2xl font-bold text-orange-500">
                <h1>{ "Register" }</h1>
            </header>
            <form onsubmit={on_submit} class="flex flex-col gap-4">
                <div class="flex flex-col gap-1">
                    <label for="email" class="text-slate-500">{ "Email" }</label>
                    <input type="email" oninput={on_email_input} required={true} class="p-2 rounded-md" name="email" id="email" placeholder="Enter your email" />
                </div>
                <div class="flex flex-col gap-1">
                    <label for="password" class="text-slate-500">{ "Password" }</label>
                    <input type="password" oninput={on_password_input} required={true} class="p-2 rounded-md" name="password" id="password" placeholder="Enter your password" />
                </div>
                <div class="mt-2">
                    <button type="submit" class="p-2 bg-orange-500 rounded-md">{ "Submit" }</button>
                </div>
                {if let Some(msg) = err_message.as_ref() {
                    html! {
                        <p class="text-red-500">{msg}</p>
                    }
                } else {
                    html! {}
                }}
            </form>
        </div>
    }
}
