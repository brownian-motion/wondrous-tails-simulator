use yew::prelude::*;
use super::rowcol::{Row, RowColProps};

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum StatsType {
	Gold, Silver, Bronze, None
}

impl StatsType {
	fn image(self) -> AttrValue {
		use StatsType::*;
		match self {
			Gold => AttrValue::Static("img/gold_certificate.png"),
			Silver => AttrValue::Static("img/silver_certificate.png"),
			Bronze => AttrValue::Static("img/bronze_certificate.png"),
			None => AttrValue::Static("img/journal.png"),
		}
	}

	fn num_matches(self) -> u8 {
		use StatsType::*;
		match self {
			Gold => 3u8,
			Silver => 2u8,
			Bronze => 1u8,
			None => 0u8,
		}
	}
}

#[derive(Properties, PartialEq, Clone, Copy, Debug)]
pub struct StatsListingProps {
	pub stats_type: StatsType,
	pub percent: f32,
}

#[function_component]
pub fn StatsListing(props: &StatsListingProps) -> Html {
	let percent_diff = props.percent - (crate::stats::percentage_shuffled_boards_with_matches(props.stats_type.num_matches().into()) as f32 )* 100.0;
	html!{
		<Row>
			<img src={props.stats_type.image()}/>
			{ format!("{} matches: {:.1}% ({:+.1}%)", props.stats_type.num_matches(), props.percent, percent_diff)}
		</Row>
	}
}