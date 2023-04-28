use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="flex justify-between p-6 text-orange-500">
            <Link<Route> to={Route::Home}>
                <span class="font-bold">{ "Rusty demo" }</span>
            </Link<Route>>
            <ul class="flex items-center gap-4">
                <li>
                    <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Login}>{ "Login" }</Link<Route>>
                </li>
            </ul>
        </nav>
    }
}
