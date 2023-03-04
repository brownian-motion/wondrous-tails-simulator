use yew::prelude::*;
use crate::components::*;

#[function_component]
pub fn App() -> Html {
    html! {
    	<div class="root">
    		<Row><img src="img/journal.png"/><h2>{ "Wondrous Tails Simulator" }</h2></Row>
    		<Row>
        		<StickerBoard />
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