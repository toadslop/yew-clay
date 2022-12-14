use super::LinkContext;
use gloo_events::EventListener;
use std::collections::HashMap;
use web_sys::Element;
use yew::{
    classes, html, Callback, Children, Classes, Component, Context, Html, NodeRef, Properties,
};
use yew_dom_attributes::{anchor_props::AnchorProps, DomInjector};

/// Yew implementation of ClayLink.
/// The type parameter T: LinkContext indicates which link context applies for this link.
/// (The link context allows customizing link tags throughout the app).
/// If you don't need a context, set T to DefaultLinkContext.
///
/// <https://clayui.com/docs/components/link.html>
///
/// A note on the ClayLinkContext. ClayLinkContext allows you to pass arbitrary props to every
/// single ClayLink. However, any props defined directly on the ClayLink take precedence and will
/// override thos set on the ClayLinkContext.
pub struct ClayLink {
    /// This vec holds all the EventListeners defined for this component. They will be automatically
    /// removed when the component is destroyed.
    listeners: HashMap<String, EventListener>,
}

impl ClayLink {
    const BTN: &'static str = "btn";
    const BTN_BLOCK: &'static str = "btn-block";
    const BTN_MONOSPACED: &'static str = "btn-monospaced";
    const BTN_OUTLINE_BORDERLESS: &'static str = "btn-outline-borderless";
    const BTN_SMALL: &'static str = "btn-small";
    const BTN_PRIMARY: &'static str = "btn-primary";
    const BTN_SECONDARY: &'static str = "btn-secondary";
    const BTN_UNSTYLED: &'static str = "btn-unstyled";
    const BTN_OUTLINE_PRIMARY: &'static str = "btn-outline-primary";
    const BTN_OUTLINE_SECONDARY: &'static str = "btn-outline-secondary";
    const BTN_OUTLINE_UNSTYLED: &'static str = "btn-outline-unstyled";
    const LINK_MONOSPACED: &'static str = "link-monospaced";
    const LINK_OUTLINE: &'static str = "link-outline";
    const LINK_BORDERLESS: &'static str = "link-outline-borderless";
    const LINK_PRIMARY: &'static str = "link-primary";
    const LINK_SECONDARY: &'static str = "link-secondary";
    const LINK_UNSTYLED: &'static str = "link-unstyled";
    const LINK_OUTLINE_PRIMARY: &'static str = "link-outline-primary";
    const LINK_OUTLINE_SECONDARY: &'static str = "link-outline-secondary";
    const LINK_OUTLINE_UNSTYLED: &'static str = "link-outline-unstyled";

    fn get_btn_class(button: &LinkButton) -> Option<&'static str> {
        match button {
            LinkButton::Boolean(is_button) => match is_button {
                true => (),
                false => return None,
            },
            LinkButton::Options(_) => (),
        }
        Some(Self::BTN)
    }

    fn get_btn_block_class(button: &LinkButton, block: Option<bool>) -> Option<&'static str> {
        let block = block.unwrap_or(false);
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.block,
        };

        if button || block {
            Some(Self::BTN_BLOCK)
        } else {
            None
        }
    }

    fn get_btn_monospaced_class(button: &LinkButton, monospaced: bool) -> Option<&'static str> {
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.monospaced,
        };

        if button || monospaced {
            Some(Self::BTN_MONOSPACED)
        } else {
            None
        }
    }

    fn get_borderless_class(borderless: bool) -> Option<&'static str> {
        if borderless {
            Some(Self::BTN_OUTLINE_BORDERLESS)
        } else {
            None
        }
    }

    fn get_btn_small_class(button: &LinkButton, small: Option<bool>) -> Option<&'static str> {
        let small = small.unwrap_or(false);
        let button = match button {
            LinkButton::Boolean(_) => false,
            LinkButton::Options(opts) => opts.small,
        };

        if button || small {
            Some(Self::BTN_SMALL)
        } else {
            None
        }
    }

    fn get_btn_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
        borderless: bool,
    ) -> Option<&'static str> {
        if let Some(display_type) = display_type {
            if !outline && !borderless {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::BTN_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::BTN_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::BTN_UNSTYLED),
                };
            }
        }
        None
    }

    fn get_btn_outline_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
        borderless: bool,
    ) -> Option<&'static str> {
        if let Some(display_type) = display_type {
            if outline || borderless {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::BTN_OUTLINE_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::BTN_OUTLINE_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::BTN_OUTLINE_UNSTYLED),
                };
            }
        };
        None
    }

    fn get_bool_class(boolean: bool, class: &str) -> Option<&str> {
        if boolean {
            Some(class)
        } else {
            None
        }
    }

    fn get_link_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
    ) -> Option<&'static str> {
        if !outline {
            if let Some(display_type) = display_type {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::LINK_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::LINK_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::LINK_UNSTYLED),
                };
            }
        }
        None
    }

    fn get_link_outline_display_class(
        display_type: &Option<LinkDisplayType>,
        outline: bool,
    ) -> Option<&'static str> {
        if outline {
            if let Some(display_type) = display_type {
                return match display_type {
                    LinkDisplayType::Primary => Some(Self::LINK_OUTLINE_PRIMARY),
                    LinkDisplayType::Secondary => Some(Self::LINK_OUTLINE_SECONDARY),
                    LinkDisplayType::Unstyled => Some(Self::LINK_OUTLINE_UNSTYLED),
                };
            }
        }
        None
    }
}

