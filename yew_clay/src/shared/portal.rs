use web_sys::Element;
use yew::{
    html::ChildrenRenderer,
    virtual_dom::{AttrValue, VComp, VList},
    Children, Classes, Component, Html, NodeRef, Properties,
};

struct ClayPortalContext(NodeRef);

fn create_div_element(class: &str, id: &str) -> Element {
    let element = gloo_utils::document()
        .create_element("div")
        .expect("creating an element to be successful");

    element.set_class_name(class);
    element.set_id(id);

    element
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct ClayPortalProps {
    /// Class to add to the root element
    #[prop_or_default]
    class: Classes,

    /// Id fof the root element
    #[prop_or_default]
    id: Option<AttrValue>,
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum PortalChild {
    NodeList(VList),
    SingleNode(VComp),
}

impl Into<Html> for PortalChild {
    fn into(self) -> Html {
        match self {
            Self::NodeList(child) => child.into(),
            Self::SingleNode(child) => child.into(),
        }
    }
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    portal_props: ClayPortalProps,
    container_ref: NodeRef,
    sub_portal_ref: NodeRef,
    children: ChildrenRenderer<PortalChild>,
}

#[derive(Debug)]
pub struct ClayPortal;

impl Component for ClayPortal {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        todo!()
    }
}
