use gloo_events::EventListener;
use std::collections::HashMap;
use strum::Display;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayCol. For more info about ClayCol, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayCol {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayContainer. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#container>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayColProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// The number of columns to span on large devices
    #[prop_or_default]
    pub lg: Option<ColSize>,

    /// The number of columns to span on medium devices
    #[prop_or_default]
    pub md: Option<ColSize>,

    /// The number of columns to span on all  devices
    #[prop_or_default]
    pub size: Option<ColSize>,

    /// The number of columns to span on small devices
    #[prop_or_default]
    pub sm: Option<ColSize>,

    /// The number of columns to span on extra-small devices
    #[prop_or_default]
    pub xs: Option<ColSize>,

    /// The number of columns to span on extra-large devices
    #[prop_or_default]
    pub xl: Option<ColSize>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<GlobalProps>,
}

impl ClayCol {
    fn get_col_class(&self, props: &ClayColProps) -> Option<String> {
        let ClayColProps {
            lg,
            md,
            sm,
            xs,
            xl,
            size,
            ..
        } = props;

        if lg.is_none()
            && md.is_none()
            && sm.is_none()
            && xs.is_none()
            && xl.is_none()
            && size.is_none()
        {
            Some("col".to_string())
        } else {
            None
        }
    }

    fn get_xs(&self, props: &ClayColProps) -> Option<ColSize> {
        let ClayColProps { xs, size, .. } = props;
        if let Some(xs) = xs {
            Some(xs.to_owned())
        } else if let Some(size) = size {
            Some(size.to_owned())
        } else {
            None
        }
    }

    fn get_col_size_class(
        &self,
        size: &Option<ColSize>,
        screen_size: ScreenSize,
    ) -> Option<String> {
        if let Some(size) = size {
            match size {
                ColSize::Boolean(val) => {
                    if *val && screen_size != ScreenSize::Xs {
                        Some(format!("col-{}", screen_size))
                    } else {
                        None
                    }
                }
                ColSize::Number(val) => {
                    if *val > 12 || *val <= 0 {
                        panic!(
                            "The value of ColSize::Number cannot be greater than 12 or less than 1"
                        )
                    } else {
                        Some(format!("col-{}-{}", screen_size, val))
                    }
                }
                ColSize::Auto => Some(format!("col-{}-auto", screen_size)),
            }
        } else {
            None
        }
    }
}

impl Component for ClayCol {
    type Message = ();
    type Properties = ClayColProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayColProps {
            class,
            container_element,
            children,
            lg,
            md,
            sm,
            xl,
            ..
        } = ctx.props().clone();

        let col_class = self.get_col_class(&ctx.props());

        let xs = self.get_xs(&ctx.props());

        let lg_class = self.get_col_size_class(&lg, ScreenSize::Lg);
        let md_class = self.get_col_size_class(&md, ScreenSize::Md);
        let xs_class = self.get_col_size_class(&xs, ScreenSize::Xs);
        let sm_class = self.get_col_size_class(&sm, ScreenSize::Sm);
        let xl_class = self.get_col_size_class(&xl, ScreenSize::Xl);

        html! {
            <@{container_element}
                class={classes!(class, col_class, lg_class, md_class, xs_class, sm_class, xl_class)}
                ref={self.node_ref.clone()} >
                {children.clone()}
            </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let html_props = html_props.clone();
            html_props.inject(&self.node_ref, &mut self.listeners);
        }
    }
}

#[derive(Display, Debug, PartialEq, Clone)]
pub enum ColSize {
    Boolean(bool),
    Number(u8),
    #[strum(serialize = "lowercase")]
    Auto,
}

#[derive(Display, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
enum ScreenSize {
    Lg,
    Md,
    Sm,
    Xs,
    Xl,
}
