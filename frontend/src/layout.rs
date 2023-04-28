use super::components::nav_bar::NavBar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div class="container flex flex-col justify-center mx-auto">
            <NavBar />
            <main class="p-6">
                { for props.children.iter() }
            </main>
        </div>
    }
}
