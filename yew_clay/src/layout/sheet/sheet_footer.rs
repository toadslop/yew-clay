use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Component, Context, Html};

use yew_dom_attributes::DomInjector;

use super::ContainerProps;

/// A Yew implementation of ClaySheetFooter.
pub struct ClaySheetFooter {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

impl Component for ClaySheetFooter {
    type Message = ();
    type Properties = ContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ContainerProps {
            container_element,
            class,
            node_ref,
            children,
            ..
        } = ctx.props().clone();
        let tag_name = container_element;

        html! {
            <@{tag_name}
                class={classes!(class, "sheet-footer")}
                ref={node_ref} >
                {children}
            </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let html_props = html_props.clone();
            let node_ref = &ctx.props().node_ref;
            html_props.inject(node_ref, &mut self.listeners);
        }
    }
}
