use std::{collections::HashMap, rc::Rc};
use yew_dom_attributes::props::DomInjector;

use gloo_events::EventListener;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::props::svg_props::SVGProps;

/// A Yew implementation of ClayIcon.
pub struct ClayIcon {
    node_ref: NodeRef,
    /// This vec holds all the EventListeners defined for this button. They will be automatically
    /// removed when the button is destroyed.
    listeners: HashMap<String, EventListener>,
}

/// Props for ClayIcon. For details, check the docs:
/// https://clayui.com/docs/components/icon/api.html
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct IconProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub spritemap: String,
    #[prop_or_default]
    pub symbol: String,
    #[prop_or_default]
    pub svg_props: Option<SVGProps>,
    #[prop_or_default]
    pub node_ref: NodeRef,
    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub svg_html_attributes: Option<Rc<SVGProps>>,
}

impl Component for ClayIcon {
    type Message = ();
    type Properties = IconProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props().clone();
        let class = vec![
            props.class,
            "lexicon-icon".into(),
            format!("lexicon-icon-{}", props.symbol),
        ]
        .join(" ");

        let xlink_href = format!("{}#{}", props.spritemap, props.symbol);

        html! {
          <svg
            class={class}
            key={props.symbol}
            ref={self.node_ref.clone()}
            role="presentation"
          >
            <use xlinkHref={xlink_href} />
          </svg>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some(svg_props) = &ctx.props().svg_html_attributes {
            let mut svg_props = svg_props.clone();
            Rc::make_mut(&mut svg_props).inject(&self.node_ref, &mut self.listeners);
            svg_props
                .get_props_update_callback()
                .emit(svg_props.clone());
        }
    }
}
