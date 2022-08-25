use domatt::attributes::global::{AriaRole, CustomAttribute, Role, Title};
use domatt::attributes::svg::Href;
use domatt::events::Click;
use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::MouseEvent;
use yew::{
    classes, html, Callback, Children, Classes, Component, Context, Html, NodeRef, Properties,
};
use yew_dom_attributes::anchor_props::AnchorProps;
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

use crate::{ButtonDisplayType, ClayButton, ClayLink};

pub struct ClayItem {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
    html: Html,
    anchor_props: Rc<AnchorProps>,
}

/// Props for ClayItem. For details, check the docs:
/// TODO: copy the URL
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayItemProps {
    /// Flag to indicate if the Breadcrumb item is active or not.
    #[prop_or(false)]
    active: bool,

    /// This value is used to be the target of the link.
    #[prop_or_default]
    href: Option<String>,

    /// Label of the Breadcrumb item
    label: String,

    ///Callback for when a Breadcrumb item is clicked.
    #[prop_or_default]
    on_click: Option<Callback<MouseEvent>>,

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

impl ClayItem {
    const ACTIVE: &'static str = "active";
    const TESTID_BASE: &'static str = "testid";
    const BREADCRUMB_TEXT_TRUNCATE: &'static str = "breadcrumb-text-truncate";
    const BREADCRUMB_LINK: &'static str = "breadcrumb-link";

    fn get_active_class(active: bool) -> Option<&'static str> {
        if active {
            Some(Self::ACTIVE)
        } else {
            None
        }
    }
}

impl Component for ClayItem {
    type Message = Msg;
    type Properties = ClayItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        let ClayItemProps {
            active,
            label,
            href,
            on_click,
            ..
        } = ctx.props().clone();

        let mut anchor_props = AnchorProps::new();

        let mut testid = String::with_capacity(Self::TESTID_BASE.len() + label.len());
        testid.push_str(Self::TESTID_BASE);
        testid.push_str(&label);
        let data_test_id = CustomAttribute::new("data-testid", Some(testid));
        anchor_props.add_attribute(Box::new(data_test_id));

        let html = if !active && href.is_some() {
            let href = href.as_ref().expect("there to be an href");
            let href = Href::new(href.to_owned());
            anchor_props.add_attribute(Box::new(href));

            let role = Role::new(AriaRole::Button);
            anchor_props.add_attribute(Box::new(role));

            html! {
                <ClayLink class={Self::BREADCRUMB_LINK} >
                    <span class={Self::BREADCRUMB_TEXT_TRUNCATE}>{label}</span>
                </ClayLink>
            }
        } else if !active && on_click.is_some() {
            let on_click = on_click.expect("onclick callback to be there");
            let on_click = Click::from(on_click);
            anchor_props.add_listener("breadcrumb-click", Rc::new(on_click));

            let title = Title::new(label.clone());
            anchor_props.add_attribute(Box::new(title));

            html! {
                <ClayButton
                    class={Self::BREADCRUMB_LINK}
                    display_type={ButtonDisplayType::Unstyled} >
                    <span class={Self::BREADCRUMB_TEXT_TRUNCATE}>{label}</span>
                </ClayButton>
            }
        } else {
            let title = Title::new(label.clone());
            anchor_props.add_attribute(Box::new(title));

            html! {
                <span class={Self::BREADCRUMB_TEXT_TRUNCATE}>{label}</span>
            }
        };

        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
            html,
            anchor_props: Rc::new(anchor_props),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayItemProps { active, .. } = ctx.props().clone();
        let active_class = Self::get_active_class(active);

        html! {
          <li ref={self.node_ref.clone()} class={classes!("breadcrumb-item", active_class)} >
            {self.html.clone()}
          </li>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let mut anchor_props = self.anchor_props.clone();
        Rc::make_mut(&mut anchor_props).inject(&self.node_ref, &mut self.listeners);
        if let Some(cb) = anchor_props.get_props_update_callback() {
            cb.emit(anchor_props.clone());
        }

        if let Some(html_props) = &ctx.props().html_props {
            let mut html_props = html_props.clone();
            Rc::make_mut(&mut html_props).inject(&self.node_ref, &mut self.listeners);
            if let Some(cb) = html_props.get_props_update_callback() {
                cb.emit(html_props.clone());
            }
        }
    }
}
