use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClaySheet.
pub struct ClaySheet {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
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
    pub html_props: Option<Rc<GlobalProps>>,
}

impl Component for ClaySheet {
    type Message = ();
    type Properties = SheetProps;

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
        let size = if props.large { Some("sheet-lg") } else { None };

        html! {
            <@{tag_name}
                class={classes!(class, "sheet", "sheet-section", size)}
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
