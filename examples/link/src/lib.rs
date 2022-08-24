use basic_usage::BasicUsage;
use wasm_bindgen::prelude::wasm_bindgen;
use with_context::WithContext;
use yew::{html, Component, Context, Html};

mod basic_usage;
mod with_context;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
struct LinkDemo {}

pub const CONTAINER_CLASS: &'static str = "mt-5 py-3 border";

impl Component for LinkDemo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BasicUsage />
                <WithContext />
            </>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<LinkDemo>();
}
