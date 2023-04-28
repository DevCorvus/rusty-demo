use yew::prelude::*;
use yew_router::prelude::*;

use super::pages::{
    home::Home, login::Login, not_found::NotFound, profile::Profile, register::Register,
};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! {
            <Home />
        },
        Route::Login => html! {
            <Login />
        },
        Route::Register => html! {
            <Register />
        },
        Route::Profile => html! {
            <Profile />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}
