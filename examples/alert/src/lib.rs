use yew::{html, Component, Context, Html};
pub mod variants;
use crate::variants::AlertVariantDemo;
pub mod with_button;
use crate::with_button::AlertWithButtonDemo;
pub mod inline;
use crate::inline::InlineAlert;
pub mod toast_container;
use crate::toast_container::WithToastContainer;
use wasm_bindgen::prelude::wasm_bindgen;
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

                <WithToastContainer />
            </>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<AlertDemo>();
}
