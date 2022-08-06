use std::{collections::HashMap, rc::Rc};

use gloo_events::EventListener;
use yew::virtual_dom::VChild;
use yew::NodeRef;
use yew::{html, html::ChildrenRenderer, Component, Context, Html, Properties};
use yew_dom_attributes::props::DomInjector;

use yew_dom_attributes::props::html_element_props::HtmlElementProps;

use super::button::ClayButton;
use super::ClayButtonWithIcon;

/// A wrapper around ClayButton.Group. Only ClayButtons may be passed as children.
pub struct ClayButtonGroup {
    node_ref: NodeRef,
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
    pub class: String,
    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub html_element_props: Option<Rc<HtmlElementProps>>,
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

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_element_props {
            let mut html_props = html_props.clone();
            Rc::make_mut(&mut html_props).inject(&self.node_ref, &mut self.listeners);
            html_props
                .get_props_update_callback()
                .emit(html_props.clone());
        }
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
