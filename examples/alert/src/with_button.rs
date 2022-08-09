use crate::{CONTAINER_CLASS, SPRITEMAP};
use yew::{html, Component, Context, Html};
use yew_clay::alert::ClayAlertFooter;
use yew_clay::alert::{AlertDisplayType, ClayAlert};
use yew_clay::button::ClayButton;
use yew_clay::button::ClayButtonGroup;
use yew_clay::layout::ClayContainer;

pub struct AlertWithButtonDemo {}

impl Component for AlertWithButtonDemo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
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
        }
    }
}
