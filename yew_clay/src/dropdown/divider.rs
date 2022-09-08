use domatt::attributes::global::AriaRole;
use yew::{html, Component, Html};

struct ClayDropDownDivider;

impl ClayDropDownDivider {
    const DROPDOWN_DIVIDER: &'static str = "dropdown-divider";
}

impl Component for ClayDropDownDivider {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <li class={Self::DROPDOWN_DIVIDER} role={AriaRole::Separator.as_ref()} />
        }
    }
}
