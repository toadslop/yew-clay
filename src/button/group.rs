use std::{collections::HashMap, rc::Rc};

use gloo_events::EventListener;
use yew::{html, ChildrenWithProps, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::props::DomInjector;

use yew_dom_attributes::props::html_element_props::HtmlElementProps;

use super::button::ClayButton;

/// A wrapper around ClayButton.Group. Only ClayButtons may be passed as children.
pub struct ButtonGroup {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for Button Group. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonGroupProps {
    #[prop_or(false)]
    pub spaced: bool,
    #[prop_or(false)]
    pub vertical: bool,
    #[prop_or("group".into())]
    pub role: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<ClayButton>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub html_element_props: Option<Rc<HtmlElementProps>>,
}

impl Component for ButtonGroup {
    type Message = ();
    type Properties = ButtonGroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_element_props {
            let mut html_props = html_props.clone();
            Rc::make_mut(&mut html_props).inject(&self.node_ref, &mut self.listeners);
            html_props
                .get_props_update_callback()
                .emit(html_props.clone());
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut class = vec![props.class.clone()];

        if props.vertical {
            class.push("btn-group-vertical".into());
        } else {
            class.push("btn-group".into());
        }

        let children = props.children.clone();

        html! {
            <div
                class={class.join(" ")}
                role={props.role.clone()}
            >
            { if props.spaced {
                children.into_iter().enumerate().map(|(key, child)| {
                    html!{<div class={"btn-group-item"} key={key}>{child}</div>}
                }).collect::<Html>()
            } else {
                children.into_iter().collect::<Html>()
            }}
            </div>

        }
    }
}
