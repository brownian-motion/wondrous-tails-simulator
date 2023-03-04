use yew::prelude::*;
use crate::components::*;

#[function_component]
pub fn App() -> Html {
    html! {
    	<div>
    		<h3>{"Sticker Board" }</h3>
        	<StickerBoard />
        </div>
    }
}