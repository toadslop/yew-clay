use gloo_events::EventListener;
use std::collections::HashMap;
use strum::AsRefStr;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::button_props::ButtonProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayButton. For more info about ClayButton, check the documentation:
/// [https://clayui.com/docs/components/button.html]
pub struct ClayButton {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayButton. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayButtonProps {
    /// Flag to indicate if button is used within an alert component.
    #[prop_or(false)]
    pub alert: bool,

    /// Flag to indicate if link should be borderless.
    #[prop_or(false)]
    pub borderless: bool,

    /// Renders the button as a block element.
    #[prop_or(false)]
    pub block: bool,

    /// Determines how to button is displayed. Follows bootstrap coloring scheme.
    #[prop_or(ButtonDisplayType::Primary)]
    pub display_type: ButtonDisplayType,

    /// Flag to indicate if button should be monospaced.
    #[prop_or_default]
    pub monospaced: Option<bool>,

    /// Flag to indicate if link needs to have an outline.
    #[prop_or(false)]
    pub outline: bool,

    /// Indicates button should be a small variant.
    #[prop_or(false)]
    pub small: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or("button".to_string())]
    pub _type: String,

    #[prop_or_default]
    pub node_ref: NodeRef,

    #[prop_or_default]
    pub class: Classes,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub button_props: Option<ButtonProps>,
}

impl ClayButton {
    fn get_classes(&self, props: &ClayButtonProps) -> String {
        let mut classes: Vec<String> = vec!["btn".into()];

        if props.alert {
            classes.push("alert-btn".into());
        }

        if props.block {
            classes.push("btn-block".into());
        }

        if props.monospaced.unwrap_or(false) {
            classes.push("btn-monospaced".into());
        }

        if props.borderless {
            classes.push("btn-outline-borderless".into());
        }

        if props.small {
            classes.push("btn-sm".into());
        }

        if !props.outline && !props.borderless && props.display_type != ButtonDisplayType::Base {
            classes.push(format!("btn-{}", props.display_type.as_ref()));
        }

        if props.outline || props.borderless && props.display_type != ButtonDisplayType::Base {
            classes.push(format!("btn-outline-{}", props.display_type.as_ref()));
        }

        classes.join(" ")
    }
}

impl Component for ClayButton {
    type Message = ();
    type Properties = ClayButtonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        let btn_classes = self.get_classes(&props);
        let user_classes = props.class.clone();

        html! {
            <button
                class={classes!(btn_classes, user_classes)}
                ref={&props.node_ref}
                type={props._type.clone()} >
                {props.children.clone()}
            </button>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(button_props) = &ctx.props().button_props {
            let node_ref = &ctx.props().node_ref;
            let button_props = button_props.clone();
            button_props.inject(node_ref, &mut self.listeners);
        }
    }
}

// An enum specifying the different default styles of ClayButton.
#[derive(Debug, PartialEq, Clone, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum ButtonDisplayType {
    #[default]
    Primary,
    Secondary,
    Link,
    Success,
    Warning,
    Danger,
    Info,
    Unstyled,
    Base,
}
