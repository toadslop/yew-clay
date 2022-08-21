use crate::HasBoolClass;
use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayContentCol. For more info about ClayContentCol, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayContentCol {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

/// Props for ClayContentCol. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#contentcol>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayContentColProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// Makes column expand and fill available space in row.
    #[prop_or_default]
    pub expand: Option<bool>,

    /// Provides the benefit of aligning content via flexbox without losing the behavior
    /// of floated elements at the expense of extra markup. Applies `autofit-col-end`.
    #[prop_or_default]
    pub float_end: Option<bool>,

    /// Applies the `autofit-col-gutters` class
    #[prop_or_default]
    pub gutters: Option<bool>,

    /// Applies the `autofit-col-shrink` class
    #[prop_or_default]
    pub shrink: Option<bool>,

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

impl ClayContentCol {
    const AUTOFIT_COL: &'static str = "autofit-col";
    const AUTOFIT_COL_EXPAND: &'static str = "autofit-col-expand";
    const AUTOFIT_COL_GUTTER: &'static str = "autofit-col-gutters";
    const AUTOFIT_COL_SHRINK: &'static str = "autofit-col-shrink";
    const AUTOFIT_COL_END: &'static str = "autofit-col-end";
}

impl HasBoolClass for ClayContentCol {}

impl Component for ClayContentCol {
    type Message = ();
    type Properties = ClayContentColProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let ClayContentColProps {
            class,
            container_element,
            children,
            expand,
            shrink,
            float_end,
            ..
        } = props;

        let expand_class = self.get_bool_class(expand, Self::AUTOFIT_COL_EXPAND);
        let gutter_class = self.get_bool_class(expand, Self::AUTOFIT_COL_GUTTER);
        let shrink_class = self.get_bool_class(shrink, Self::AUTOFIT_COL_SHRINK);
        let float_class = self.get_bool_class(float_end, Self::AUTOFIT_COL_END);

        html! {
            <@{container_element}
                class={classes!(class, Self::AUTOFIT_COL, expand_class, gutter_class, shrink_class, float_class)}
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
