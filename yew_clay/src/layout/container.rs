use super::Sizing;
use gloo_events::EventListener;
use std::collections::HashMap;
use yew::{classes, html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::global_props::GlobalProps;
use yew_dom_attributes::DomInjector;

/// A Yew implementation of ClayContainer. For more info about ClayContainer, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayContainer {
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayContainer. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#container>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayContainerProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

    /// Adds `.container-fluid` class to create a fluid container that doesn't expand beyond a set width
    #[prop_or(false)]
    pub fluid: bool,

    /// Adds `.container-fluid-${size}` class to set max width on container.
    #[prop_or_default]
    pub fluid_size: Option<FluidSize>,

    /// Adds the `.container-form-${formSize}` class to properly space between application controls and
    /// the form. This class only modifies the padding on the container.
    #[prop_or_default]
    pub form_size: Option<FormSize>,

    /// Adds the `.container-view` class to properly space between application controls and view pages
    /// (e.g., Card View, Table View, or List View). This class only modifies the padding on the container.
    #[prop_or(false)]
    pub view: bool,

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

impl ClayContainer {
    const FLUID: &'static str = "fluid";
    const CONTAINER: &'static str = "container";
    const FORM: &'static str = "form";
    const VIEW: &'static str = "view";
    const MAX: &'static str = "max";

    fn get_container_type_class(&self, fluid: bool) -> String {
        if fluid {
            let mut container_class =
                String::with_capacity(Self::CONTAINER.len() + Self::FLUID.len() + 1);
            container_class.push_str(Self::CONTAINER);
            container_class.push_str("-");
            container_class.push_str(Self::FLUID);
            container_class
        } else {
            Self::CONTAINER.into()
        }
    }

    fn get_container_form_size_class(&self, form_size: Option<FormSize>) -> Option<String> {
        form_size.map(|size| {
            let size = size.as_ref();
            let mut form_class =
                String::with_capacity(Self::CONTAINER.len() + Self::FLUID.len() + size.len() + 2);
            form_class.push_str(Self::CONTAINER);
            form_class.push_str("-");
            form_class.push_str(Self::FORM);
            form_class.push_str("-");
            form_class.push_str(size);
            form_class
        })
    }

    fn get_container_view_class(&self, view: bool) -> Option<String> {
        if view {
            let mut view_class =
                String::with_capacity(Self::CONTAINER.len() + Self::VIEW.len() + 1);
            view_class.push_str(Self::CONTAINER);
            view_class.push_str("-");
            view_class.push_str(Self::VIEW);
            Some(view_class)
        } else {
            None
        }
    }

    fn get_fluid_max_class(&self, fluid: bool, fluid_size: Option<FluidSize>) -> Option<String> {
        if fluid && fluid_size.is_some() {
            let fluid_size = fluid_size.unwrap();
            let fluid_size_ref = fluid_size.as_ref();
            let mut fluid_max_class = String::with_capacity(
                Self::CONTAINER.len()
                    + Self::FLUID.len()
                    + Self::MAX.len()
                    + fluid_size_ref.len()
                    + 3,
            );
            fluid_max_class.push_str(Self::CONTAINER);
            fluid_max_class.push_str("-");
            fluid_max_class.push_str(Self::FLUID);
            fluid_max_class.push_str("-");
            fluid_max_class.push_str(Self::MAX);
            fluid_max_class.push_str("-");
            fluid_max_class.push_str(fluid_size_ref);
            Some(fluid_max_class)
        } else {
            None
        }
    }
}

impl Component for ClayContainer {
    type Message = ();
    type Properties = ClayContainerProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayContainerProps {
            container_element,
            class,
            fluid,
            form_size,
            view,
            fluid_size,
            node_ref,
            children,
            ..
        } = ctx.props().clone();

        let container_type = self.get_container_type_class(fluid);
        let container_form_size = self.get_container_form_size_class(form_size);
        let container_view = self.get_container_view_class(view);
        let fluid_max = self.get_fluid_max_class(fluid, fluid_size);
        let tag_name = container_element;

        html! {
            <@{tag_name}
                class={classes!(class, container_type, container_form_size, container_view, fluid_max)}
                ref={node_ref} >
                {children}
            </@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(html_props) = &ctx.props().html_props {
            let html_props = html_props.clone();
            let node_ref = &ctx.props().node_ref;
            html_props.inject(node_ref, &mut self.listeners);
        }
    }
}

/// Type alias for [Sizing].
type FluidSize = Sizing;

/// Type alias for [Sizing].
type FormSize = Sizing;
