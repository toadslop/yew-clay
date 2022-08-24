use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Children, Classes, Component, Context, Html, NodeRef, Properties,
};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

pub struct ClayItem {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

/// Props for ClayItem. For details, check the docs:
/// TODO: copy the URL
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayItemProps {
    /// Flag to indicate if the Breadcrumb item is active or not.
    #[prop_or(false)]
    active: bool,

    /// This value is used to be the target of the link.
    #[prop_or_default]
    href: Option<String>,

    /// Label of the Breadcrumb item
    #[prop_or_default]
    label: Option<String>,

    ///Callback for when a Breadcrumb item is clicked.
    #[prop_or_default]
    on_click: Option<Callback<MouseEvent>>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<Rc<GlobalProps>>,
}

pub enum Msg {}

impl ClayItem {
    const ACTIVE: &'static str = "active";

    fn get_active_class(active: bool) -> Option<&'static str> {
        if active {
            Some(Self::ACTIVE)
        } else {
            None
        }
    }
}

impl Component for ClayItem {
    type Message = Msg;
    type Properties = ClayItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayItemProps { active, .. } = ctx.props().clone();
        let active_class = Self::get_active_class(active);
        html! {
          <li class={classes!("breadcrumb-item", active_class)} >
           // 	{!active && href ? (
        // 		<ClayLink
        // 			className="breadcrumb-link"
        // 			data-testid={`testId${label}`}
        // 			href={href}
        // 			role="button"
        // 		>
        // 			<span className="breadcrumb-text-truncate">{label}</span>
        // 		</ClayLink>
        // 	) : !active && onClick ? (
        // 		<ClayButton
        // 			className="breadcrumb-link"
        // 			data-testid={`testId${label}`}
        // 			displayType="unstyled"
        // 			onClick={onClick}
        // 			title={label}
        // 		>
        // 			<span className="breadcrumb-text-truncate">{label}</span>
        // 		</ClayButton>
        // 	) : (
        // 		<span
        // 			className="breadcrumb-text-truncate"
        // 			data-testid={`testId${label}`}
        // 			title={label}
        // 		>
        // 			{label}
        // 		</span>
        // 	)}
          </li>
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
