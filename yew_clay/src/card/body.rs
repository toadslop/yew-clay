use std::{collections::HashMap, rc::Rc};

use gloo_events::EventListener;
use yew::{classes, html, Children, Classes, Component, Html, NodeRef, Properties};
use yew_dom_attributes::{global_props::GlobalProps, DomInjector};

use super::Interactive;

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub other_props: Option<Rc<GlobalProps>>,

    #[prop_or_default]
    pub node_ref: NodeRef,
}

pub struct ClayCardBody {
    listeners: HashMap<String, Rc<EventListener>>,
}

impl ClayCardBody {
    const CARD_BODY: &'static str = "card-body";
}

impl Component for ClayCardBody {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let interactive = Self::get_interactive(ctx);
        let tag = Self::get_tag_name(interactive);
        let Props { class, .. } = ctx.props().clone();

        html! {
          <@{tag} class={classes!(class, Self::CARD_BODY)}>

          </@>
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

impl Interactive for ClayCardBody {}
