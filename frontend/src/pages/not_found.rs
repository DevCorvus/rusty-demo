use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="text-2xl">
            <h1>{ "Not found" }</h1>
        </div>
    }
}
