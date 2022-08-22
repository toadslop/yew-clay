use std::rc::Rc;

use yew::{html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;

pub struct Ellipsis;

/// Props for ClayEllipsisProps. For details, check the docs:
/// <https://clayui.com/docs/components/breadcrumb.html>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayEllipsisProps {
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

impl Component for Ellipsis {
    type Message = Msg;
    type Properties = ClayEllipsisProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {}
    }
}
