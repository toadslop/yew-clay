use super::button::ClayButton;
use crate::icon::ClayIcon;
use yew::{html, Component, Context, NodeRef, Properties};
use yew_dom_attributes::props::svg_props::SVGProps;

use super::ClayButtonProps;

/// A Yew implementation of ClayButtonWithIcon. For more info about ClayButton, check the documentation:
/// [https://clayui.com/docs/components/button.html#icon]
pub struct ClayButtonWithIcon {}

/// Props for ClayButton. For details, check the docs:
/// [https://clayui.com/docs/components/button/api.html#buttonwithicon]
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonWithIconProps {
    /// Path to the spritemap that contains your SVG icons. The default Clay SVGs can be found
    /// here [https://github.com/liferay/clay/blob/master/clayui.com/static/images/icons/icons.svg]
    #[prop_or_default]
    spritemap: String,
    /// String identifying the SVG from the spritemap that you want to render.
    #[prop_or_default]
    symbol: String,
    /// Props to be passed down to the ClayButton component that this element wraps.
    #[prop_or_default]
    clay_button_props: ClayButtonProps,
    #[prop_or_default]
    /// Props to be passed down to the underlying SVG of the icon.
    icon_svg_props: Option<SVGProps>,

    /// The NodeRef for the underlying icon. To provide a NodeRef for the underlying button,
    /// use node_ref in clay_button_props.
    #[prop_or_default]
    pub icon_node_ref: NodeRef,
}

impl Component for ClayButtonWithIcon {
    type Message = ();

    type Properties = ButtonWithIconProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let props = ctx.props();
        let button_props = props.clay_button_props.clone();
        let button_ref = button_props.node_ref.clone();
        let monospaced = button_props.monospaced.unwrap_or(true);

        html! {
            <ClayButton
                monospaced={monospaced}
                ref={button_ref}
                ..button_props
            >
                <ClayIcon
                    spritemap={props.spritemap.clone()}
                    symbol={props.symbol.clone()}
                    node_ref={props.icon_node_ref.clone()}
                    svg_props={props.icon_svg_props.clone()}
                     />
            </ClayButton>
        }
    }
}
