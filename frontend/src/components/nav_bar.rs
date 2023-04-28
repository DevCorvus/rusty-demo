use crate::{components::log_out::LogOut, router::Route, store::Store};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let (store, _) = use_store::<Store>();
    html! {
        <nav class="flex justify-between p-6 text-orange-500">
            <Link<Route> to={Route::Home}>
                <span class="font-bold">{ "Rusty demo" }</span>
            </Link<Route>>
            <ul class="flex items-center gap-4">
                if store.token.is_some() {
                    <li>
                        <Link<Route> to={Route::Profile}>{ "Profile" }</Link<Route>>
                    </li>
                    <li>
                        <LogOut />
                    </li>
                } else {
                    <li>
                        <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Login}>{ "Login" }</Link<Route>>
                    </li>
                }
            </ul>
        </nav>
    }
}
