use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("text-orange-500", "text-3xl")}>
            <h1>{ "Rusty demo" }</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
