use super::enums::{AlertDisplayType, AlertVariant};
// use crate::icon::ClayIcon;
use yew::{function_component, html, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct CondCompProps {
    pub variant: Option<AlertVariant>,
    pub children: Children,
}

#[function_component(ConditionalContainer)]
pub fn contiditional_container(props: &CondCompProps) -> Html {
    if let Some(variant) = props.variant.clone() {
        match variant {
            AlertVariant::Stripe => {
                return html! {<div class={"container"}>{for props.children.iter()}</div> }
            }
            _ => (),
        }
    }

    html! {<>{for props.children.iter()}</>}
}

#[derive(Properties, PartialEq)]
pub struct AlertIndicatorProps {
    pub spritemap: &'static str,
    pub display_type: AlertDisplayType,
}

#[function_component(AlertIndicator)]
pub fn alert_indicator(props: &AlertIndicatorProps) -> Html {
    let spritemap = props.spritemap;
    let symbol = match &props.display_type {
        AlertDisplayType::Danger => "exclamation-full",
        AlertDisplayType::Info => "info-circle",
        AlertDisplayType::Success => "check-circle-full",
        AlertDisplayType::Warning => "warning-full",
    };
    html! {
        <span class={"alert-indicator"}>
            // <ClayIcon {spritemap} {symbol} />
        </span>
    }
}
