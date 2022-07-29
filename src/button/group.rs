use yew::prelude::*;

use crate::MiscAttrs;

use super::button::ClayButton;

/// A wrapper around ClayButton.Group. Only ClayButtons may be passed as children.
pub struct Group {
    node_ref: NodeRef,
}

/// Props for ClayButton. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct GroupProps {
    #[prop_or(false)]
    pub spaced: bool,
    #[prop_or(false)]
    pub vertical: bool,
    #[prop_or("group".into())]
    pub role: String,
    #[prop_or_default]
    pub children: ChildrenWithProps<ClayButton>,
    #[prop_or_default]
    pub class: String,
    /// Arbitrary props that will be passed down to the underlying component.
    #[prop_or_default]
    pub misc_attrs: MiscAttrs,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

impl Component for Group {
    type Message = ();
    type Properties = GroupProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        ctx.props().misc_attrs.render(&self.node_ref);
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut class = vec![props.class.clone()];

        if props.vertical {
            class.push("btn-group-vertical".into());
        } else {
            class.push("btn-group".into());
        }

        let children = props.children.clone();

        html! {
            <div
                class={class.join(" ")}
                role={props.role.clone()}
            >
            { if props.spaced {
                children.into_iter().enumerate().map(|(key, child)| {
                    html!{<div class={"btn-group-item"} key={key}>{child}</div>}
                }).collect::<Html>()
            } else {
                children.into_iter().collect::<Html>()
            }}
            </div>

        }
    }
}
