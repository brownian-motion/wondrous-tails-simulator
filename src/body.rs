use yew::prelude::*;
use crate::stats::BoardState;
use crate::components::*;

#[function_component]
pub fn App() -> Html {
    let board = use_state_eq(|| BoardState::empty());
    let on_sticker_click = {
        let board = board.clone();
        Callback::from(move |event: StickerClickEvent| {
            let new_board = board.toggle(event.row, event.col);
            board.set(new_board);
        })
    };
    html! {
    	<div class="root">
    		<Row><img src="img/journal.png"/><h2>{ "Wondrous Tails Simulator" }</h2></Row>
    		<Row>
        		<StickerBoard board={*board} on_click={on_sticker_click} />
        		<Col>
        			<StatsListing stats_type={StatsType::None} percent={59.2f32} />
        			<StatsListing stats_type={StatsType::Bronze} percent={59.2f32} />
        			<StatsListing stats_type={StatsType::Silver} percent={59.2f32} />
        			<StatsListing stats_type={StatsType::Gold} percent={59.2f32} />
        		</Col>
        	</Row>
        </div>
    }
}