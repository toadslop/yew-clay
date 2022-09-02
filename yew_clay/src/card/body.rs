use std::collections::HashMap;

use gloo_events::EventListener;
use yew::{classes, html, Children, Classes, Component, Html, NodeRef, Properties};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

use super::Interactive;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub other_props: Option<GlobalProps>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

pub struct ClayCardBody {
    listeners: HashMap<String, EventListener>,
}

impl ClayCardBody {
    const CARD_BODY: &'static str = "card-body";
}

impl Component for ClayCardBody {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let interactive = Self::get_interactive(ctx);
        let tag = Self::get_tag_name(interactive);
        let Props {
            class, children, ..
        } = ctx.props().clone();

        html! {
          <@{tag} class={classes!(class, Self::CARD_BODY)}>
            {children}
          </@>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if let Some(other_props) = &ctx.props().other_props {
            let other_props = other_props.clone();
            other_props.inject(&ctx.props().node_ref, &mut self.listeners);
        }
    }
}

impl Interactive for ClayCardBody {}
