use crate::alert::utils::sub_components::AlertIndicator;
use crate::alert::AlertDisplayType;
use crate::alert::AlertVariant;
use crate::alert::ClayAlertFooter;
use crate::button::ClayButtonGroup;
// use crate::icon::ClayIcon;
use crate::layout::ClayContentCol;
use crate::layout::ClayContentSection;
use web_sys::MouseEvent;
use yew::virtual_dom::VChild;
use yew::Callback;
use yew::{html, Html};

pub fn gen_stripe_alert(
    spritemap: Option<&'static str>,
    display_type: &AlertDisplayType,
    variant: &Option<AlertVariant>,
) -> Html {
    let stripe_alert = html! {
        <ClayContentCol>
            <ClayContentSection>
                <AlertIndicator
                    spritemap={spritemap.unwrap_or_default()}
                    display_type={display_type.clone()} />
            </ClayContentSection>
        </ClayContentCol>
    };

    if let Some(variant) = variant {
        match variant {
            AlertVariant::Stripe => stripe_alert,
            _ => html! {},
        }
    } else {
        stripe_alert
    }
}

pub fn gen_default_alert(
    spritemap: Option<&'static str>,
    display_type: &AlertDisplayType,
    variant: &Option<AlertVariant>,
) -> Html {
    if let Some(variant) = variant {
        match variant {
            AlertVariant::Stripe => html! {},
            _ => html! {<AlertIndicator
                spritemap={spritemap.unwrap_or_default()}
                display_type={display_type.clone()} />
            },
        }
    } else {
        html! {}
    }
}

pub fn gen_title_element(title: &Option<String>) -> Html {
    if let Some(title) = title {
        html! {<strong class={"lead"}>{title}</strong>}
    } else {
        html! {}
    }
}

pub fn gen_default_footer_element(
    variant: &Option<AlertVariant>,
    actions: &Option<VChild<ClayButtonGroup>>,
) -> Html {
    if let Some(variant) = &variant {
        if *variant != AlertVariant::Inline && actions.is_some() {
            return html! { <ClayAlertFooter>{actions.clone().unwrap()}</ClayAlertFooter>};
        };
    }
    html! {}
}

pub fn gen_inline_footer_element(
    variant: &Option<AlertVariant>,
    actions: &Option<VChild<ClayButtonGroup>>,
) -> Html {
    if let Some(variant) = &variant {
        if *variant == AlertVariant::Inline && actions.is_some() {
            return html! {
            <ClayContentCol>
                <ClayContentSection>
                    {actions.clone().unwrap()}
                </ClayContentSection>
            </ClayContentCol>};
        };
    }

    html! {}
}

pub fn gen_dismiss_button(
    show_dismissible: bool,
    on_close: Option<Callback<MouseEvent>>,
    spritemap: Option<&'static str>,
) -> Html {
    if show_dismissible {
        html! {
            <button aria-label={"Close"} class={"close"} onclick={on_close} type="button">
                // <ClayIcon spritemap={spritemap.unwrap_or_default()} symbol={"times"} />
            </button>
        }
    } else {
        html! {}
    }
}
