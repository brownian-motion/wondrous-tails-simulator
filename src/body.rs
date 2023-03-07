use yew::prelude::*;
use crate::stats::{BoardMatchCounter, BoardState, simulate_until_9_stickers};
use crate::components::*;

#[function_component]
pub fn App() -> Html {
    let board = use_state_eq(|| BoardState::empty());
    let stats = use_state_eq(|| BoardMatchCounter::empty());
    let on_sticker_click = {
        let board = board.clone();
        let stats = stats.clone();
        Callback::from(move |event: StickerClickEvent| {
            let new_board = board.toggle(event.row, event.col);
            board.set(new_board);
            if new_board.count_stickers() < 3 || new_board.count_stickers() > 9 {
                stats.set(BoardMatchCounter::empty());
            } else {
                let sim_result = simulate_until_9_stickers(new_board).unwrap_or(BoardMatchCounter::empty());
                stats.set(sim_result);
            }
        })
    };
    html! {
    	<div class="root">
    		<Row><img src="img/journal.png"/><h2>{ "Wondrous Tails Simulator" }</h2></Row>
    		<Row>
        		<StickerBoard board={*board} on_click={on_sticker_click} />
        		<StatsListingTable stats={*stats} />
        	</Row>
        </div>
    }
}