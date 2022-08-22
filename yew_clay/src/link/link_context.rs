use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::{anchor_props::AnchorProps, DomInjector};

pub trait LinkContext {
    type BaseComponent: Component<Properties = ATagProps> + Clone + PartialEq;

    fn render(&self, base_html: Html, class: Classes, node_ref: NodeRef) -> Html {
        html! {
            <Self::BaseComponent class={class} ref={node_ref}>{base_html}</Self::BaseComponent>
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultLinkContext;

impl LinkContext for DefaultLinkContext {
    type BaseComponent = ATag;
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ATagProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    #[prop_or_default]
    anchor_props: Option<Rc<AnchorProps>>,
}

#[derive(Debug, Clone)]
pub struct ATag {
    node_ref: NodeRef,
    // This vec holds all the EventListeners defined for this component. They will be automatically
    // removed when the component is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

impl PartialEq for ATag {
    fn eq(&self, other: &Self) -> bool {
        if self.node_ref != other.node_ref {
            return false;
        }

        for key in self.listeners.keys() {
            let own_listener = self.listeners.get(key);
            let other_listener = other.listeners.get(key);
            if !own_listener.is_some() && other_listener.is_some() {
                return false;
            } else if !Rc::ptr_eq(own_listener.unwrap(), other_listener.unwrap()) {
                return false;
            }
        }

        true
    }
}

impl Component for ATag {
    type Message = ();
    type Properties = ATagProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ATagProps {
            children, class, ..
        } = ctx.props().clone();
        html! {
            <a ref={self.node_ref.clone()} class={class}>{children}</a>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(anchor_props) = &ctx.props().anchor_props {
            let mut anchor_props = anchor_props.clone();
            Rc::make_mut(&mut anchor_props).inject(&self.node_ref, &mut self.listeners);
            anchor_props
                .get_props_update_callback()
                .emit(anchor_props.clone());
        }
    }
}
