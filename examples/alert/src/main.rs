use yew::{html, html_nested, Component, Context, Html};
use yew_clay::{
    alert::{AlertDisplayType, AlertVariant, ClayAlert},
    button::{ClayButton, ClayButtonGroup},
    layout::ClayContainer,
};
pub mod variants;
use crate::variants::AlertVariantDemo;
pub mod with_button;
use crate::with_button::AlertWithButtonDemo;
pub mod inline;
use crate::inline::InlineAlert;

struct AlertDemo {}

const SPRITEMAP: &'static str = "static/icons.svg";
const CONTAINER_CLASS: &'static str = "mt-5 py-3 border";

impl Component for AlertDemo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <AlertVariantDemo />

                <AlertWithButtonDemo />

                <InlineAlert />
            </>
        }
    }
}

fn main() {
    yew::start_app::<AlertDemo>();
}
