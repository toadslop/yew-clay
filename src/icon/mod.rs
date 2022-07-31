use yew::{html, Component, Context, Html, NodeRef, Properties};
use yew_dom_attributes::attribute_collection::AttributeCollection;
use yew_dom_attributes::misc_attributes::MiscAttrs;

/// A Yew implementation of ClayIcon.
pub struct ClayIcon {
    node_ref: NodeRef,
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
    /// Arbitrary props that will be passed down to the underlying component.
    #[prop_or_default]
    pub misc_attrs: MiscAttrs,
    #[prop_or_default]
    pub node_ref: NodeRef,
}

impl Component for ClayIcon {
    type Message = ();
    type Properties = IconProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            node_ref: ctx.props().node_ref.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            () => {}
        }
        false
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
        ctx.props().misc_attrs.inject(&self.node_ref);
    }
}
