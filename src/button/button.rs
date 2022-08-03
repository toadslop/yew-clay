use gloo_events::EventListener;
use strum_macros::Display;
use yew::{html, virtual_dom::VNode, Children, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::attribute_injector::AttributeInjector;
use yew_dom_attributes::listener_injector::ListenerInjector;
use yew_dom_attributes::props::button_props::ButtonProps;

/// A Yew implementation of ClayButton. For more info about ClayButton, check the documentation:
/// [https://clayui.com/docs/components/button.html]
pub struct ClayButton {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: Vec<EventListener>,
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
    #[prop_or(DisplayType::Primary)]
    pub display_type: DisplayType,

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

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub button_props: Option<ButtonProps>,

    /// This prop allows you to optimize your use of this component.
    /// It defaults to 1, meaning if you don't need any events, it
    /// won't allocate space for them. If you expect to attach 2
    /// listeners, set this prop to 2 and you'll get exactly space for 2
    /// allocated.
    #[prop_or(1)]
    pub event_count: usize,
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

        if !props.outline && !props.borderless {
            classes.push(format!("btn-{}", props.display_type.to_string()));
        }

        if props.outline || props.borderless {
            classes.push(format!("btn-outline-{}", props.display_type.to_string()));
        }

        classes.join(" ")
    }
}

impl Component for ClayButton {
    type Message = ();
    type Properties = ClayButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        Self {
            node_ref: props.node_ref.clone(),
            listeners: Vec::with_capacity(props.event_count),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let children: Vec<VNode> = props.children.iter().collect();

        if children.len() > 1 {
            panic!("Clay Button only accepts a single text node as a child");
        };

        let classes = self.get_classes(&props);

        html! {
            <button
                class={classes}
                ref={self.node_ref.clone()}
                type={props._type} >
                {props.children}
            </button>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let Some(mut misc_props) = ctx.props().button_props.clone() {
            misc_props.inject(&self.node_ref).unwrap();
            if first_render {
                match misc_props.inject_listeners(&self.node_ref) {
                    Ok(mut listeners) => {
                        self.listeners.append(&mut listeners);
                    }
                    Err(_) => todo!(),
                }
            }
        }
    }
}

// An enum specifying the different default styles of ClayButton.
#[derive(Debug, PartialEq, Clone, Display, Default)]
#[strum(serialize_all = "lowercase")]
pub enum DisplayType {
    #[default]
    Primary,
    Secondary,
    Link,
    Success,
    Warning,
    Danger,
    Info,
    Unstyled,
}
