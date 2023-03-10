use crate::stats::BoardState;
use log;
use std::rc::Rc;
use yew::prelude::*;

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

pub struct StickerClickEvent {
    pub row: usize,
    pub col: usize,
}

#[derive(Properties, PartialEq)]
pub struct StickerBoardProps {
    pub board: BoardState,
    pub on_click: Callback<StickerClickEvent>,
}

// impl Reducible for StickerBoardState {
// 	type Action = StickerBoardClick;

// 	fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
// 		log::info!("reducing! flipping state {} at {},{}", self.0.is_sticker(action.row,action.col), action.row, action.col);
// 		Self(self.0.toggle(action.row, action.col)).into()
// 	}
// }

#[function_component]
pub fn StickerBoard(props: &StickerBoardProps) -> Html {
    let cells = (0..16).map(move |idx| {
			let r = (idx/4) as usize;
			let c = (idx%4) as usize;

			let toggle_sticker = {
				let on_click = props.on_click.clone();
				Callback::from(move |_|{
					log::info!("clicked sticker at {},{}", r, c); 
					on_click.emit(StickerClickEvent{row: r, col: c})
				})
			};
			html!{
				<StickerSlot row={r} col={c} is_checked={props.board.is_sticker(r,c)} on_click={toggle_sticker} />
			}
		});
    html! {
        <div class="sticker-board">
        {for cells}
        </div>
    }
}
