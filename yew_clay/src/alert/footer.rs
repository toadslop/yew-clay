use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayAlertFooter.
pub struct ClayAlertFooter {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayAlert. For details, check the docs:
/// <https://clayui.com/docs/components/alert/api.html#alert>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayAlertFooterProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<GlobalProps>,
}

impl Component for ClayAlertFooter {
    type Message = ();
    type Properties = ClayAlertFooterProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayAlertFooterProps {
            class,
            children,
            node_ref,
            ..
        } = ctx.props().clone();

        html! {
            <div ref={node_ref} class={classes!(class, "alert-footer")}>
                {children}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let html_props = html_props.clone();
            html_props.inject(&self.node_ref, &mut self.listeners);
        }
    }
}
