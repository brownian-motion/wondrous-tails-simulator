use std::rc::Rc;
use yew::prelude::*;
use crate::stats::BoardState;
use log;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct StickerSlotProps {
	pub row: usize,
	pub col: usize,
	pub is_checked: bool,
	pub on_click: Callback<MouseEvent>,
}

#[function_component]
pub fn StickerSlot(props: &StickerSlotProps) -> Html {
	html! {
		<div onclick={props.on_click.clone()} class="sticker-slot">
		<img src={format!("img/sticker-{}.png", props.row * 8 + props.col)} class={if props.is_checked {"sticker-visible"} else {"sticker-invisible"} }/>
		</div>
	}
}

struct StickerBoardClick {
	pub row: usize,
	pub col: usize,
}


struct StickerBoardState(BoardState);
impl Reducible for StickerBoardState {
	type Action = StickerBoardClick;

	fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
		log::info!("reducing! flipping state {} at {},{}", self.0.is_sticker(action.row,action.col), action.row, action.col);
		Self(self.0.toggle(action.row, action.col)).into()
	}
}

#[function_component]
pub fn StickerBoard() -> Html {
	let board_state = use_reducer(|| StickerBoardState(BoardState::empty()));
	
	let cells = (0..16).map(move |idx| {
			let r = (idx/4) as usize;
			let c = (idx%4) as usize;

			let toggle_sticker = {
				let board_state = board_state.clone();
				Callback::from(move |_|{
					log::info!("dispatching at {},{}", r, c); 
					board_state.dispatch(StickerBoardClick{row: r, col: c})
				})
			};
			html!{
				<StickerSlot row={r} col={c} is_checked={board_state.0.is_sticker(r,c)} on_click={toggle_sticker} />
			}
		});
	html!{
		<div class="sticker-board">
		{for cells}
		</div>
	}
}