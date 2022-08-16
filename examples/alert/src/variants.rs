use crate::{CONTAINER_CLASS, SPRITEMAP};
use yew::{html, Component, Context, Html};
use yew_clay::ClayContainer;
use yew_clay::{AlertDisplayType, AlertVariant, ClayAlert};

pub struct AlertVariantDemo {}

impl Component for AlertVariantDemo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {

                <ClayContainer class={CONTAINER_CLASS}>
                    <h2>{"Variants"}</h2>
                    <ClayAlert
                        display_type={AlertDisplayType::Info}
                        spritemap={SPRITEMAP}
                        title={"Info"}>
                        {"This is a default alert"}
                    </ClayAlert>
                    <ClayAlert
                        display_type={AlertDisplayType::Warning}
                        spritemap={SPRITEMAP}
                        title={"Warning"}
                        variant={AlertVariant::Stripe}>
                        {"This is a stripe alert"}
                    </ClayAlert>
                    <div class={"c-mt-3"}>
                        <ClayAlert
                            display_type={AlertDisplayType::Danger}
                            spritemap={SPRITEMAP}
                            title={"Error Indicator"}
                            variant={AlertVariant::Feedback} />
                    </div>
                </ClayContainer>
        }
    }
}
