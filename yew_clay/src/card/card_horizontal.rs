use super::context::ClayCardContext;
use gloo_events::EventListener;
use std::{collections::HashMap, rc::Rc};
use yew::{
    classes, html, Children, Classes, Component, ContextProvider, Html, NodeRef, Properties,
};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    /// Flag that indicates if `active` class is applied
    active: bool,

    /// Flag that indicates if the card can be selectable.
    selectable: bool,

    #[prop_or_default]
    other_props: Option<Rc<GlobalProps>>,

    #[prop_or_default]
    node_ref: NodeRef,

    #[prop_or_default]
    class: Classes,

    #[prop_or_default]
    children: Children,
}

pub struct ClayCardHorizontalBody {
    listeners: HashMap<String, Rc<EventListener>>,
}

impl Component for ClayCardHorizontalBody {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props {
            class, children, ..
        } = ctx.props().clone();
        html! {
          <div class={classes!(class, ClayCardHorizontal::NOT_SELECTABLE)}>{children}</div>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if let Some(other_props) = &ctx.props().other_props {
            let mut other_props = other_props.clone();
            Rc::make_mut(&mut other_props).inject(&ctx.props().node_ref, &mut self.listeners);
            if let Some(cb) = other_props.get_props_update_callback() {
                cb.emit(other_props.clone());
            }
        }
    }
}

pub struct ClayCardHorizontal {
    listeners: HashMap<String, Rc<EventListener>>,
    context: ClayCardContext,
}

impl ClayCardHorizontal {
    const SELECTABLE: &'static str = "form-check-card form-check form-check-middle-left";
    const CARD_TYPE_DIRECTORY: &'static str = "card-type-directory";
    const ACTIVE: &'static str = "active";
    const NOT_SELECTABLE: &'static str = "card card-horizontal";

    fn get_selectable_class(selectable: bool) -> &'static str {
        if selectable {
            Self::SELECTABLE
        } else {
            Self::NOT_SELECTABLE
        }
    }

    fn get_active_class(active: bool) -> Option<&'static str> {
        if active {
            Some(Self::ACTIVE)
        } else {
            None
        }
    }
}

impl Component for ClayCardHorizontal {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        let context = ClayCardContext {
            horizontal: true,
            interactive: false,
        };
        Self {
            listeners: HashMap::new(),
            context,
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props {
            class,
            selectable,
            children,
            active,
            ..
        } = ctx.props().clone();

        let selectable_class = Self::get_selectable_class(selectable);
        let active_class = Self::get_active_class(active);
        // <ContextProvider<ClayCardContext> context={self.context.clone()}></ContextProvider<ClayCardContext>>
        html! {
         <ContextProvider<ClayCardContext> context={self.context.clone()}>
          <div class={classes!(class, selectable_class, Self::CARD_TYPE_DIRECTORY, active_class)}>
            {children}
          </div>
         </ContextProvider<ClayCardContext>>
        }
    }

    fn rendered(&mut self, ctx: &yew::Context<Self>, _first_render: bool) {
        if let Some(other_props) = &ctx.props().other_props {
            let mut other_props = other_props.clone();
            Rc::make_mut(&mut other_props).inject(&ctx.props().node_ref, &mut self.listeners);
            if let Some(cb) = other_props.get_props_update_callback() {
                cb.emit(other_props.clone());
            }
        }
    }
}
