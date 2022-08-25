use wasm_bindgen::prelude::wasm_bindgen;
use yew::{html, Component, Context, Html};
use yew_clay::{ButtonDisplayType, ClayButton, ClayButtonGroup, ClayContainer};

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
                <h2>{"Button Group"}</h2>
                <ClayButtonGroup>
                    <ClayButton>{"This"}</ClayButton>
                    <ClayButton display_type={ButtonDisplayType::Secondary}>{"is"}</ClayButton>
                    <ClayButton>{"a"}</ClayButton>
                    <ClayButton display_type={ButtonDisplayType::Secondary}>{"button"}</ClayButton>
                    <ClayButton>{"group."}</ClayButton>
                </ClayButtonGroup>
            </ClayContainer>
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    yew::start_app::<LinkDemo>();
}
