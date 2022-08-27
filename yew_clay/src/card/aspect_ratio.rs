use super::Interactive;
use strum::AsRefStr;
use yew::{classes, html, Children, Classes, Component, Html, Properties};

#[derive(Debug, PartialEq, AsRefStr, Clone)]
pub enum ContainerAspectRatioType {
    #[strum(serialize = "1-to-1")]
    OneToOne,
    #[strum(serialize = "3-to-2")]
    ThreeToTwo,
    #[strum(serialize = "4-to-3")]
    FourToThree,
    #[strum(serialize = "8-to-5")]
    EightToFive,
    #[strum(serialize = "16-to-9")]
    SixteenToNine,
}

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct Props {
    /// AspectRatio content.
    pub children: Children,

    /// Defines a CSS class for the element.
    #[prop_or_default]
    pub class: Classes,

    /// Contrains an image for a given Aspect Ratio.
    #[prop_or_default]
    pub container_aspect_ratio: Option<ContainerAspectRatioType>,
}

#[derive(Debug)]
pub struct ClayCardAspectRatio;

impl ClayCardAspectRatio {
    const ASPECT_RATIO: &'static str = "aspect-ratio-";

    fn get_apect_ratio_class(
        container_aspect_ratio: &Option<ContainerAspectRatioType>,
    ) -> Option<String> {
        if let Some(container_aspect_ratio) = container_aspect_ratio {
            let ratio = container_aspect_ratio.as_ref();

            let mut aspect_ratio_class =
                String::with_capacity(ratio.len() + Self::ASPECT_RATIO.len());
            aspect_ratio_class.push_str(Self::ASPECT_RATIO);
            aspect_ratio_class.push_str(ratio);
            Some(aspect_ratio_class)
        } else {
            None
        }
    }
}

impl Component for ClayCardAspectRatio {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let interactive = Self::get_interactive(ctx);
        let tag = Self::get_tag_name(interactive);
        let Props {
            children,
            class,
            container_aspect_ratio,
        } = ctx.props().clone();
        let aspect_ratio_class = Self::get_apect_ratio_class(&container_aspect_ratio);

        html! {
            <@{tag} class={classes!(class, aspect_ratio_class)}>
                {children.clone()}
            </@>
        }
    }
}

impl Interactive for ClayCardAspectRatio {}
