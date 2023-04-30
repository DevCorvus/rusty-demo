use common::UpdateUserPasswordDto;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api::api_update_user_password,
    router::Route,
    store::{set_profile, Store},
    utils::get_input_handler,
};

#[function_component(UpdatePassword)]
pub fn update_password() -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();
    let show = use_state(|| false);
    let password = use_state(String::new);
    let err_message = use_state(|| Option::<String>::None);

    let on_password_input = get_input_handler(&password);

    let onclick = {
        let show = show.clone();
        Callback::from(move |_| show.set(!(*show)))
    };

    let on_submit = {
        let show = show.clone();
        let err_message = err_message.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            let show = show.clone();
            let password = password.clone();
            let err_message = err_message.clone();

            event.prevent_default();

            if let Some(t) = store.token.to_owned() {
                let data = UpdateUserPasswordDto {
                    password: password.to_string(),
                };
                spawn_local(async move {
                    let res = api_update_user_password(t.as_str(), data).await;

                    match res {
                        Ok(_) => {
                            set_profile(None, dispatch);
                            show.set(false);
                            password.set(String::new());
                            err_message.set(None);
                            navigator.replace(&Route::Profile);
                        }
                        Err(e) => {
                            err_message.set(Some(e));
                        }
                    }
                });
            }
        })
    };

    html! {
        if *show {
            <form onsubmit={on_submit} class="flex flex-col gap-1">
                <div class="flex flex-col gap-1">
                    <label for="new_password" class="text-slate-500">{ "New password" }</label>
                    <input type="password" oninput={on_password_input} class="p-2 rounded-md" name="password" id="new_password" placeholder="Enter your new password" />
                </div>
                <div class="flex justify-between text-sm">
                    <button type="submit" class="p-1 transition rounded-md bg-sky-500 hover:bg-sky-400">{"Submit"}</button>
                    <button {onclick} class="transition text-slate-500 hover:text-slate-400">{"Cancel"}</button>
                </div>
                {if let Some(msg) = err_message.as_ref() {
                    html! {
                        <p class="text-red-500">{msg}</p>
                    }
                } else {
                    html! {}
                }}
            </form>
        } else {
            <button {onclick} class="transition text-sky-500 hover:text-sky-400">{"Update password"}</button>
        }
    }
}
