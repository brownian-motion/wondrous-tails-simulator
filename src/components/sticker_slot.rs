use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct StickerSlotProps {
	// TODO: sticker image!
	pub image_path: AttrValue,
}

#[function_component]
pub fn StickerSlot(props: &StickerSlotProps) -> Html {
	let is_checked = use_state(|| false);
	let on_click = {
		let is_checked = is_checked.clone();
		Callback::from(move |_| {
			is_checked.set(!*is_checked)
		})
	};
	html! {
		<div onclick={on_click} class="sticker-slot">
		 	<img src={props.image_path.clone()} class={if *is_checked {"sticker-visible"} else {"sticker-invisible"} }/>
		</div>
	}
}

#[function_component]
pub fn StickerBoard() -> Html {
	html!{
		<div class="sticker-board">
			<StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" />
			<StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" />
			<StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" />
			<StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" /><StickerSlot image_path="img/journal.png" />
		</div>
	}
}