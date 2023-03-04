use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct RowColProps {
	pub children: Children,
}

#[function_component]
pub fn Row(props: &RowColProps) -> Html {
	html! {
		<div class="row">
			{ for props.children.iter() }
		</div>
	}
}

#[function_component]
pub fn Col(props: &RowColProps) -> Html {
	html! {
		<div class="col">
			{ for props.children.iter() }
		</div>
	}
}