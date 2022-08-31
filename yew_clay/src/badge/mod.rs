use gloo_events::EventListener;
use std::collections::HashMap;
use strum::AsRefStr;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayBadge. For more info, check the documentation:
/// [https://clayui.com/docs/components/badge.html]
pub struct ClayBadge {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
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
    pub button_html_attributes: Option<GlobalProps>,
}

impl ClayBadge {
    const BADGE: &'static str = "badge-";

    fn get_display_class(display_type: BadgeDisplayType) -> String {
        let as_str = display_type.as_ref();
        let mut display_class = String::with_capacity(Self::BADGE.len() + as_str.len());
        display_class.push_str(Self::BADGE);
        display_class.push_str(as_str);
        display_class
    }
}

impl Component for ClayBadge {
    type Message = ();
    type Properties = ClayBadgeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayBadgeProps {
            class,
            display_type,
            label,
            node_ref,
            ..
        } = ctx.props().clone();
        let display_class = Self::get_display_class(display_type);

        html! {
            <span
                class={classes!(class, "badge", display_class)}
                ref={node_ref} >
                {label}
            </span>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(button_props) = &ctx.props().button_html_attributes {
            let button_props = button_props.clone();
            button_props.inject(&ctx.props().node_ref, &mut self.listeners);
        }
    }
}

#[derive(AsRefStr, Debug, PartialEq, Clone, Default)]
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
