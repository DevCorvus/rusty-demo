use crate::{components::log_out::LogOut, router::Route, store::Store};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let (store, _) = use_store::<Store>();
    html! {
        <nav class="flex justify-between p-6 text-orange-500">
            <Link<Route> to={Route::Home} classes="font-bold transition focus:text-orange-400 hover:text-orange-400">
                { "Rusty demo" }
            </Link<Route>>
            <ul class="flex items-center gap-4">
                if store.token.is_some() {
                    <li>
                        <Link<Route> to={Route::Profile} classes="transition focus:text-orange-400 hover:text-orange-400">
                            { "Profile" }
                        </Link<Route>>
                    </li>
                    <li>
                        <LogOut />
                    </li>
                } else {
                    <li>
                        <Link<Route> to={Route::Register} classes="transition focus:text-orange-400 hover:text-orange-400">
                            { "Register" }
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Login} classes="transition focus:text-orange-400 hover:text-orange-400">
                            { "Login" }
                        </Link<Route>>
                    </li>
                }
            </ul>
        </nav>
    }
}
