use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use strum::Display;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayBadge. For more info, check the documentation:
/// [https://clayui.com/docs/components/badge.html]
pub struct ClayBadge {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

/// Props for ClayButton. For details, check the docs:
/// https://clayui.com/docs/components/badge/api.html
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayBadgeProps {
    /// Determines the color of the badge.
    #[prop_or_default]
    pub display_type: BadgeDisplayType,

    /// Info that is shown inside of the badge itself.
    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    #[prop_or_default]
    pub class: Classes,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub button_html_attributes: Option<Rc<GlobalProps>>,
}

impl ClayBadge {
    fn get_display_class(display_type: BadgeDisplayType) -> String {
        format!("badge-{}", display_type.to_string())
    }
}

impl Component for ClayBadge {
    type Message = ();
    type Properties = ClayBadgeProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayBadgeProps {
            class,
            display_type,
            label,
            ..
        } = ctx.props().clone();
        let display_class = Self::get_display_class(display_type);

        html! {
            <span
                class={classes!(class, "badge", display_class)}
                ref={self.node_ref.clone()} >
                {label}
            </span>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(button_props) = &ctx.props().button_html_attributes {
            let mut button_props = button_props.clone();
            Rc::make_mut(&mut button_props).inject(&self.node_ref, &mut self.listeners);
            if let Some(cb) = button_props.get_props_update_callback() {
                cb.emit(button_props.clone());
            }
        }
    }
}

#[derive(Display, Debug, PartialEq, Clone, Default)]
#[strum(serialize_all = "lowercase")]
pub enum BadgeDisplayType {
    #[default]
    Primary,
    Secondary,
    Info,
    Danger,
    Success,
    Warning,
}
