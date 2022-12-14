use super::button::ClayButton;
use super::ClayButtonWithIcon;
use gloo_events::EventListener;
use std::collections::HashMap;
use yew::virtual_dom::VChild;
use yew::{classes, Classes, NodeRef};
use yew::{html, html::ChildrenRenderer, Component, Context, Html, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A wrapper around ClayButton.Group. Only ClayButtons may be passed as children.
pub struct ClayButtonGroup {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for Button Group. For details, check the docs:
/// https://clayui.com/docs/components/button/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonGroupProps {
    #[prop_or(false)]
    pub spaced: bool,
    #[prop_or(false)]
    pub vertical: bool,
    #[prop_or("group".into())]
    pub role: String,
    #[prop_or_default]
    pub children: ChildrenRenderer<ButtonType>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub html_element_props: Option<GlobalProps>,
}

#[derive(Clone, derive_more::From, PartialEq)]
pub enum ButtonType {
    Button(VChild<ClayButton>),
    IconButton(VChild<ClayButtonWithIcon>),
}

// Now, we implement `Into<Html>` so that yew knows how to render `Item`.
#[allow(clippy::from_over_into)]
impl Into<Html> for ButtonType {
    fn into(self) -> Html {
        match self {
            Self::Button(child) => child.into(),
            Self::IconButton(child) => child.into(),
        }
    }
}

impl Component for ClayButtonGroup {
    type Message = ();
    type Properties = ButtonGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_element_props {
            let html_props = html_props.clone();
            let node_ref = &ctx.props().node_ref;
            html_props.inject(node_ref, &mut self.listeners);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let user_classes = props.class.clone();

        let btn_group_class = if props.vertical {
            "btn-group-vertical".to_string()
        } else {
            "btn-group".to_string()
        };

        let children = props.children.clone();

        html! {
            <div
                ref={&ctx.props().node_ref}
                class={classes!(user_classes, btn_group_class)}
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
