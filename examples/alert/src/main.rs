use yew::{html, Component, Context, Html};
use yew_clay::{
    alert::{AlertDisplayType, AlertVariant, ClayAlert, ClayAlertFooter},
    button::{ClayButton, ClayButtonGroup},
    layout::ClayContainer,
};

pub enum Msg {}
struct AlertDemo {}

const SPRITEMAP: &'static str = "static/icons.svg";
const CONTAINER_CLASS: &'static str = "mt-5 py-3 border";

impl Component for AlertDemo {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
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

                <ClayContainer class={CONTAINER_CLASS}>
                    <h2>{"Using with ClayButton"}</h2>
                    <ClayAlert
                        display_type={AlertDisplayType::Info}
                        spritemap={SPRITEMAP}
                        title={"With a Button"}>
                        {"This is an alert with a button!"}
                        <ClayAlertFooter>
                            <ClayButtonGroup>
                                <ClayButton alert={true}>{"View"}</ClayButton>
                            </ClayButtonGroup>
                        </ClayAlertFooter>
                    </ClayAlert>
                </ClayContainer>
            </>
        }
    }
}

fn main() {
    yew::start_app::<AlertDemo>();
}
