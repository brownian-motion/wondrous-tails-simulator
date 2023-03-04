use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div styles="margin: auto; width=400; border: 1px solid black;">
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}