use std::collections::HashMap;
use strum_macros::Display;
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    JsValue,
};
use web_sys::Node;
use yew::{prelude::*, virtual_dom::VNode};

/// A wrapper around ClayButton. Note that only text is an acceptible child component.
pub struct ClayButton {
    node: Node,
    #[allow(dead_code)]
    props: ButtonProps,
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
    pub onclick: Callback<()>,
    /// Arbitrary props that will be passed down to the underlying component.
    #[prop_or_default]
    pub misc_attrs: HashMap<String, String>,
}

impl Component for ClayButton {
    type Message = ();
    type Properties = ButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().to_owned();

        ClayButton {
            node: Node::from(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap(),
            ),
            props,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();

        let children: Vec<VNode> = props.children.iter().collect();

        if children.len() > 1 {
            panic!("Clay Button only accepts a single text node as a child");
        };

        let text = match children.first() {
            Some(VNode::VText(text_node)) => text_node.text.to_string(),
            _ => panic!("A non-text node was passed as a child to ClayButton"),
        };

        let cb: Closure<dyn Fn() -> ()> = Closure::new(move || props.onclick.emit(()));

        render_clay_ui_button(
            &self.node,
            props.alert,
            props.borderless,
            props.block,
            props.display_type.to_string(),
            props.monospaced,
            props.outline,
            props.small,
            text,
            Closure::into_js_value(cb),
            JsValue::from_serde(&props.misc_attrs).unwrap(),
        );
        VNode::VRef(self.node.clone())
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

#[wasm_bindgen(module = "/src/build/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_clay_button")]
    fn render_clay_ui_button(
        node: &Node,
        alert: bool,
        borderless: bool,
        block: bool,
        displayType: String,
        monospaced: bool,
        outline: bool,
        small: bool,
        text: String,
        onClick: JsValue,
        miscAttrs: JsValue,
    );
}
