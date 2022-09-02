use std::{collections::HashMap, rc::Rc};

use gloo_events::EventListener;
use yew::{html, Children, Classes, Component, ContextProvider, Html, NodeRef, Properties};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

use super::context::ClayCardContext;

#[derive(Properties, PartialEq, Clone)]
struct Props {
    #[prop_or_default]
    horizontal: bool,

    #[prop_or_default]
    class: Classes,

    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    node_ref: NodeRef,

    #[prop_or_default]
    other_props: Option<Rc<GlobalProps>>,
}

struct ClayCardNavigation {
    listeners: HashMap<String, Rc<EventListener>>,
}

impl Component for ClayCardNavigation {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let Props {
            children,
            horizontal,
            mut other_props,
            ..
        } = ctx.props().clone();

        let context = ClayCardContext {
            horizontal,
            interactive: true,
        };

        html! {
          <ContextProvider<ClayCardContext> context={context}>
            {children}
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
