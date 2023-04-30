use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::use_store;

use crate::{
    api::api_delete_user,
    router::Route,
    store::{set_profile, set_token, Store},
};

#[function_component(DeleteAccount)]
pub fn delete_account() -> Html {
    let navigator = use_navigator().unwrap();
    let (store, dispatch) = use_store::<Store>();
    let err_message = use_state(|| Option::<String>::None);

    let on_submit = {
        let err_message = err_message.clone();

        Callback::from(move |event: SubmitEvent| {
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            let err_message = err_message.clone();

            event.prevent_default();

            if let Some(t) = store.token.to_owned() {
                spawn_local(async move {
                    let res = api_delete_user(t.as_str()).await;

                    match res {
                        Ok(_) => {
                            set_profile(None, dispatch.to_owned());
                            set_token(None, dispatch.to_owned());
                            navigator.push(&Route::Home);
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
        <form onsubmit={on_submit} class="flex flex-col items-center gap-1">
            <div>
                <button type="submit" class="transition text-rose-500 hover:text-rose-400">{"Delete account"}</button>
            </div>
            {if let Some(msg) = err_message.as_ref() {
                    html! {
                        <p class="text-red-500">{msg}</p>
                    }
                } else {
                    html! {}
                }}
        </form>
    }
}
