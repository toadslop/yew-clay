use crate::{CONTAINER_CLASS, SPRITEMAP};
use yew::{html, html_nested, Component, Context, Html};
use yew_clay::AlertVariant;
use yew_clay::ClayButton;
use yew_clay::ClayButtonGroup;
use yew_clay::ClayContainer;
use yew_clay::{AlertDisplayType, ClayAlert};

pub struct InlineAlert {}

impl Component for InlineAlert {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Inline"}</h2>
                <ClayAlert
                    display_type={AlertDisplayType::Info}
                    spritemap={SPRITEMAP}
                    title={"With a Button"}
                    variant={AlertVariant::Inline}
                    actions={{html_nested! {
                        <ClayButtonGroup>
                            <ClayButton alert={true}>{"View"}</ClayButton>
                        </ClayButtonGroup>
                    }}}>
                    {"This is an alert with a button!"}
                </ClayAlert>
            </ClayContainer>
        }
    }
}
