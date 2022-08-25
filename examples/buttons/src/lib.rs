use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, Component, Context, Html};
use yew_clay::{ButtonDisplayType, ClayButton, ClayContainer};

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
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Button"}</h2>
                <ClayButton display_type={ButtonDisplayType::Primary}>
                    {"Button Primary"}
                </ClayButton>
                <ClayButton display_type={ButtonDisplayType::Secondary}>
                    {"Button Secondary"}
                </ClayButton>
                <ClayButton display_type={ButtonDisplayType::Base}>
                    {"Base Button"}
                </ClayButton>
                <ClayButton display_type={ButtonDisplayType::Link}>
                    {"Button Link"}
                </ClayButton>
                <ClayButton display_type={ButtonDisplayType::Unstyled}>
                    {"Button Unstyled"}
                </ClayButton>
            </ClayContainer>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<LinkDemo>();
}
