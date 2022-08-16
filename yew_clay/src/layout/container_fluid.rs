use super::container::ClayContainer;
use super::Sizing;
use gloo_events::EventListener;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{html, Children, Classes, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::props::global_props::GlobalProps;
use yew_dom_attributes::props::DomInjector;

/// A Yew implementation of ClayContainer. For more info about ClayContainer, check the documentation:
/// <https://clayui.com/docs/components/layout.html>
pub struct ClayContainerFluid {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, Rc<EventListener>>,
}

/// Props for ClayContainer. For details, check the docs:
/// <https://clayui.com/docs/components/layout/api.html#container>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayContainerFluidProps {
    /// Element or component to render for container
    #[prop_or("div".into())]
    pub container_element: String,

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

    #[prop_or(Some(FluidSize::XLarge))]
    size: Option<FluidSize>,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub html_props: Option<Rc<GlobalProps>>,
}

impl Component for ClayContainerFluid {
    type Message = ();
    type Properties = ClayContainerFluidProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let class = props.class;
        let container_element = props.container_element;
        let form_size = props.form_size;
        let view = props.view;
        let size = props.size;

        html! {
            <ClayContainer
                class={class}
                container_element={container_element}
                form_size={form_size}
                view={view}
                fluid={true}
                fluid_size={size}
                ref={self.node_ref.clone()} >
                {props.children.clone()}
            </ClayContainer>
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

/// Type alias for [Sizing].
type FluidSize = Sizing;

/// Type alias for [Sizing].
type FormSize = Sizing;
