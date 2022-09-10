use std::collections::HashMap;

use gloo_events::EventListener;
use yew::{classes, html, Children, Classes, Component, Html, NodeRef, Properties};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

#[derive(Debug, Properties, PartialEq, Clone)]
struct Props {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    class: Classes,

    #[prop_or_default]
    other_props: Option<GlobalProps>,

    #[prop_or_default]
    node_ref: NodeRef,
}

struct ClayDropDownCaption {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

impl ClayDropDownCaption {
    const DROPDOWN_CAPTION: &'static str = "dropdown-caption";
}

impl Component for ClayDropDownCaption {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props {
            class, children, ..
        } = ctx.props().clone();
        html! {
            <div class={classes!(class, Self::DROPDOWN_CAPTION)}>
                {children}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if let Some(button_props) = &ctx.props().other_props {
            let node_ref = &ctx.props().node_ref;
            let button_props = button_props.clone();
            button_props.inject(node_ref, &mut self.listeners);
        }
    }
}
