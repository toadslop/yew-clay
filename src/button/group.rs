use std::collections::HashMap;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Node;
use yew::{
    html::IntoPropValue,
    prelude::*,
    virtual_dom::{VChild, VNode},
};

use super::button::ClayButton;

/// A wrapper around ClayButton.Group. Only ClayButtons may be passed as children.
pub struct Group {
    node: Node,
    #[allow(dead_code)]
    props: GroupProps,
}

/// Props for ClayButton. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct GroupProps {
    #[prop_or(false)]
    pub spaced: bool,
    #[prop_or(false)]
    pub vertical: bool,
    #[prop_or_default]
    pub role: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<ClayButton>,
    /// Arbitrary props that will be passed down to the underlying component.
    #[prop_or_default]
    pub misc_attrs: HashMap<String, String>,
}

impl Component for Group {
    type Message = ();
    type Properties = GroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().to_owned();

        Group {
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
        let children = html! {for props.children.iter()};
        let thingy = match children {
            VNode::VTag(tag) => panic!(),
            VNode::VText(text) => panic!(),
            VNode::VComp(comp) => panic!(),
            VNode::VList(list) => {
                let mut nodes = Vec::new();
                for node in list.iter() {
                    match node {
                        VNode::VTag(_) => panic!(),
                        VNode::VText(_) => panic!(),
                        // VNode::VComp(comp) => comp.,
                        VNode::VList(_) => panic!(),
                        VNode::VPortal(_) => panic!(),
                        VNode::VRef(node) => nodes.push(node.clone()),
                    }
                }
                nodes
            }
            VNode::VPortal(portal) => panic!(),
            VNode::VRef(refer) => vec![refer],
        };
        render_clay_button_group(
            &self.node,
            props.spaced,
            props.vertical,
            props.role,
            thingy,
            JsValue::from_serde(&props.misc_attrs).unwrap(),
        );
        VNode::VRef(self.node.clone())
    }
}

#[wasm_bindgen(module = "/src/build/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_clay_button_group")]
    fn render_clay_button_group(
        node: &Node,
        spaced: bool,
        vertical: bool,
        role: String,
        chidlren: Vec<Node>,
        miscAttrs: JsValue,
    );
}
