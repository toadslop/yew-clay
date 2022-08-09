use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use strum::Display;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::props::html_element_props::HtmlElementProps;
use yew_dom_attributes::props::DomInjector;

use crate::HasBoolClass;

/// A Yew implementation of ClayContent. For more info about ClayContent, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayContentRow {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayContainer. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#container>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayContentRowProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// Provides the benefit of aligning content via flexbox without losing the behavior
    /// of floated elements at the expense of extra markup.
    #[prop_or_default]
    pub float: Option<ContentFloat>,

    /// Give negative margins on the top, right, bottom, and left to offset the padding
    #[prop_or_default]
    pub no_gutters: Option<ContentNoGutters>,

    /// Gives padding to all autofit-cols that are direct children of autofit-row.
    #[prop_or_default]
    pub padded: Option<bool>,

    /// Adds class for aligning items within the row.
    #[prop_or_default]
    pub vertical_align: Option<ContentVerticalAlign>,

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

impl ClayContentRow {
    const AUTOFIT_FLOAT: &'static str = "autofit-float";
    const AUTOFIT_PADDED: &'static str = "autofit-padded";
    const AUTOFIT_PADDED_NO_GUTTERS: &'static str = "autofit-padded-no-gutters";
    const AUTOFIT_ROW: &'static str = "autofit-row";

    fn get_float_class(&self, float: &Option<ContentFloat>) -> Option<String> {
        if let Some(float) = float {
            match float {
                ContentFloat::Boolean(val) => match val {
                    true => Some(Self::AUTOFIT_FLOAT.into()),
                    false => None,
                },
                _ => Some(format!("{}-{}", Self::AUTOFIT_FLOAT, float.to_string())),
            }
        } else {
            None
        }
    }

    fn get_no_gutters_class(&self, no_gutters: &Option<ContentNoGutters>) -> Option<String> {
        if let Some(no_gutters) = no_gutters {
            match no_gutters {
                ContentNoGutters::Boolean(val) => match val {
                    true => Some(Self::AUTOFIT_PADDED_NO_GUTTERS.into()),
                    false => None,
                },
                _ => Some(format!(
                    "{}-{}",
                    Self::AUTOFIT_PADDED_NO_GUTTERS,
                    no_gutters.to_string()
                )),
            }
        } else {
            None
        }
    }

    fn get_vertical_align_class(
        &self,
        vertical_align: &Option<ContentVerticalAlign>,
    ) -> Option<String> {
        if let Some(vertical_align) = vertical_align {
            Some(format!(
                "{}-{}",
                Self::AUTOFIT_ROW,
                vertical_align.to_string()
            ))
        } else {
            None
        }
    }
}

impl HasBoolClass for ClayContentRow {}

impl Component for ClayContentRow {
    type Message = ();
    type Properties = ClayContentRowProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let ClayContentRowProps {
            class,
            container_element,
            children,
            float,
            padded,
            no_gutters,
            vertical_align,
            ..
        } = props;

        let float_class = self.get_float_class(&float);
        let padded_class = self.get_bool_class(padded, Self::AUTOFIT_PADDED);
        let no_gutters_class = self.get_no_gutters_class(&no_gutters);
        let vertical_align_class = self.get_vertical_align_class(&vertical_align);

        html! {
            <@{container_element}
                class={classes!(class, "autofit-row", float_class, padded_class, no_gutters_class, vertical_align_class)}
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
#[strum(serialize_all = "kebab-case")]
pub enum ContentFloat {
    Boolean(bool),
    SmDown,
    MdDown,
    End,
    EndSmDown,
    EndMdDown,
}

#[derive(Display, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ContentNoGutters {
    Sm,
    X,
    Y,
    Boolean(bool),
}

#[derive(Display, Debug, PartialEq, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum ContentVerticalAlign {
    Center,
    End,
}
