use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayContentSection. For more info about ClayContentSection, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayContentSection {
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayContentCol. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#contentcol>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayContentSectionProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

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

impl ClayContentSection {
    const AUTOFIT_SECTION: &'static str = "autofit-section";
}

impl Component for ClayContentSection {
    type Message = ();
    type Properties = ClayContentSectionProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let ClayContentSectionProps {
            class,
            container_element,
            children,
            node_ref,
            ..
        } = props;

        html! {
            <@{container_element}
                class={classes!(class, Self::AUTOFIT_SECTION)}
                ref={node_ref} >
                {children.clone()}
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
