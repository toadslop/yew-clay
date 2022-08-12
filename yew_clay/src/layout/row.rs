use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use strum::Display;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::props::html_element_props::HtmlElementProps;
use yew_dom_attributes::props::DomInjector;

use crate::HasBoolClass;

/// A Yew implementation of ClayRow. For more info about ClayRow, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayRow {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

/// Props for ClayRow. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#row>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayRowProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// This removes the negative margins from .row and the
    /// horizontal padding from all immediate children columns
    #[prop_or(true)]
    pub gutters: bool,

    /// Horizontal positioning of row contents
    #[prop_or_default]
    pub justify: Option<RowJustify>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<Rc<HtmlElementProps>>,
}

impl ClayRow {
    const ROW: &'static str = "row";
    const NO_GUTTERS: &'static str = "no-gutters";
    const JUSTIFY_CONTENT: &'static str = "justify-content";

    fn get_justify_class(&self, justify: Option<RowJustify>) -> Option<String> {
        if let Some(justify) = justify {
            Some(format!("{}-{}", Self::JUSTIFY_CONTENT, justify))
        } else {
            None
        }
    }
}

impl HasBoolClass for ClayRow {}

impl Component for ClayRow {
    type Message = ();
    type Properties = ClayRowProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let ClayRowProps {
            class,
            container_element,
            children,
            gutters,
            justify,
            ..
        } = props;

        let gutters_class = self.match_bool(!gutters, Self::NO_GUTTERS);
        let justify_class = self.get_justify_class(justify);

        html! {
            <@{container_element}
                class={classes!(class, Self::ROW, gutters_class, justify_class)}
                ref={self.node_ref.clone()} >
                {children.clone()}
            </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let mut html_props = html_props.clone();
            Rc::make_mut(&mut html_props).inject(&self.node_ref, &mut self.listeners);
            html_props
                .get_props_update_callback()
                .emit(html_props.clone());
        }
    }
}

#[derive(Display, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum RowJustify {
    Start,
    Center,
    End,
    Around,
    Between,
}
