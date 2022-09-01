use std::collections::HashMap;

use crate::{card::ClayCardContext, ClayLink};
use domatt::attributes::anchor::Href;
use gloo_events::EventListener;
use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Children, Classes, Component, ContextProvider, NodeRef, Properties,
};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub other_props: Option<GlobalProps>,

    #[prop_or_default]
    node_ref: NodeRef,
}

pub struct ClayCardNavigation {
    context: ClayCardContext,
    listeners: HashMap<String, EventListener>,
}

impl ClayCardNavigation {
    const CARD: &'static str = "card";
    const CARD_INTERACTIVE: &'static str = "card-interactive";
    const CARD_INTERACTIVE_PRIMARY: &'static str = "card-interactive-primary";
    const CARD_TYPE_TEMPLATE: &'static str = "card-type-template";
    const TEMPLATE_CARD: &'static str = "template-card";
    const TEMPLATE_CARD_HORIZONTAL: &'static str = "template-card-horizontal";

    fn get_horizontal_class(horizontal: bool) -> &'static str {
        if horizontal {
            Self::TEMPLATE_CARD_HORIZONTAL
        } else {
            Self::TEMPLATE_CARD
        }
    }
}

impl Component for ClayCardNavigation {
    type Message = ();

    type Properties = Props;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let context = ClayCardContext {
            horizontal: ctx.props().horizontal,
            interactive: true,
        };
        Self {
            context,
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        let Props {
            children,
            other_props,
            node_ref,
            class,
            horizontal,
            onclick,
            ..
        } = ctx.props().clone();

        let classes = classes!(
            class,
            Self::CARD,
            Self::CARD_INTERACTIVE,
            Self::CARD_INTERACTIVE_PRIMARY,
            Self::CARD_TYPE_TEMPLATE,
            Self::get_horizontal_class(horizontal)
        );

        // TODO: redo events section so instead of user providing a key, each type of event has a key like 'onclick' or 'onhover'
        // TODO: remove current onclick from props and instead check for onclick in other props
        // TODO: instead of manually setting role to button, add role to other props if onclick exists

        let role = if onclick.is_some() {
            Some("button")
        } else {
            None
        };

        // TODO: move this to a separate function
        let content = other_props
            .and_then(|other_props| {
                if other_props.has_attribute(Href::KEY) {
                    Some(html! {
                        <ClayLink class={classes.clone()} node_ref={&node_ref}>
                            {children.clone()}
                        </ClayLink>
                    })
                } else {
                    None
                }
            })
            .or_else(|| {
                Some(html! {
                    <div class={classes.clone()} ref={&node_ref} role={role}>{children}</div>
                })
            })
            .unwrap();

        html! {
            <ContextProvider<ClayCardContext> context={self.context.to_owned()}>
                {content}
            </ContextProvider<ClayCardContext>>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if let Some(other_props) = &ctx.props().other_props {
            let other_props = other_props.clone();
            other_props.inject(&ctx.props().node_ref, &mut self.listeners);
        }
    }
}
