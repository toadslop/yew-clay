#![recursion_limit = "512"]
#![deny(elided_lifetimes_in_paths)]

pub mod inline;
pub mod toast_container;
pub mod variants;
pub mod with_button;

use crate::inline::InlineAlert;
use crate::toast_container::WithToastContainer;
use crate::variants::AlertVariantDemo;
use crate::with_button::AlertWithButtonDemo;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::Component;
use yew::Context;
use yew::Html;

const SPRITEMAP: &'static str = "/icons.svg";
const CONTAINER_CLASS: &'static str = "mt-5 py-3 border";

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

pub struct AlertDemo {}

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

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<AlertDemo>();
    Ok(())
}
