use strum_macros::Display;
use web_sys::MouseEvent;
use yew::virtual_dom::Attributes;
use yew::{
    html, virtual_dom::VNode, Callback, Children, Component, Context, Html, NodeRef, Properties,
};
use yew_dom_attributes::aria_attributes::AriaAttributes;
use yew_dom_attributes::attribute_collection::AttributeCollection;
use yew_dom_attributes::dom_attributes::DOMAttributes;
use yew_dom_attributes::misc_attributes::MiscAttrs;

/// A Yew implementation of ClayButton.
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
    pub disabled: bool,
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
    #[prop_or("button".to_string())]
    pub _type: String,
    /// Arbitrary props that will be passed down to the underlying component.
    /// If the value is None, it represents a boolean attribute.
    #[prop_or_default]
    pub misc_attrs: MiscAttrs,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub aria: Option<AriaAttributes>,
    #[prop_or_default]
    pub dom_attrs: Option<DOMAttributes>,
}

impl ClayButton {
    fn get_classes(&self, props: &ButtonProps) -> String {
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

        classes.join(" ")
    }
}

impl Component for ClayButton {
    type Message = Msg;
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(ev) => {
                // ctx.props().onclick.emit(ev);
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

        let classes = self.get_classes(&props);

        // let onclick = ctx.link().callback(|ev| Msg::Clicked(ev));

        html! {

            <button
                class={classes}
                disabled={ctx.props().disabled}
                ref={self.node_ref.clone()}
                type={props._type}
                // onclick={onclick} >
                >
                {props.children}
            </button>

        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        ctx.props().misc_attrs.inject(&self.node_ref);
        if let Some(aria) = ctx.props().aria.clone() {
            aria.inject(&self.node_ref);
        }

        if first_render {
            if let Some(dom_attrs) = ctx.props().dom_attrs.clone() {
                dom_attrs.inject(&self.node_ref);
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
