use domatt::attributes::anchor::Href;
use domatt::attributes::aria::AriaLabel;
use std::rc::Rc;
use yew::{html, Component};
use yew_clay::{ClayContainer, ClayLink, LinkDisplayType};
use yew_dom_attributes::anchor_props::AnchorProps;
use yew_dom_attributes::DomInjector;

use crate::CONTAINER_CLASS;

pub struct BasicUsage {
    anchor_props: Rc<AnchorProps>,
    with_aria_props: Rc<AnchorProps>,
}

impl Component for BasicUsage {
    type Message = ();
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut anchor_props = AnchorProps::new();
        anchor_props.add_attribute(Box::new(Href::new(String::from("#link-styles"))));

        let mut with_aria_props = AnchorProps::new();
        with_aria_props.add_attribute(Box::new(AriaLabel::new(String::from("My Link"))));
        with_aria_props.add_attribute(Box::new(Href::new(String::from("#link-styles"))));

        Self {
            anchor_props: Rc::new(anchor_props),
            with_aria_props: Rc::new(with_aria_props),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <h2>{"Basic Usage"}</h2>
                <ClayLink anchor_props={self.anchor_props.clone()}>{"Default"}</ClayLink>
                <ClayLink
                    anchor_props={self.anchor_props.clone()}
                    display_type={LinkDisplayType::Secondary}>
                        {"Secondary"}
                </ClayLink>
                <ClayLink anchor_props={self.with_aria_props.clone()}>{"With Aria Label"}</ClayLink>
            </ClayContainer>
        }
    }
}
