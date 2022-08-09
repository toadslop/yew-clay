use yew::{html, Component, Context, Html};
use yew_clay::badge::{BadgeDisplayType, ClayBadge};
use yew_clay::layout::ClayContainer;

struct BadgeDemo {}

const CONTAINER_CLASS: &'static str = "mt-5 py-3 border";

impl Component for BadgeDemo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Badge"}</h2>
                <ClayBadge display_type={BadgeDisplayType::Success} label={"100"} />
                <ClayBadge display_type={BadgeDisplayType::Primary} label={"100"} />
                <ClayBadge display_type={BadgeDisplayType::Secondary} label={"100"} />
                <ClayBadge display_type={BadgeDisplayType::Info} label={"100"} />
                <ClayBadge display_type={BadgeDisplayType::Warning} label={"100"} />
                <ClayBadge display_type={BadgeDisplayType::Danger} label={"100"} />
            </ClayContainer>
        }
    }
}

fn main() {
    yew::start_app::<BadgeDemo>();
}
