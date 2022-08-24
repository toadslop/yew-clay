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

pub enum Msg {
    UpdateAnchorProps(Rc<AnchorProps>),
    UpdateWithAriaProps(Rc<AnchorProps>),
}

impl Component for BasicUsage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let update_func = |anchor_props: Rc<AnchorProps>| Msg::UpdateAnchorProps(anchor_props);
        let mut anchor_props = AnchorProps::new(&ctx, update_func);
        anchor_props.add_attribute(Box::new(Href::new(String::from("#link-styles"))));

        let update_func = |anchor_props: Rc<AnchorProps>| Msg::UpdateWithAriaProps(anchor_props);
        let mut with_aria_props = AnchorProps::new(&ctx, update_func);
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

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateAnchorProps(new_props) => {
                self.anchor_props = new_props;
                false
            }
            Msg::UpdateWithAriaProps(new_props) => {
                self.with_aria_props = new_props;
                false
            }
        }
    }
}
