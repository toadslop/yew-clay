use gloo_events::EventListener;
use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::{
    attribute_injector::AttributeInjector, listener_injector::ListenerInjector,
    props::svg_props::SVGProps,
};

/// A Yew implementation of ClayIcon.
pub struct ClayIcon {
    node_ref: NodeRef,
    listeners: Vec<EventListener>,
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
    /// This prop allows you to optimize your use of this component.
    /// It defaults to 0, meaning if you don't need any events, it
    /// won't allocate space for them. If you expect to attach 2
    /// listeners, set this prop to 2 and you'll get exactly space for 2
    /// allocated.
    #[prop_or(0)]
    pub event_count: usize,
}

impl Component for ClayIcon {
    type Message = ();
    type Properties = IconProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
            listeners: Vec::with_capacity(ctx.props().event_count),
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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if let Some(mut misc_props) = ctx.props().svg_props.clone() {
            misc_props.inject(&self.node_ref).unwrap();
            if first_render {
                match misc_props.inject_listeners(&self.node_ref) {
                    Ok(mut listeners) => {
                        self.listeners.append(&mut listeners);
                    }
                    Err(_) => todo!(),
                }
            }
        }
    }
}
