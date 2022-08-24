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
    None,
}

impl Component for WithContext {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let mut context_props = AnchorProps::new();
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

        let mut anchor_props = AnchorProps::new();
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
                    <div class="row pl-2">
                        <ClayLink anchor_props={self.anchor_props.clone()}>{"Click Me"}</ClayLink>
                    </div>
                    <div class="row pl-2">
                        <ClayLink >{"Click Me Too"}</ClayLink>
                    </div>
                </ContextProvider<LinkContext>>
            </ClayContainer>
        }
    }
}
