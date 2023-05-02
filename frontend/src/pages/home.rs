use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::Store;

#[function_component(Home)]
pub fn home() -> Html {
    let (store, _) = use_store::<Store>();

    html! {
        <div class="text-2xl text-slate-100">
            {if store.token.is_some() {
                html! {
                    <h1>{ "You are authenticated!" }</h1>
                }
            } else {
                html! {
                    <h1>{ "You are not authenticated .(" }</h1>
                }
            }}
        </div>
    }
}
