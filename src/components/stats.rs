use yew::prelude::*;
use super::rowcol::{Row, Col};

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
	pub percent: Option<f32>,
}

#[function_component]
pub fn StatsListing(props: &StatsListingProps) -> Html {
	
	let shuffled_probability : f32 = crate::stats::percentage_shuffled_boards_with_matches(props.stats_type.num_matches().into()) as f32;

	html!{
		<Row>
			<img src={props.stats_type.image()}/>
			{
				match props.percent {
					Some(p) => {
						let percent_diff = p - shuffled_probability * 100.0;
						format!("{} matches: {:.1}% ({:+.1}%)", props.stats_type.num_matches(), p, percent_diff)
					},
					None => format!("{} matches: {:.1}% (--.-%)", props.stats_type.num_matches(), shuffled_probability * 100.0),
				}
			}
		</Row>
	}
}


#[derive(Properties, PartialEq, Clone, Copy, Debug)]
pub struct StatsListingTableProps {
	pub stats: crate::stats::BoardMatchCounter,
}

#[function_component]
pub fn StatsListingTable(props: &StatsListingTableProps) -> Html {
	let distribution = props.stats.distribution();
	html!{
		<Col>
			<StatsListing stats_type={StatsType::None}   percent={distribution.map(|d| 100.0 * d[0])} />
			<StatsListing stats_type={StatsType::Bronze} percent={distribution.map(|d| 100.0 * d[1])} />
			<StatsListing stats_type={StatsType::Silver} percent={distribution.map(|d| 100.0 * d[2])} />
			<StatsListing stats_type={StatsType::Gold}   percent={distribution.map(|d| 100.0 * d[3])} />
		</Col>
	}
}