/// Props for ClayLink. For details, check the docs:
/// <https://clayui.com/docs/components/link/api.html>
#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct ClayLinkProps {
    /// Renders the button as a block element.
    #[prop_or_default]
    pub block: Option<bool>,

    /// Flag to indicate if link should be borderless.
    #[prop_or(false)]
    pub borderless: bool,

    /// Config for button props
    #[prop_or_default]
    pub button: Option<LinkButton>,

    /// Determines how the link is displayed.
    #[prop_or_default]
    pub display_type: Option<LinkDisplayType>,

    /// Flag to indicate if link should be monospaced.
    #[prop_or(false)]
    pub monospaced: bool,

    /// Flag to indicate if link need have an outline.
    #[prop_or(false)]
    pub outline: bool,

    /// Indicates button should be a small variant.
    #[prop_or_default]
    pub small: Option<bool>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub node_ref: NodeRef,

    /// A catchall prop to pass down anything not specified here to the underlying component.
    #[prop_or_default]
    pub anchor_props: Option<AnchorProps>,
}

pub enum Msg {}

impl Component for ClayLink {
    type Message = Msg;
    type Properties = ClayLinkProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let ClayLinkProps {
            class,
            children,
            node_ref,
            button,
            block,
            monospaced,
            borderless,
            small,
            outline,
            display_type,
            ..
        } = ctx.props().clone();

        let class = if let Some(button) = button {
            let btn_class = Self::get_btn_class(&button);
            let btn_block_class = Self::get_btn_block_class(&button, block);
            let btn_monospaced_class = Self::get_btn_monospaced_class(&button, monospaced);
            let borderless_class = Self::get_borderless_class(borderless);
            let btn_small_class = Self::get_btn_small_class(&button, small);
            let display_class = Self::get_btn_display_class(&display_type, outline, borderless);
            let outline_display_class =
                Self::get_btn_outline_display_class(&display_type, outline, borderless);

            classes!(
                class,
                btn_class,
                btn_block_class,
                btn_monospaced_class,
                borderless_class,
                btn_small_class,
                display_class,
                outline_display_class
            )
        } else {
            let link_monospaced_class = Self::get_bool_class(monospaced, Self::LINK_MONOSPACED);
            let link_outline_class = Self::get_bool_class(outline, Self::LINK_OUTLINE);
            let link_borderless = Self::get_bool_class(borderless, Self::LINK_BORDERLESS);
            let link_display_class = Self::get_link_display_class(&display_type, outline);
            let link_outline_display_class =
                Self::get_link_outline_display_class(&display_type, outline);

            classes!(
                class,
                link_monospaced_class,
                link_outline_class,
                link_borderless,
                link_display_class,
                link_outline_display_class
            )
        };

        let tag = if let Some((context, _)) = ctx.link().context::<LinkContext>(Callback::noop()) {
            context.tag
        } else {
            "a".into()
        };

        html! {
            <@{tag} class={class} ref={node_ref}>{children}</@>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if let Some((context, _)) = ctx.link().context::<LinkContext>(Callback::noop()) {
            let context_props = context.props.clone();
            let node_ref = &ctx.props().node_ref;
            context_props.inject(node_ref, &mut self.listeners);
        }

        if let Some(custom_props) = &ctx.props().anchor_props {
            let custom_props = custom_props.clone();
            let node_ref = &ctx.props().node_ref;
            custom_props.inject(node_ref, &mut self.listeners);
        }

        if let Some(elem) = &ctx.props().node_ref.cast::<Element>() {
            let target = elem.has_attribute("target");
            let rel = elem.has_attribute("rel");
            if target && !rel {
                match elem.set_attribute("rel", "noreferrer noopener") {
                    Ok(_) => (),
                    Err(err) => gloo_console::error!(err),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum LinkButton {
    Boolean(bool),
    Options(LinkButtonOptions),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LinkButtonOptions {
    pub block: bool,
    pub monospaced: bool,
    pub small: bool,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum LinkDisplayType {
    Primary,
    Secondary,
    Unstyled,
}
