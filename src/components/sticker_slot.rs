use yew::prelude::*;

#[derive(Properties, PartialEq, Debug)]
pub struct StickerSlotProps {
	// TODO: sticker image!
}

#[function_component]
pub fn StickerSlot(_props: &StickerSlotProps) -> Html {
	let is_checked = use_state(|| false);
	let on_click = {
		let is_checked = is_checked.clone();
		Callback::from(move |_| {
			is_checked.set(!*is_checked)
		})
	};
	html! {
		<div onclick={on_click} class="sticker-slot">
		 	{"{Sticker ["}{if *is_checked {"x"}else{"_"}}{"]}"}
		</div>
	}
}

#[function_component]
pub fn StickerBoard() -> Html {
	html!{
		<div class="sticker-board">
			<StickerSlot /><StickerSlot /><StickerSlot /><StickerSlot />
			<StickerSlot /><StickerSlot /><StickerSlot /><StickerSlot />
			<StickerSlot /><StickerSlot /><StickerSlot /><StickerSlot />
			<StickerSlot /><StickerSlot /><StickerSlot /><StickerSlot />
		</div>
	}
}