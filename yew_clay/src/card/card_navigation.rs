use crate::{card::ClayCardContext, ClayLink};
use domatt::{
    attributes::{anchor::Href, global::AriaRole},
    events::Click,
};
use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Children, Classes, Component, ContextProvider, NodeRef, Properties};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub horizontal: bool,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    // #[prop_or_default]
    // pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub other_props: Option<GlobalProps>,

    #[prop_or_default]
    pub node_ref: NodeRef,
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

        let role = if let Some(other_props) = &other_props {
            if other_props.has_event_type(Click::KEY) {
                Some(AriaRole::Button.as_ref()) // TODO: implement into prop value for all domatt values (cfg Yew)
            } else {
                None
            }
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
