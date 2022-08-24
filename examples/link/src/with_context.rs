use domatt::attributes::svg::Href;
use domatt::events::Click;
use gloo_dialogs::confirm;
use std::rc::Rc;
use yew::{html, Component, ContextProvider, MouseEvent};
use yew_clay::{ClayContainer, ClayLink, LinkContext};
use yew_dom_attributes::anchor_props::AnchorProps;
use yew_dom_attributes::DomInjector;

use crate::CONTAINER_CLASS;

pub struct WithContext {
    link_context: LinkContext,
    anchor_props: Rc<AnchorProps>,
}

pub enum Msg {
    UpdateContextProps(Rc<AnchorProps>),
    UpdateAnchorProps(Rc<AnchorProps>),
    None,
}

impl Component for WithContext {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let update_func = |context_props: Rc<AnchorProps>| Msg::UpdateContextProps(context_props);
        let mut context_props = AnchorProps::new(&ctx, update_func);
        let click = ctx.link().callback(|ev: MouseEvent| {
            match confirm("r u sure?") {
                true => (),
                false => ev.prevent_default(),
            };
            Msg::None
        });
        context_props.add_listener("link-alert", Rc::new(Click::from(click)));

        let link_context = LinkContext {
            tag: String::from("a"),
            props: Rc::new(context_props),
        };

        let update_func = |anchor_props: Rc<AnchorProps>| Msg::UpdateAnchorProps(anchor_props);
        let mut anchor_props = AnchorProps::new(&ctx, update_func);
        anchor_props.add_attribute(Box::new(Href::new(String::from("#"))));

        Self {
            link_context,
            anchor_props: Rc::new(anchor_props),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <ClayContainer class={CONTAINER_CLASS}>
                <ContextProvider<LinkContext> context={self.link_context.clone()}>
                    <ClayLink anchor_props={self.anchor_props.clone()}>{"Click Me"}</ClayLink>
                </ContextProvider<LinkContext>>
            </ClayContainer>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateContextProps(context_props) => {
                self.link_context.props = context_props;
                false
            }
            Msg::None => false,
            Msg::UpdateAnchorProps(anchor_props) => {
                self.anchor_props = anchor_props;
                false
            }
        }
    }
}
