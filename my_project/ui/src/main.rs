use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <script>{ "window.location.assign('https://www.ibm.com')" }</script>
    }
}
