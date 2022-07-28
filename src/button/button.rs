use std::collections::HashMap;

use strum_macros::Display;
use web_sys::{Element, MouseEvent};
use yew::{
    html, virtual_dom::VNode, Callback, Children, Component, Context, Html, NodeRef, Properties,
};

/// A wrapper around ClayButton. Note that only text is an acceptible child component.
pub struct ClayButton {
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked(MouseEvent),
}

/// Props for ClayButton. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    #[prop_or(false)]
    pub alert: bool,
    #[prop_or(false)]
    pub borderless: bool,
    #[prop_or(false)]
    pub block: bool,
    #[prop_or(DisplayType::Primary)]
    pub display_type: DisplayType,
    #[prop_or(false)]
    pub monospaced: bool,
    #[prop_or(false)]
    pub outline: bool,
    #[prop_or(false)]
    pub small: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or("button".to_string())]
    pub _type: String,
    /// Arbitrary props that will be passed down to the underlying component.
    #[prop_or_default]
    pub misc_attrs: HashMap<String, Option<String>>,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

#[derive(PartialEq)]
struct MiscAttrs(HashMap<String, String>);

impl Properties for MiscAttrs {
    type Builder = AttrBuilder;

    fn builder() -> Self::Builder {
        AttrBuilder {}
    }
}

struct AttrBuilder {}

impl Component for ClayButton {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => {
                ctx.props().onclick.emit(ev);
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let children: Vec<VNode> = props.children.iter().collect();

        if children.len() > 1 {
            panic!("Clay Button only accepts a single text node as a child");
        };

        let mut classes: Vec<String> = vec!["btn".into()];

        if props.alert {
            classes.push("alert-btn".into());
        }

        if props.block {
            classes.push("btn-block".into());
        }

        if props.monospaced {
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

        html! {
            <button
                class={classes.join(" ")}
            ref={self.node_ref.clone()}
            type={props._type}

        >
            {props.children}
        </button>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let misc_attrs = ctx.props().misc_attrs.clone();
            let elem = self.node_ref.cast::<Element>().unwrap();

            for (key, maybe_val) in &misc_attrs {
                let val = maybe_val.clone().unwrap_or("".to_string());
                elem.set_attribute(key, &val).unwrap();
            }
        }
    }
}

// An enum specifying the different default styles of this button.
#[derive(Debug, PartialEq, Clone, Display)]
pub enum DisplayType {
    #[strum(serialize = "primary")]
    Primary,
    #[strum(serialize = "secondary")]
    Secondary,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "success")]
    Success,
    #[strum(serialize = "warning")]
    Warning,
    #[strum(serialize = "danger")]
    Danger,
    #[strum(serialize = "info")]
    Info,
    #[strum(serialize = "unstyled")]
    Unstyled,
}
