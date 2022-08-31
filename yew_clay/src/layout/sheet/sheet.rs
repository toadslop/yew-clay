use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClaySheet.
pub struct ClaySheet {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// A generic set of props for container elements.
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct SheetProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// Intercepts the class tag from the parent so it can be modified with classes
    /// necessary for this container.
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Setting this to sets a max-width on the sheet
    #[prop_or(false)]
    pub large: bool,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<GlobalProps>,
}

impl Component for ClaySheet {
    type Message = ();
    type Properties = SheetProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let SheetProps {
            container_element,
            class,
            node_ref,
            children,
            large,
            ..
        } = ctx.props().clone();
        let size = if large { Some("sheet-lg") } else { None };
        let tag_name = container_element;

        html! {
            <@{tag_name}
                class={classes!(class, "sheet", "sheet-section", size)}
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
