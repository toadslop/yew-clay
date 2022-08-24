use std::collections::HashMap;
use std::rc::Rc;

use gloo_events::EventListener;
use yew::{classes, html, Component, Context, Html, NodeRef};

use yew_dom_attributes::DomInjector;

use super::ContainerProps;

/// A Yew implementation of ClaySheetSection.
pub struct ClaySheetSection {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

impl Component for ClaySheetSection {
    type Message = ();
    type Properties = ContainerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let tag_name = props.container_element;
        let class = props.class;

        html! {
            <@{tag_name}
                class={classes!(class, "sheet-section")}
                ref={self.node_ref.clone()} >
                {props.children.clone()}
            </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let mut html_props = html_props.clone();
            Rc::make_mut(&mut html_props).inject(&self.node_ref, &mut self.listeners);
            if let Some(cb) = html_props.get_props_update_callback() {
                cb.emit(html_props.clone());
            }
        }
    }
}
