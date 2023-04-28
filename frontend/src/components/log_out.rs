use crate::{
    router::Route,
    store::{set_token, Store},
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(LogOut)]
pub fn log_out() -> Html {
    let navigator = use_navigator().unwrap();
    let (_, dispatch) = use_store::<Store>();

    let onclick = {
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            let dispatch = dispatch.clone();

            set_token(None, dispatch);
            navigator.push(&Route::Home);
        })
    };

    html! {
        <button {onclick}>{ "Logout" }</button>
    }
}
